use crate::model::ModelManager;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;
use std::time::Duration;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};
use tracing::error;

const TEST_DB_PREFIX: &str = "airlab_test";
const DOCKER_IMAGE_DEFAULT: &str = "postgres:15";
const DOCKER_POSTGRES_USER: &str = "postgres";
const DOCKER_POSTGRES_PASSWORD: &str = "postgres";
const DOCKER_POSTGRES_DB: &str = "postgres";

type BoxError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, BoxError>;

pub fn init_test_env() {
    static INIT: std::sync::Once = std::sync::Once::new();

    INIT.call_once(|| {
        // SAFETY: tests initialize env once before any config is read.
        unsafe {
            std::env::set_var("SERVICE_PWD_KEY", "MDEyMzQ1Njc4OWFiY2RlZg");
            std::env::set_var("SERVICE_TOKEN_KEY", "ZmVkY2JhOTg3NjU0MzIxMA");
            std::env::set_var("SERVICE_TOKEN_DURATION_SEC", "3600");
        }
    });
}

pub async fn new_test_db() -> TestDb {
    init_test_env();

    let permit = concurrency_semaphore()
        .acquire_owned()
        .await
        .unwrap_or_else(|err| {
            error!("test db semaphore should not be closed: {err}");
            std::process::abort();
        });
    let harness = shared_harness().await;
    harness.reset_working_database().unwrap_or_else(|err| {
        error!("failed to reset working test database: {err:#}");
        std::process::abort();
    });

    let model_manager = ModelManager::from_db_url(&harness.db_url())
        .await
        .unwrap_or_else(|err| {
            error!("failed to connect to test database: {err:#}");
            std::process::abort();
        });

    TestDb {
        model_manager: Some(model_manager),
        _permit: Some(permit),
    }
}

pub struct TestDb {
    model_manager: Option<ModelManager>,
    _permit: Option<OwnedSemaphorePermit>,
}

impl Deref for TestDb {
    type Target = ModelManager;

    fn deref(&self) -> &Self::Target {
        match self.model_manager.as_ref() {
            Some(model_manager) => model_manager,
            None => {
                error!("test model manager already dropped");
                std::process::abort();
            }
        }
    }
}

impl Drop for TestDb {
    fn drop(&mut self) {
        let _ = self.model_manager.take();
        let _ = self._permit.take();
    }
}

struct Harness {
    docker_path: Option<PathBuf>,
    container_id: Option<String>,
    admin_db_url: String,
    working_db_name: String,
    psql_path: PathBuf,
}

impl Harness {
    fn db_url(&self) -> String {
        replace_db_name(&self.admin_db_url, &self.working_db_name).unwrap_or_else(|err| {
            error!("admin database url should be valid: {err:#}");
            std::process::abort();
        })
    }

    fn reset_working_database(&self) -> Result<()> {
        let db_url = self.db_url();
        let _ =
            drop_database_with_retry(&self.psql_path, &self.admin_db_url, &self.working_db_name);
        create_database(&self.psql_path, &self.admin_db_url, &self.working_db_name)?;
        run_psql(
            &self.psql_path,
            &db_url,
            "CREATE EXTENSION IF NOT EXISTS pgcrypto;",
        )?;

        for migration in migration_files()? {
            run_psql_file(&self.psql_path, &db_url, &migration)?;
        }

        run_psql(&self.psql_path, &db_url, seed_sql())?;
        Ok(())
    }
}

impl Drop for Harness {
    fn drop(&mut self) {
        let _ =
            drop_database_with_retry(&self.psql_path, &self.admin_db_url, &self.working_db_name);
        if let (Some(docker_path), Some(container_id)) = (&self.docker_path, &self.container_id) {
            let _ = docker_rm_force(docker_path, container_id);
        }
    }
}

