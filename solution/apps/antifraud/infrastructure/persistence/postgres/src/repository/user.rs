use application::repository::user::UserRepository;
use domain::{
    email::Email,
    password_hash::PasswordHash,
    user::{CreateUser, User, region::UserRegion},
};
use lib::{
    async_trait,
    domain::{DomainType as _, Id},
    infrastructure::persistence::postgres::error::PostgresAdapterError,
    instrument_all,
    tap::Pipe as _,
};
use sqlx::{query_file_as, query_file_scalar};

use crate::{
    entity::user::{
        StoredUser, gender::StoredUserGender,
        marital_status::StoredUserMaritalStatus, role::StoredUserRole,
    },
    repository::PostgresRepositoryImpl,
};

#[async_trait]
#[instrument_all]
impl UserRepository for PostgresRepositoryImpl<User> {
    type AdapterError = PostgresAdapterError;

    async fn create(
        &self,
        id: Id<User>,
        source: CreateUser,
        password_hash: PasswordHash,
    ) -> Result<User, Self::AdapterError> {
        let mut connection = self.pool.get().await?;

        let id = id.value;
        let email = source.email.into_inner();
        let full_name = source.full_name.into_inner();
        let password_hash = password_hash.0;
        let age: Option<i16> =
            source.age.map(|age| age.into_inner().into()).into_option();
        let gender = source.gender.map(StoredUserGender::from).into_option();
        let marital_status = source
            .marital_status
            .map(StoredUserMaritalStatus::from)
            .into_option();
        let region = source.region.map(UserRegion::into_inner).into_option();
        let role: StoredUserRole = source.role.into();

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

    async fn find_by_id(
        &self,
        id: Id<User>,
    ) -> Result<Option<User>, Self::AdapterError> {
        let mut connection = self.pool.get().await?;

        query_file_as!(StoredUser, "sql/user/find_by_id.sql", id.value)
            .fetch_optional(&mut *connection)
            .await?
            .map(User::from)
            .pipe(Ok)
    }

    async fn find_by_email(
        &self,
        email: &Email,
    ) -> Result<Option<User>, Self::AdapterError> {
        let mut connection = self.pool.get().await?;

        query_file_as!(StoredUser, "sql/user/find_by_email.sql", email.as_ref())
            .fetch_optional(&mut *connection)
            .await?
            .map(User::from)
            .pipe(Ok)
    }

    async fn list(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<User>, Self::AdapterError> {
        let mut connection = self.pool.get().await?;

        query_file_as!(StoredUser, "sql/user/list.sql", limit, offset,)
            .fetch_all(&mut *connection)
            .await?
            .into_iter()
            .map(User::from)
            .collect::<Vec<_>>()
            .pipe(Ok)
    }

    async fn count(&self) -> Result<i64, Self::AdapterError> {
        let mut connection = self.pool.get().await?;

        query_file_scalar!("sql/user/count.sql")
            .fetch_one(&mut *connection)
            .await?
            .unwrap_or_default()
            .pipe(Ok)
    }

    async fn update(&self, source: User) -> Result<User, Self::AdapterError> {
        let mut connection = self.pool.get().await?;

        let id = source.id.value;
        let full_name = source.full_name.into_inner();
        let age: Option<i16> = source.age.map(|age| age.into_inner().into());
        let gender = source.gender.map(StoredUserGender::from);
        let marital_status =
            source.marital_status.map(StoredUserMaritalStatus::from);
        let region = source.region.map(UserRegion::into_inner);
        let role: StoredUserRole = source.role.into();
        let is_active: bool = source.status.into();

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
