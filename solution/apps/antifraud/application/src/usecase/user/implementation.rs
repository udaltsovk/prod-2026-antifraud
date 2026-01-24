use domain::{
    email::Email,
    pagination::Pagination,
    session::CreateSession,
    user::{CreateUser, User, UserUpdate, role::UserRole, status::UserStatus},
};
use lib::{
    async_trait,
    domain::{
        Id,
        validation::{ExternalInput, error::ValidationResult},
    },
    instrument_all,
    tap::{Pipe as _, Tap as _},
};

use crate::{
    repository::{RepositoriesModuleExt, user::UserRepository as _},
    service::{ServicesModuleExt, hasher::HasherService as _},
    usecase::{
        UseCase,
        user::{
            CreateUserSource, GetUserByEmailSource, UserUseCase,
            error::{UserUseCaseError, UserUseCaseResult},
        },
    },
};

#[async_trait]
#[instrument_all]
impl<R, S> UserUseCase<R, S> for UseCase<R, S, User>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    async fn find_by_email(
        &self,
        user_email: &Email,
    ) -> UserUseCaseResult<R, S, Option<User>> {
        self.repositories
            .user_repository()
            .find_by_email(user_email)
            .await
            .map_err(R::Error::from)
            .map_err(UserUseCaseError::Repository)
    }

    async fn get_by_email(
        &self,
        user_email: Email,
        source: GetUserByEmailSource,
    ) -> UserUseCaseResult<R, S, User> {
        self.find_by_email(&user_email).await?.ok_or(
            UserUseCaseError::NotFoundByEmail {
                email: user_email,
                from_auth: source == GetUserByEmailSource::Auth,
            },
        )
    }

    async fn create(
        &self,
        source: CreateUserSource,
        input: ValidationResult<CreateUser>,
    ) -> UserUseCaseResult<R, S, User> {
        if source != CreateUserSource::User(UserRole::Admin)
            && source != CreateUserSource::Registration
        {
            return UserUseCaseError::MissingPermissions.pipe(Err);
        }

        let new_user = input.map_err(UserUseCaseError::Validation)?;

        if self.find_by_email(&new_user.email).await?.is_some() {
            return UserUseCaseError::EmailAlreadyUsed(new_user.email)
                .pipe(Err);
        }

        let password_hash = self
            .services
            .password_hasher_service()
            .hash(new_user.password.as_bytes())
            .map_err(S::Error::from)
            .map_err(UserUseCaseError::Service)?;

        self.repositories
            .user_repository()
            .create((Id::generate(), new_user, password_hash.into()))
            .await
            .map_err(R::Error::from)
            .map_err(UserUseCaseError::Repository)
    }

    async fn authorize(
        &self,
        input: CreateSession,
    ) -> UserUseCaseResult<R, S, User> {
        let user = self
            .get_by_email(input.email, GetUserByEmailSource::Auth)
            .await?;

        self.services
            .password_hasher_service()
            .verify(input.password.as_bytes(), &user.password_hash.0)
            .map_err(|_| UserUseCaseError::InvalidPassword)?;

        if user.status != UserStatus::Active {
            return UserUseCaseError::UserDeactivated.pipe(Err);
        }

        Ok(user)
    }

    async fn find_by_id(
        &self,
        (requester_id, requester_role): (Id<User>, UserRole),
        user_id: Id<User>,
    ) -> UserUseCaseResult<R, S, Option<User>> {
        if requester_role != UserRole::Admin && requester_id != user_id {
            return UserUseCaseError::MissingPermissions.pipe(Err);
        }

        self.repositories
            .user_repository()
            .find_by_id(user_id)
            .await
            .map_err(R::Error::from)
            .map_err(UserUseCaseError::Repository)?
            .pipe(Ok)
    }

    async fn get_by_id(
        &self,
        requester: (Id<User>, UserRole),
        user_id: Id<User>,
    ) -> UserUseCaseResult<R, S, User> {
        self.find_by_id(requester, user_id)
            .await?
            .ok_or(UserUseCaseError::NotFoundById(user_id))
    }

    async fn list(
        &self,
        requester_role: UserRole,
        input: ValidationResult<Pagination>,
    ) -> UserUseCaseResult<R, S, (Vec<User>, u64)> {
        if requester_role != UserRole::Admin {
            return UserUseCaseError::MissingPermissions.pipe(Err);
        }

        let (limit, offset) = input
            .map_err(UserUseCaseError::Validation)?
            .into_limit_offset();

        let items = self
            .repositories
            .user_repository()
            .list(limit, offset)
            .await
            .map_err(R::Error::from)
            .map_err(UserUseCaseError::Repository)?;

        let total = self
            .repositories
            .user_repository()
            .count()
            .await
            .map_err(R::Error::from)
            .map_err(UserUseCaseError::Repository)?;

        Ok((items, total.try_into().unwrap_or(u64::MIN)))
    }

    async fn update_by_id(
        &self,
        (requester_id, requester_role): (Id<User>, UserRole),
        user_id: Id<User>,
        (update_result, new_status, new_role): (
            ValidationResult<UserUpdate>,
            ExternalInput<bool>,
            ExternalInput<String>,
        ),
    ) -> UserUseCaseResult<R, S, User> {
        if requester_role != UserRole::Admin
            && !(new_status.is_missing() && new_role.is_missing())
        {
            return UserUseCaseError::MissingPermissions.pipe(Err);
        }

        let user_update =
            update_result.map_err(UserUseCaseError::Validation)?;

        let user = self
            .get_by_id((requester_id, requester_role), user_id)
            .await?;

        if user_update.eq(&user) {
            return Ok(user);
        }

        let updated_user = user_update.apply_to(user);

        self.repositories
            .user_repository()
            .update(updated_user)
            .await
            .map_err(R::Error::from)
            .map_err(UserUseCaseError::Repository)
    }

    async fn deactivate_by_id(
        &self,
        (requester_id, requester_role): (Id<User>, UserRole),
        user_id: Id<User>,
    ) -> UserUseCaseResult<R, S, User> {
        if requester_role != UserRole::Admin {
            return UserUseCaseError::MissingPermissions.pipe(Err);
        }

        let user = self
            .get_by_id((requester_id, requester_role), user_id)
            .await?;

        if user.status == UserStatus::Deactivated {
            return Ok(user);
        }

        let updated_user =
            user.tap_mut(|user| user.status = UserStatus::Deactivated);

        self.repositories
            .user_repository()
            .update(updated_user)
            .await
            .map_err(R::Error::from)
            .map_err(UserUseCaseError::Repository)
    }
}
