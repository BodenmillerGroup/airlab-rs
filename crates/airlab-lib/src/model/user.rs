use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::base::{self, DbBmc};
use crate::pwd::{self, ContentToHash};
use modql::field::{Fields, HasFields};
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query, SimpleExpr};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::postgres::PgRow;
use uuid::Uuid;

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize, Default)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl UserBmc {
    #[must_use]
    pub fn get_create_sql(drop_table: bool) -> String {
        let table = Self::TABLE;
        format!(
            r##"{}
create table if not exists "{table}" (
  id integer GENERATED BY DEFAULT AS IDENTITY (START WITH 1000) PRIMARY KEY,

  username varchar(128) NOT NULL UNIQUE,

  email character varying,
  name character varying,
  password character varying,
  is_active boolean DEFAULT false NOT NULL,
  is_admin boolean DEFAULT false NOT NULL,
  meta jsonb,
  created_at timestamp with time zone DEFAULT now() NOT NULL,
  updated_at timestamp with time zone DEFAULT now() NOT NULL,

  -- Auth
  pwd varchar(256),
  reset_token varchar(256),
  pwd_salt uuid NOT NULL DEFAULT gen_random_uuid(),
  token_salt uuid NOT NULL DEFAULT gen_random_uuid()
);
ALTER TABLE ONLY "user"
  ADD CONSTRAINT "UQ_user_email" UNIQUE (email);
CREATE INDEX "IDX_user_email" ON "user" USING btree (email);
CREATE INDEX "IDX_user_is_active" ON "user" USING btree (is_active);
        "##,
            if drop_table {
                format!("drop table if exists {table};")
            } else {
                String::new()
            }
        )
    }
}
#[derive(Fields, Default, Deserialize, Debug)]
pub struct UserForCreate {
    pub username: Option<String>,
    pub pwd_clear: Option<String>,
    pub email: String,
    pub name: Option<String>,
}

#[derive(Fields, Default, Serialize, Deserialize, Debug, Clone)]
pub struct MinUser {
    pub email: String,
    pub id: i32,
    pub name: String,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct UserForUpdate {
    pub email: Option<String>,
    pub name: Option<String>,
    pub reset_token: Option<String>,
    pub is_admin: Option<bool>,
    pub is_active: Option<bool>,
}

#[derive(Fields)]
pub struct UserForInsert {
    pub username: String,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForLogin {
    pub id: i32,
    pub username: String,

    pub pwd: Option<String>,
    pub pwd_salt: Uuid,
    pub token_salt: Uuid,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForAuth {
    pub id: i32,
    pub username: String,

    // -- token info
    pub token_salt: Uuid,
}

pub trait UserBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl UserBy for User {}
impl UserBy for UserForLogin {}
impl UserBy for UserForAuth {}

#[derive(Iden)]
enum UserIden {
    Id,
    Username,
    ResetToken,
    Pwd,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct UserFilter {
    id: Option<OpValsInt64>,

    name: Option<OpValsString>,
}

pub struct UserBmc;

impl DbBmc for UserBmc {
    const TABLE: &'static str = "user";
}

impl UserBmc {
    pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i32) -> Result<E>
    where
        E: UserBy,
    {
        base::get::<Self, _>(ctx, mm, id).await
    }

    pub async fn list(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<UserFilter>>,
        list_options: Option<ListOptions>,
    ) -> Result<Vec<User>> {
        base::list::<Self, _, _>(ctx, mm, filters, list_options).await
    }

    pub async fn first_by_username<E>(
        _ctx: &Ctx,
        mm: &ModelManager,
        username: &str,
    ) -> Result<Option<E>>
    where
        E: UserBy,
    {
        let db = mm.db();

        let mut query = Query::select();
        query
            .from(Self::table_ref())
            .columns(E::field_idens())
            .and_where(Expr::col(UserIden::Username).eq(username));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let user = sqlx::query_as_with::<_, E, _>(&sql, values)
            .fetch_optional(db)
            .await?;

        Ok(user)
    }

    pub async fn first_by_token<E>(_ctx: &Ctx, mm: &ModelManager, token: &str) -> Result<Option<E>>
    where
        E: UserBy,
    {
        let db = mm.db();

        let mut query = Query::select();
        query
            .from(Self::table_ref())
            .columns(E::field_idens())
            .and_where(Expr::col(UserIden::ResetToken).eq(token));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let user = sqlx::query_as_with::<_, E, _>(&sql, values)
            .fetch_optional(db)
            .await?;

        Ok(user)
    }

    pub async fn create(ctx: &Ctx, mm: &ModelManager, user_c: UserForCreate) -> Result<i32> {
        base::create::<Self, _>(ctx, mm, user_c).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i32,
        group_u: UserForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, group_u).await
    }

    pub async fn update_pwd(ctx: &Ctx, mm: &ModelManager, id: i32, pwd_clear: &str) -> Result<()> {
        let db = mm.db();

        let user: UserForLogin = Self::get(ctx, mm, id).await?;
        let pwd = pwd::hash_pwd(&ContentToHash {
            content: pwd_clear.to_string(),
            salt: user.pwd_salt,
        })?;

        let mut query = Query::update();
        query
            .table(Self::table_ref())
            .value(UserIden::Pwd, SimpleExpr::from(pwd))
            .and_where(Expr::col(UserIden::Id).eq(id));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let _count = sqlx::query_with(&sql, values)
            .execute(db)
            .await?
            .rows_affected();

        Ok(())
    }
}