async fn shared_harness() -> &'static Harness {
    static HARNESS: tokio::sync::OnceCell<Harness> = tokio::sync::OnceCell::const_new();

    HARNESS
        .get_or_init(|| async {
            build_harness().unwrap_or_else(|err| {
                error!("failed to initialize postgres test harness: {err:#}");
                std::process::abort();
            })
        })
        .await
}

fn concurrency_semaphore() -> std::sync::Arc<Semaphore> {
    static SEMAPHORE: OnceLock<std::sync::Arc<Semaphore>> = OnceLock::new();

    SEMAPHORE
        .get_or_init(|| {
            let limit = std::env::var("AIRLAB_TEST_DB_CONCURRENCY")
                .ok()
                .and_then(|value| value.parse::<usize>().ok())
                .filter(|value| *value > 0)
                .unwrap_or(1);
            std::sync::Arc::new(Semaphore::new(limit))
        })
        .clone()
}

fn build_harness() -> Result<Harness> {
    let psql_path = psql_path()?;
    let (docker_path, container_id, admin_db_url) =
        if let Ok(admin_db_url) = std::env::var("AIRLAB_TEST_DB_ADMIN_URL") {
            (None, None, admin_db_url)
        } else {
            let docker_path = docker_path()?;
            let container_id = start_postgres_container(&docker_path)?;
            let admin_db_url = admin_db_url(&docker_path, &container_id)?;
            (Some(docker_path), Some(container_id), admin_db_url)
        };
    let working_db_name = format!("{TEST_DB_PREFIX}_{}", std::process::id());

    wait_until_ready(&psql_path, &admin_db_url)?;

    Ok(Harness {
        docker_path,
        container_id,
        admin_db_url,
        working_db_name,
        psql_path,
    })
}

fn docker_path() -> Result<PathBuf> {
    let output = Command::new("which").arg("docker").output()?;
    if output.status.success() {
        let path = String::from_utf8(output.stdout)?.trim().to_string();
        return Ok(PathBuf::from(path));
    }

    Err(std::io::Error::other("could not find docker").into())
}

fn psql_path() -> Result<PathBuf> {
    let output = Command::new("pg_config").arg("--bindir").output()?;
    if output.status.success() {
        let bindir = String::from_utf8(output.stdout)?.trim().to_string();
        return Ok(PathBuf::from(bindir).join("psql"));
    }

    let output = Command::new("which").arg("psql").output()?;
    if output.status.success() {
        let path = String::from_utf8(output.stdout)?.trim().to_string();
        return Ok(PathBuf::from(path));
    }

    Err(std::io::Error::other("could not find psql via pg_config or which").into())
}

fn admin_db_url(docker_path: &Path, container_id: &str) -> Result<String> {
    let host_port = docker_mapped_port(docker_path, container_id, "5432/tcp")?;
    Ok(format!(
        "postgresql://{}:{}@127.0.0.1:{}/{}",
        DOCKER_POSTGRES_USER, DOCKER_POSTGRES_PASSWORD, host_port, DOCKER_POSTGRES_DB
    ))
}

fn replace_db_name(url: &str, db_name: &str) -> Result<String> {
    let (base, query) = match url.split_once('?') {
        Some((base, query)) => (base, Some(query)),
        None => (url, None),
    };
    let (head, _old_db) = base.rsplit_once('/').ok_or_else(|| {
        std::io::Error::other(format!("database url is missing database name: {url}"))
    })?;

    let replaced = match query {
        Some(query) => format!("{head}/{db_name}?{query}"),
        None => format!("{head}/{db_name}"),
    };

    Ok(replaced)
}

fn run_psql(psql_path: &Path, db_url: &str, sql: &str) -> Result<()> {
    let output = Command::new(psql_path)
        .env("PGCONNECT_TIMEOUT", "5")
        .args(["-w", "-v", "ON_ERROR_STOP=1", db_url, "-c", sql])
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other(format!(
            "psql failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ))
        .into())
    }
}

