use application::repository::user::UserRepository;
use domain::{
    email::Email,
    user::{CreateUser, User, region::UserRegion},
};
use lib::{
    async_trait,
    domain::{DomainType as _, Id},
    infrastructure::persistence::postgres::error::PostgresAdapterError,
    instrument_all,
    tap::{Conv as _, Pipe as _},
};
use sqlx::{Acquire as _, query_file_as};

use crate::{
    entity::user::{
        StoredUser, gender::StoredUserGender,
        martial_status::StoredUserMartialStatus, role::StoredUserRole,
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
        password_hash: String,
    ) -> Result<User, Self::AdapterError> {
        let mut connection = self.pool.get().await?;
        let mut transaction = connection.begin().await?;

        let id = id.value;
        let email = source.email.into_inner();
        let full_name = source.full_name.into_inner();
        let age: Option<i16> = source.age.map(|age| age.into_inner().into());
        let gender = source.gender.map(StoredUserGender::from);
        let martial_status =
            source.martial_status.map(StoredUserMartialStatus::from);
        let region = source.region.map(UserRegion::into_inner);
        let role: StoredUserRole = source.role.into();

        let user = query_file_as!(
            StoredUser,
            "sql/user/create.sql",
            id,
            email,
            full_name,
            password_hash,
            age,
            gender as Option<StoredUserGender>,
            martial_status as Option<StoredUserMartialStatus>,
            region,
            role as StoredUserRole,
        )
        .fetch_one(&mut *transaction)
        .await?
        .conv::<User>();

        transaction.commit().await?;

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
}
