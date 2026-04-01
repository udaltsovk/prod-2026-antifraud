use application::repository::user::UserRepositoryImpl;
use domain::{
    email::Email,
    pagination::Pagination,
    password::PasswordHash,
    user::{CreateUser, User, filter::UserFilter, region::UserRegion},
};
use entrait::entrait;
use lib::{
    anyhow::Result,
    application::di::Has,
    async_trait,
    domain::{DomainType as _, Id},
    infrastructure::persistence::HasPoolExt as _,
    instrument_all,
    tap::Pipe as _,
};
use mobc_sqlx::{SqlxConnectionManager, mobc::Pool};
use sqlx::{Postgres, query_file_as, query_file_scalar};

use crate::{
    entity::user::{
        StoredUser, gender::StoredUserGender,
        marital_status::StoredUserMaritalStatus, role::StoredUserRole,
    },
    repository::PostgresRepositoryImpl,
};

#[entrait(ref)]
#[async_trait]
#[instrument_all]
impl UserRepositoryImpl for PostgresRepositoryImpl {
    async fn create_user<Deps>(
        deps: &Deps,
        (id, source, password_hash): (Id<User>, CreateUser, PasswordHash),
    ) -> Result<User>
    where
        Deps: Has<Pool<SqlxConnectionManager<Postgres>>>,
    {
        let id = id.value;
        let email = source.email.into_inner();
        let full_name = source.full_name.into_inner();
        let password_hash = password_hash.0.expose_secret();
        let age: Option<i16> = source.age.map(|age| age.into_inner().into());
        let gender = source.gender.map(StoredUserGender::from);
        let marital_status =
            source.marital_status.map(StoredUserMaritalStatus::from);
        let region = source.region.map(UserRegion::into_inner);
        let role: StoredUserRole = source.role.into();

        let mut connection = deps.get_connection().await?;

        let user = query_file_as!(
            StoredUser,
            "sql/user/create.sql",
            id,
            email,
            full_name,
            password_hash,
            age,
            gender as _,
            marital_status as _,
            region,
            role as _,
        )
        .fetch_one(&mut *connection)
        .await
        .map(User::from)?;

        Ok(user)
    }

    async fn find_user_by_id<Deps>(
        deps: &Deps,
        id: Id<User>,
    ) -> Result<Option<User>>
    where
        Deps: Has<Pool<SqlxConnectionManager<Postgres>>>,
    {
        let mut connection = deps.get_connection().await?;

        query_file_as!(StoredUser, "sql/user/find_by_id.sql", id.value)
            .fetch_optional(&mut *connection)
            .await?
            .map(User::from)
            .pipe(Ok)
    }

    async fn find_user_by_email<Deps>(
        deps: &Deps,
        email: &Email,
    ) -> Result<Option<User>>
    where
        Deps: Has<Pool<SqlxConnectionManager<Postgres>>>,
    {
        let mut connection = deps.get_connection().await?;

        query_file_as!(StoredUser, "sql/user/find_by_email.sql", email.as_ref())
            .fetch_optional(&mut *connection)
            .await?
            .map(User::from)
            .pipe(Ok)
    }

    async fn list_users<Deps>(
        deps: &Deps,
        UserFilter {
            pagination:
                Pagination {
                    limit,
                    offset,
                },
        }: UserFilter,
    ) -> Result<Vec<User>>
    where
        Deps: Has<Pool<SqlxConnectionManager<Postgres>>>,
    {
        let mut connection = deps.get_connection().await?;

        query_file_as!(StoredUser, "sql/user/list.sql", limit, offset,)
            .fetch_all(&mut *connection)
            .await?
            .into_iter()
            .map(User::from)
            .collect::<Vec<_>>()
            .pipe(Ok)
    }

    async fn count_users<Deps>(
        deps: &Deps,
        UserFilter {
            ..
        }: UserFilter,
    ) -> Result<i64>
    where
        Deps: Has<Pool<SqlxConnectionManager<Postgres>>>,
    {
        let mut connection = deps.get_connection().await?;

        query_file_scalar!("sql/user/count.sql")
            .fetch_one(&mut *connection)
            .await?
            .unwrap_or_default()
            .pipe(Ok)
    }

    async fn update_user<Deps>(deps: &Deps, source: User) -> Result<User>
    where
        Deps: Has<Pool<SqlxConnectionManager<Postgres>>>,
    {
        let id = source.id.value;
        let full_name = source.full_name.into_inner();
        let age: Option<i16> = source.age.map(|age| age.into_inner().into());
        let gender = source.gender.map(StoredUserGender::from);
        let marital_status =
            source.marital_status.map(StoredUserMaritalStatus::from);
        let region = source.region.map(UserRegion::into_inner);
        let role: StoredUserRole = source.role.into();
        let is_active: bool = source.status.into();

        let mut connection = deps.get_connection().await?;

        let user = query_file_as!(
            StoredUser,
            "sql/user/update.sql",
            id,
            full_name,
            age,
            gender as _,
            marital_status as _,
            region,
            role as _,
            is_active,
        )
        .fetch_one(&mut *connection)
        .await
        .map(User::from)?;

        Ok(user)
    }
}
