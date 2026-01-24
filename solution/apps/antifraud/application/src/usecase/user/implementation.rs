use domain::{
    email::Email,
    pagination::Pagination,
    session::CreateSession,
    user::{
        CreateUser, RawUserAdminUpdate, User, UserCommonUpdate, UserUpdate,
        role::UserRole, status::UserStatus,
    },
};
use lib::{
    async_trait,
    domain::{
        Id, into_validators,
        validation::{
            Optional, Validator,
            error::{ValidationErrors, ValidationResult},
        },
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
            GetByEmailSource, UserUseCase,
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
        source: GetByEmailSource,
    ) -> UserUseCaseResult<R, S, User> {
        self.find_by_email(&user_email).await?.ok_or(
            UserUseCaseError::NotFoundByEmail {
                email: user_email,
                from_auth: source == GetByEmailSource::Auth,
            },
        )
    }

    async fn create(
        &self,
        creator_role: Option<UserRole>,
        source: ValidationResult<CreateUser>,
    ) -> UserUseCaseResult<R, S, User> {
        if let Some(role) = creator_role
            && role != UserRole::Admin
        {
            return UserUseCaseError::MissingPermissions.pipe(Err);
        }

        let source = source.map_err(UserUseCaseError::Validation)?;

        if self.find_by_email(&source.email).await?.is_some() {
            return UserUseCaseError::EmailAlreadyUsed(source.email).pipe(Err);
        }

        let password_hash = self
            .services
            .password_hasher_service()
            .hash(source.password.as_bytes())
            .map_err(S::Error::from)
            .map_err(UserUseCaseError::Service)?;

        self.repositories
            .user_repository()
            .create(Id::generate(), source, password_hash.into())
            .await
            .map_err(R::Error::from)
            .map_err(UserUseCaseError::Repository)
    }

    async fn authorize(
        &self,
        source: CreateSession,
    ) -> UserUseCaseResult<R, S, User> {
        let user = self
            .get_by_email(source.email, GetByEmailSource::Auth)
            .await?;

        self.services
            .password_hasher_service()
            .verify(source.password.as_bytes(), &user.password_hash.0)
            .map_err(|_| UserUseCaseError::InvalidPassword)?;

        if user.status != UserStatus::Active {
            return UserUseCaseError::UserDeactivated.pipe(Err);
        }

        Ok(user)
    }

    async fn find_by_id(
        &self,
        requester_id: Id<User>,
        requester_role: UserRole,
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
        requester_id: Id<User>,
        requester_role: UserRole,
        user_id: Id<User>,
    ) -> UserUseCaseResult<R, S, User> {
        self.find_by_id(requester_id, requester_role, user_id)
            .await?
            .ok_or(UserUseCaseError::NotFoundById(user_id))
    }

    async fn list(
        &self,
        requester_role: Option<UserRole>,
        pagination_result: ValidationResult<Pagination>,
    ) -> UserUseCaseResult<R, S, (Vec<User>, u64)> {
        if let Some(role) = requester_role
            && role != UserRole::Admin
        {
            return UserUseCaseError::MissingPermissions.pipe(Err);
        }

        let (limit, offset) = pagination_result
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
        requester_id: Id<User>,
        requester_role: UserRole,
        user_id: Id<User>,
        common_update_result: ValidationResult<UserCommonUpdate>,
        raw_admin_update: RawUserAdminUpdate,
    ) -> UserUseCaseResult<R, S, User> {
        if requester_role != UserRole::Admin
            && !(raw_admin_update.status.is_missing()
                && raw_admin_update.role.is_missing())
        {
            return UserUseCaseError::MissingPermissions.pipe(Err);
        }

        let user_update = Self::user_update_from_parts(
            requester_role,
            common_update_result,
            raw_admin_update,
        )
        .map_err(UserUseCaseError::Validation)?;

        let user = self
            .get_by_id(requester_id, requester_role, user_id)
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
        requester_id: Id<User>,
        requester_role: UserRole,
        user_id: Id<User>,
    ) -> UserUseCaseResult<R, S, User> {
        if requester_role != UserRole::Admin {
            return UserUseCaseError::MissingPermissions.pipe(Err);
        }

        let user = self
            .get_by_id(requester_id, requester_role, user_id)
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

impl<R, S> UseCase<R, S, User>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    fn user_update_from_parts(
        requester_role: UserRole,
        common_update_result: ValidationResult<UserCommonUpdate>,
        raw_admin_update: RawUserAdminUpdate,
    ) -> ValidationResult<UserUpdate> {
        let mut errors = ValidationErrors::new();

        let common: Validator<_> =
            Validator::from_result(common_update_result, &mut errors);

        let (admin_update_errors, (status, role)) =
            if requester_role == UserRole::Admin {
                let (update_errors, (status, role)) = into_validators!(
                    raw_admin_update.status,
                    raw_admin_update.role
                );

                let status = status.map(Optional::Present);
                let role = role.map(Optional::Present);

                (update_errors, (status, role))
            } else {
                into_validators!(raw_admin_update.status, raw_admin_update.role)
            };

        errors.extend(admin_update_errors);

        errors.into_result(|ok| UserUpdate {
            common: common.validated(ok),
            status: status.validated(ok),
            role: role.validated(ok),
        })
    }
}
