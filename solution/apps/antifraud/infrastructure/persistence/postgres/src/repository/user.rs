use application::repository::user::UserRepository;
use chrono::{DateTime, Utc};
use domain::{
    email::Email,
    password::PasswordHash,
    user::{
        CreateUser, User, is_active::UserStatus, region::UserRegion,
        role::UserRole,
    },
};
use lib::{
    async_trait,
    domain::{DomainType as _, Id},
    infrastructure::persistence::postgres::error::PostgresAdapterError,
    instrument_all,
    tap::{Conv as _, Pipe as _},
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

    async fn save(&self, source: User) -> Result<User, Self::AdapterError> {
        let mut connection = self.pool.get().await?;

        let id = source.id.value;
        let email = source.email.into_inner();
        let full_name = source.full_name.into_inner();
        let password_hash = source.password_hash.0;
        let age: Option<i16> = source.age.map(|age| age.into_inner().into());
        let gender = source.gender.map(StoredUserGender::from);
        let marital_status =
            source.marital_status.map(StoredUserMaritalStatus::from);
        let region = source.region.map(UserRegion::into_inner);
        let role: StoredUserRole = source.role.into();
        let is_active: bool = source.status.into();
        let created_at = source.created_at;

        let user = query_file_as!(
            StoredUser,
            "sql/user/save.sql",
            id,
            email,
            full_name,
            password_hash,
            age,
            gender as _,
            marital_status as _,
            region,
            role as _,
            is_active,
            created_at,
        )
        .fetch_one(&mut *connection)
        .await?
        .conv::<User>();

        Ok(user)
    }

    async fn create(
        &self,
        id: Id<User>,
        CreateUser {
            email,
            full_name,
            age,
            gender,
            marital_status,
            region,
            role,
            ..
        }: CreateUser,
        password_hash: PasswordHash,
    ) -> Result<User, Self::AdapterError> {
        self.save(User {
            id,
            email,
            full_name,
            password_hash,
            age: age.into(),
            gender: gender.into(),
            marital_status: marital_status.into(),
            region: region.into(),
            role,
            status: UserStatus::Active,
            created_at: Utc::now(),
            updated_at: DateTime::UNIX_EPOCH,
        })
        .await
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
        roles: Option<&[UserRole]>,
        status: Option<UserStatus>,
    ) -> Result<Vec<User>, Self::AdapterError> {
        let mut connection = self.pool.get().await?;

        let roles = roles.map(|roles| {
            roles
                .iter()
                .map(|role| StoredUserRole::from(*role))
                .collect::<Vec<_>>()
        });

        let is_active = status.map(bool::from);

        query_file_as!(
            StoredUser,
            "sql/user/list.sql",
            roles as _,
            is_active,
            limit,
            offset,
        )
        .fetch_all(&mut *connection)
        .await?
        .into_iter()
        .map(User::from)
        .collect::<Vec<_>>()
        .pipe(Ok)
    }

    async fn count(
        &self,
        roles: Option<&[UserRole]>,
        status: Option<UserStatus>,
    ) -> Result<i64, Self::AdapterError> {
        let mut connection = self.pool.get().await?;

        let roles = roles.map(|roles| {
            roles
                .iter()
                .map(|role| StoredUserRole::from(*role))
                .collect::<Vec<_>>()
        });

        let is_active = status.map(bool::from);

        query_file_scalar!("sql/user/count.sql", roles as _, is_active)
            .fetch_one(&mut *connection)
            .await?
            .unwrap_or_default()
            .pipe(Ok)
    }
}