fn run_psql_file(psql_path: &Path, db_url: &str, file: &Path) -> Result<()> {
    let output = Command::new(psql_path)
        .env("PGCONNECT_TIMEOUT", "5")
        .args(["-w", "-v", "ON_ERROR_STOP=1", db_url, "-f"])
        .arg(file)
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other(format!(
            "psql failed for {}: {}",
            file.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
        .into())
    }
}

fn wait_until_ready(psql_path: &Path, admin_db_url: &str) -> Result<()> {
    let mut last_err = None;

    for _ in 0..100 {
        match run_psql(psql_path, admin_db_url, "SELECT 1;") {
            Ok(()) => return Ok(()),
            Err(err) => {
                last_err = Some(err);
                std::thread::sleep(Duration::from_millis(200));
            }
        }
    }

    Err(last_err
        .unwrap_or_else(|| std::io::Error::other("wait_until_ready exhausted retries").into()))
}

fn create_database(psql_path: &Path, admin_db_url: &str, db_name: &str) -> Result<()> {
    run_psql(
        psql_path,
        admin_db_url,
        &format!("CREATE DATABASE \"{db_name}\";"),
    )
}

fn drop_database(psql_path: &Path, admin_db_url: &str, db_name: &str) -> Result<()> {
    run_psql(
        psql_path,
        admin_db_url,
        &format!("DROP DATABASE IF EXISTS \"{db_name}\" WITH (FORCE);"),
    )
}

fn drop_database_with_retry(psql_path: &Path, admin_db_url: &str, db_name: &str) -> Result<()> {
    let mut last_err = None;

    for _ in 0..50 {
        match drop_database(psql_path, admin_db_url, db_name) {
            Ok(()) => return Ok(()),
            Err(err) => {
                last_err = Some(err);
                std::thread::sleep(Duration::from_millis(20));
            }
        }
    }

    Err(last_err.unwrap_or_else(|| {
        std::io::Error::other("drop_database_with_retry exhausted retries").into()
    }))
}

fn migration_files() -> Result<Vec<PathBuf>> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let parent_dir = manifest_dir.parent().ok_or_else(|| {
        std::io::Error::other(format!(
            "crate dir has no parent: {}",
            manifest_dir.display()
        ))
    })?;
    let migrations_dir = parent_dir.join("airlab-web").join("migrations");
    let mut files = std::fs::read_dir(migrations_dir)?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "sql"))
        .collect::<Vec<_>>();
    files.sort();
    Ok(files)
}

