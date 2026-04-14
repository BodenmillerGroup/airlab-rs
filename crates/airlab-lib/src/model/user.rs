use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use crate::model::base::{self, DbBmc};
use crate::model::helpers::{opt_bool, opt_string, string_or};
use crate::pwd::{self, ContentToHash};
use modql::field::HasSeaFields;
use modql::field::{Fields, HasFields};
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use sea_query::Asterisk;
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query, SimpleExpr};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use sqlx::postgres::PgRow;
#[allow(unused_imports)]
use tracing::{debug, warn};
use uuid::Uuid;

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize, Default)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "mfaEnabled")]
    pub mfa_enabled: bool,
    #[serde(rename = "mfaSecret")]
    pub mfa_secret: String,
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct UserForCreate {
    pub username: Option<String>,
    pub pwd_clear: Option<String>,
    pub email: String,
    pub name: Option<String>,
}

impl From<Value> for UserForCreate {
    fn from(v: Value) -> Self {
        let obj = match v {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(Default::default()),
        };

        UserForCreate {
            username: opt_string(&obj, "username"),
            pwd_clear: opt_string(&obj, "pwd_clear"),
            email: string_or(&obj, "email"),
            name: opt_string(&obj, "name"),
        }
    }
}

#[derive(Fields, Default, Serialize, Deserialize, Debug, Clone)]
pub struct MinUser {
    pub email: String,
    pub id: i64,
    pub name: String,
}

#[derive(Fields, Default, Deserialize, Debug)]
pub struct UserForUpdate {
    pub email: Option<String>,
    pub name: Option<String>,
    pub reset_token: Option<String>,
    pub mfa_secret: Option<String>,
    pub is_admin: Option<bool>,
    pub is_active: Option<bool>,
    pub mfa_enabled: Option<bool>,
}

impl From<Value> for UserForUpdate {
    fn from(v: Value) -> Self {
        let obj = match v {
            Value::Object(map) => Value::Object(map),
            _ => Value::Object(Default::default()),
        };

        UserForUpdate {
            email: opt_string(&obj, "email"),
            name: opt_string(&obj, "name"),
            reset_token: opt_string(&obj, "reset_token"),
            mfa_secret: opt_string(&obj, "mfa_secret"),
            is_admin: opt_bool(&obj, "is_admin"),
            is_active: opt_bool(&obj, "is_active"),
            mfa_enabled: opt_bool(&obj, "mfa_enabled"),
        }
    }
}

#[derive(Fields)]
pub struct UserForInsert {
    pub username: String,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForLogin {
    pub id: i64,
    pub username: String,
    pub mfa_enabled: bool,
    pub mfa_secret: String,

    pub pwd: Option<String>,
    pub pwd_salt: Uuid,
    pub token_salt: Uuid,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForAuth {
    pub id: i64,
    pub username: String,

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

#[derive(FilterNodes, Deserialize, Default, Debug, Clone)]
pub struct UserFilter {
    id: Option<OpValsInt64>,

    name: Option<OpValsString>,
}

pub struct UserBmc;

impl DbBmc for UserBmc {
    const TABLE: &'static str = "user";
}

impl UserBmc {
    pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
    where
        E: UserBy + HasSeaFields,
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

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        base::delete::<Self>(ctx, mm, id).await
    }

    pub async fn count(
        ctx: &Ctx,
        mm: &ModelManager,
        filters: Option<Vec<UserFilter>>,
    ) -> Result<i64> {
        base::count::<Self, _>(ctx, mm, filters).await
    }

    pub async fn first_by_username<E>(
        _ctx: &Ctx,
        mm: &ModelManager,
        username: &str,
    ) -> Result<Option<E>>
    where
        E: UserBy + HasSeaFields,
    {
        let db = mm.db();

        let mut query = Query::select();
        query
            .from(Self::table_ref())
            .column(Asterisk)
            .and_where(Expr::col(UserIden::Username).eq(username));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        debug!("SQL: {sql}");
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
            .column(Asterisk)
            .and_where(Expr::col(UserIden::ResetToken).eq(token));

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
        let user = sqlx::query_as_with::<_, E, _>(&sql, values)
            .fetch_optional(db)
            .await?;

        Ok(user)
    }

    pub async fn create(ctx: &Ctx, mm: &ModelManager, user_c: UserForCreate) -> Result<i64> {
        base::create::<Self, _>(ctx, mm, user_c).await
    }

    pub async fn update(
        ctx: &Ctx,
        mm: &ModelManager,
        id: i64,
        group_u: UserForUpdate,
    ) -> Result<()> {
        base::update::<Self, _>(ctx, mm, id, group_u).await
    }

    pub async fn update_pwd(ctx: &Ctx, mm: &ModelManager, id: i64, pwd_clear: &str) -> Result<()> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;
    type TestResult<T = ()> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

    #[tokio::test]
    async fn test_first_ok_demo1() -> TestResult {
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_username = "demo1@uzh.ch";

        let user: User = UserBmc::first_by_username(&ctx, &mm, fx_username)
            .await?
            .ok_or_else(|| std::io::Error::other("Should have user 'demo1'"))?;

        assert_eq!(user.username, fx_username);

        Ok(())
    }
}