fn start_postgres_container(docker_path: &Path) -> Result<String> {
    let image =
        std::env::var("AIRLAB_TEST_DB_IMAGE").unwrap_or_else(|_| DOCKER_IMAGE_DEFAULT.to_string());
    let container_name = format!("airlab-test-db-{}", std::process::id());

    let output = Command::new(docker_path)
        .args([
            "run",
            "-d",
            "--rm",
            "--name",
            &container_name,
            "-e",
            &format!("POSTGRES_USER={DOCKER_POSTGRES_USER}"),
            "-e",
            &format!("POSTGRES_PASSWORD={DOCKER_POSTGRES_PASSWORD}"),
            "-e",
            &format!("POSTGRES_DB={DOCKER_POSTGRES_DB}"),
            "-p",
            "127.0.0.1::5432",
            &image,
        ])
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    } else {
        Err(std::io::Error::other(format!(
            "docker run failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ))
        .into())
    }
}

fn docker_mapped_port(docker_path: &Path, container_id: &str, port: &str) -> Result<u16> {
    let output = Command::new(docker_path)
        .args(["port", container_id, port])
        .output()?;

    if !output.status.success() {
        return Err(std::io::Error::other(format!(
            "docker port failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ))
        .into());
    }

    let mapping = String::from_utf8(output.stdout)?.trim().to_string();
    let port = mapping
        .rsplit_once(':')
        .ok_or_else(|| std::io::Error::other(format!("unexpected docker port mapping: {mapping}")))?
        .1
        .parse::<u16>()?;

    Ok(port)
}

fn docker_rm_force(docker_path: &Path, container_id: &str) -> Result<()> {
    let output = Command::new(docker_path)
        .args(["rm", "-f", container_id])
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other(format!(
            "docker rm -f failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ))
        .into())
    }
}

fn seed_sql() -> &'static str {
    r#"
INSERT INTO public."group" (id, name, institution, url, is_open, description, location, tags, cid, ctime, mid, mtime, created_at)
VALUES
    (1, 'primary test group', 'Airlab', 'https://example.test/primary', false, 'Primary test group', 'Zurich', ARRAY['test'], 1, NOW(), 1, NOW(), NOW()),
    (1000, 'seed group', 'Airlab', 'https://example.test/seed', false, 'Seed group', 'Zurich', ARRAY['seed'], 1, NOW(), 1, NOW(), NOW())
ON CONFLICT (id) DO NOTHING;

INSERT INTO public."user" (id, username, email, name, is_active, is_admin, pwd, pwd_salt, token_salt, mfa_enabled, mfa_secret, cid, ctime, mid, mtime, created_at, updated_at)
VALUES
    (1, 'demo1@uzh.ch', 'demo1@uzh.ch', 'Demo One', true, false, NULL, '11111111-1111-1111-1111-111111111111', 'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa', false, '', 1, NOW(), 1, NOW(), NOW(), NOW()),
    (261, 'member261@example.test', 'member261@example.test', 'Member 261', true, false, NULL, '22222222-2222-2222-2222-222222222222', 'bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb', false, '', 1, NOW(), 1, NOW(), NOW(), NOW()),
    (1000, 'member1000@example.test', 'member1000@example.test', 'Member 1000', true, false, NULL, '33333333-3333-3333-3333-333333333333', 'cccccccc-cccc-cccc-cccc-cccccccccccc', false, '', 1, NOW(), 1, NOW(), NOW(), NOW()),
    (1001, 'seed-owner@example.test', 'seed-owner@example.test', 'Seed Owner', true, false, NULL, '44444444-4444-4444-4444-444444444444', 'dddddddd-dddd-dddd-dddd-dddddddddddd', false, '', 1, NOW(), 1, NOW(), NOW(), NOW()),
    (1002, 'seed-analyst@example.test', 'seed-analyst@example.test', 'Seed Analyst', true, false, NULL, '55555555-5555-5555-5555-555555555555', 'eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee', false, '', 1, NOW(), 1, NOW(), NOW(), NOW())
ON CONFLICT (id) DO NOTHING;

INSERT INTO public.member (id, group_id, user_id, role, all_panels, is_active, cid, ctime, mid, mtime, created_at, updated_at)
VALUES
    (1, 1, 1, 0, false, true, 1, NOW(), 1, NOW(), NOW(), NOW()),
    (261, 1, 261, 0, false, true, 1, NOW(), 1, NOW(), NOW(), NOW()),
    (1000, 1000, 1000, 0, false, true, 1, NOW(), 1, NOW(), NOW(), NOW()),
    (1303, 1000, 1001, 0, false, true, 1, NOW(), 1, NOW(), NOW(), NOW()),
    (1304, 1000, 1002, 1, true, true, 1, NOW(), 1, NOW(), NOW(), NOW())
ON CONFLICT (id) DO NOTHING;

INSERT INTO public.protein (id, group_id, created_by, name, description, cid, ctime, mid, mtime, created_at)
VALUES
    (1002, 1000, 1303, 'seed-protein', 'seed protein', 1, NOW(), 1, NOW(), NOW()),
    (1012, 1000, 1304, 'aux-protein', 'aux protein', 1, NOW(), 1, NOW(), NOW())
ON CONFLICT (id) DO NOTHING;

INSERT INTO public.provider (id, group_id, name, description, url, cid, ctime, mid, mtime, created_at)
VALUES
    (103, 1, 'primary-provider', 'provider for primary group', NULL, 1, NOW(), 1, NOW(), NOW()),
    (1003, 1000, 'seed-provider', 'seed provider', NULL, 1, NOW(), 1, NOW(), NOW()),
    (1013, 1000, 'backup-provider', 'backup provider', NULL, 1, NOW(), 1, NOW(), NOW())
ON CONFLICT (id) DO NOTHING;

INSERT INTO public.species (id, group_id, name, acronym, cid, ctime, mid, mtime, created_at)
VALUES
    (44, 1, 'Human', 'HU', 1, NOW(), 1, NOW(), NOW()),
    (1004, 1000, 'Mouse', 'MO', 1, NOW(), 1, NOW(), NOW()),
    (1014, 1000, 'Rat', 'RA', 1, NOW(), 1, NOW(), NOW())
ON CONFLICT (id) DO NOTHING;

INSERT INTO public.tag (id, group_id, name, description, is_metal, is_fluorophore, is_enzyme, is_biotin, is_other, status, cid, ctime, mid, mtime, created_at)
VALUES
    (211, 1, 'primary-tag', 'primary tag', true, false, false, false, false, 0, 1, NOW(), 1, NOW(), NOW()),
    (1005, 1000, 'seed-tag', 'seed tag', true, false, false, false, false, 0, 1, NOW(), 1, NOW(), NOW()),
    (1015, 1000, 'backup-tag', 'backup tag', false, true, false, false, false, 1, 1, NOW(), 1, NOW(), NOW())
ON CONFLICT (id) DO NOTHING;

INSERT INTO public.clone (id, group_id, created_by, protein_id, species_id, name, isotype, epitope, is_phospho, is_polyclonal, reactivity, is_archived, cid, ctime, mid, mtime, created_at, updated_at)
VALUES
    (1006, 1000, 1303, 1002, 1004, 'seed-clone', '', '', false, false, ARRAY[7001,7002], false, 1, NOW(), 1, NOW(), NOW(), NOW()),
    (1016, 1000, 1304, 1012, 1014, 'backup-clone', '', '', false, false, ARRAY[7100,7200], false, 1, NOW(), 1, NOW(), NOW(), NOW()),
    (3123, 1, 261, 1002, 44, 'primary-clone-1', '', '', false, false, NULL, false, 1, NOW(), 1, NOW(), NOW(), NOW()),
    (3124, 1, 261, 1002, 44, 'primary-clone-2', '', '', false, false, NULL, false, 1, NOW(), 1, NOW(), NOW(), NOW())
ON CONFLICT (id) DO NOTHING;

INSERT INTO public.collection (id, name, description, created_at, created_by)
VALUES
    (1101, 'seed-collection-a', 'seed collection a', NOW(), 1303),
    (1102, 'seed-collection-b', 'seed collection b', NOW(), 1304)
ON CONFLICT (id) DO NOTHING;

INSERT INTO public.storage (id, name, "type", location, temperature_c, active, created_at, updated_at)
VALUES
    (1201, 'seed-storage-a', 'fridge', 'Room A', 4, true, NOW(), NOW()),
    (1202, 'seed-storage-b', 'freezer', 'Room B', -80, false, NOW(), NOW())
ON CONFLICT (id) DO NOTHING;

INSERT INTO public.lot (id, group_id, created_by, clone_id, provider_id, collection_id, name, status, is_archived, cid, ctime, mid, mtime, created_at, updated_at)
VALUES
    (1007, 1000, 1303, 1006, 1003, 1101, 'seed-lot', 0, false, 1, NOW(), 1, NOW(), NOW(), NOW()),
    (1018, 1000, 1304, 1016, 1013, 1102, 'backup-lot', 1, false, 1, NOW(), 1, NOW(), NOW(), NOW()),
    (5495, 1, 261, 3123, 103, NULL, 'primary-lot', 0, false, 1, NOW(), 1, NOW(), NOW(), NOW())
ON CONFLICT (id) DO NOTHING;

INSERT INTO public.conjugate (id, group_id, created_by, lot_id, tag_id, storage_id, status, tube_number, description, is_archived, cid, ctime, mid, mtime, created_at, updated_at)
VALUES
    (1008, 1000, 1303, 1007, 1005, 1201, 0, 1, 'seed-conjugate', false, 1, NOW(), 1, NOW(), NOW(), NOW()),
    (1019, 1000, 1304, 1018, 1015, 1202, 1, 4, 'backup-conjugate', false, 1, NOW(), 1, NOW(), NOW(), NOW()),
    (4291, 1, 261, 5495, 211, NULL, 0, 2, 'primary-conjugate-1', false, 1, NOW(), 1, NOW(), NOW(), NOW()),
    (4292, 1, 261, 5495, 211, NULL, 0, 3, 'primary-conjugate-2', false, 1, NOW(), 1, NOW(), NOW(), NOW())
ON CONFLICT (id) DO NOTHING;

INSERT INTO public.panel (id, group_id, created_by, name, is_fluorophore, is_locked, is_archived, cid, ctime, mid, mtime, created_at, updated_at)
VALUES
    (1009, 1000, 1303, 'seed-panel', false, false, false, 1, NOW(), 1, NOW(), NOW(), NOW()),
    (1020, 1000, 1304, 'backup-panel', true, true, false, 1, NOW(), 1, NOW(), NOW(), NOW()),
    (1815, 1, 261, 'primary-panel', false, false, false, 1, NOW(), 1, NOW(), NOW(), NOW())
ON CONFLICT (id) DO NOTHING;

INSERT INTO public.panel_element (id, panel_id, conjugate_id, dilution_type, concentration)
VALUES
    (1021, 1020, 1008, 1, 0.1),
    (1022, 1009, 1019, 2, 0.2),
    (1023, 1020, 1019, 3, 0.3)
ON CONFLICT (id) DO NOTHING;

INSERT INTO public.validation (id, group_id, created_by, clone_id, lot_id, conjugate_id, species_id, application, tissue, status, file_id, is_archived, cid, ctime, mid, mtime, created_at, updated_at)
VALUES
    (1011, 1000, 1303, 1006, 1007, 1008, 1004, 1, 'seed-tissue', 1, 1, false, 1, NOW(), 1, NOW(), NOW(), NOW()),
    (1024, 1000, 1304, 1016, 1018, 1019, 1014, 2, 'backup-tissue', 2, 1, false, 1, NOW(), 1, NOW(), NOW(), NOW()),
    (2221, 1, 261, 3124, 5495, 4291, 44, 1, 'primary-tissue', 1, 1, false, 1, NOW(), 1, NOW(), NOW(), NOW())
ON CONFLICT (id) DO NOTHING;

SELECT setval('public.group_id_seq', 2000, true);
SELECT setval('public.user_id_seq', 2000, true);
SELECT setval('public.member_id_seq', 2000, true);
SELECT setval('public.protein_id_seq', 2000, true);
SELECT setval('public.provider_id_seq', 2000, true);
SELECT setval('public.species_id_seq', 2000, true);
SELECT setval('public.tag_id_seq', 2000, true);
SELECT setval('public.clone_id_seq', 4000, true);
SELECT setval('public.lot_id_seq', 6000, true);
SELECT setval('public.conjugate_id_seq', 5000, true);
SELECT setval('public.panel_id_seq', 2000, true);
SELECT setval('public.panel_element_id_seq', 2000, true);
SELECT setval('public.validation_id_seq', 3000, true);
SELECT setval('public.storage_id_seq', 2000, true);
SELECT setval('public.collection_id_seq', 2000, true);
"#
}
