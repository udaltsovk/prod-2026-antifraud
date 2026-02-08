use chrono::{DateTime, Utc};
use domain::{
    email::Email,
    session::CreateSession,
    user::{
        CreateUser, User, UserUpdate, filter::UserFilterInput, role::UserRole,
        status::UserStatus,
    },
};
use lib::{
    async_trait,
    domain::{
        Id,
        validation::{ExternalInput, error::ValidationResultWithFields},
    },
    instrument_all,
    tap::{Pipe as _, Tap as _},
};

use crate::usecase::{
    UseCase,
    user::{
        CreateUserSource, UserUseCase,
        error::{UserUseCaseError, UserUseCaseResult},
    },
};

#[async_trait]
#[instrument_all]
impl UserUseCase for UseCase<User> {
    async fn find_by_email(
        &self,
        user_email: &Email,
    ) -> UserUseCaseResult<Option<User>> {
        self.repositories
            .user()
            .find_by_email(user_email)
            .await
            .map_err(UserUseCaseError::Infrastructure)
    }

    async fn get_by_email(&self, user_email: Email) -> UserUseCaseResult<User> {
        self.find_by_email(&user_email)
            .await?
            .ok_or(UserUseCaseError::NotFoundByEmail(user_email))
    }

    async fn create(
        &self,
        source: CreateUserSource,
        input: ValidationResultWithFields<CreateUser>,
    ) -> UserUseCaseResult<User> {
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
            .password_hasher()
            .hash(&new_user.password.clone().into())
            .map_err(UserUseCaseError::Infrastructure)?;

        self.repositories
            .user()
            .create((Id::generate(), new_user, password_hash))
            .await
            .map_err(UserUseCaseError::Infrastructure)
    }

    async fn authorize(&self, input: CreateSession) -> UserUseCaseResult<User> {
        let user = self.find_by_email(&input.email).await?;

        self.services
            .password_hasher()
            .verify(
                &input.password.clone().into(),
                user.as_ref().map(|u| &u.password_hash),
            )
            .map_err(|_| UserUseCaseError::InvalidPassword)?;

        let user = user.expect("we can't match nonexistent user password successfully so user should be Some at this point");

        if user.status != UserStatus::Active {
            return UserUseCaseError::UserDeactivated.pipe(Err);
        }

        Ok(user)
    }

    async fn find_by_id(
        &self,
        (requester_id, requester_role): (Id<User>, UserRole),
        user_id: Id<User>,
    ) -> UserUseCaseResult<Option<User>> {
        if requester_role != UserRole::Admin && requester_id != user_id {
            return UserUseCaseError::MissingPermissions.pipe(Err);
        }

        self.repositories
            .user()
            .find_by_id(user_id)
            .await
            .map_err(UserUseCaseError::Infrastructure)?
            .pipe(Ok)
    }

    async fn get_by_id(
        &self,
        requester: (Id<User>, UserRole),
        user_id: Id<User>,
    ) -> UserUseCaseResult<User> {
        self.find_by_id(requester, user_id)
            .await?
            .ok_or(UserUseCaseError::NotFoundById(user_id))
    }

    async fn list(
        &self,
        requester_role: UserRole,
        input: ValidationResultWithFields<UserFilterInput>,
    ) -> UserUseCaseResult<(Vec<User>, u64)> {
        if requester_role != UserRole::Admin {
            return UserUseCaseError::MissingPermissions.pipe(Err);
        }

        let user_filter =
            input.map_err(UserUseCaseError::Validation)?.normalize();

        let items = self
            .repositories
            .user()
            .list(user_filter)
            .await
            .map_err(UserUseCaseError::Infrastructure)?;

        let total = self
            .repositories
            .user()
            .count(user_filter)
            .await
            .map_err(UserUseCaseError::Infrastructure)?;

        Ok((items, total.try_into().unwrap_or(u64::MIN)))
    }

    async fn update_by_id(
        &self,
        (requester_id, requester_role): (Id<User>, UserRole),
        user_id: Id<User>,
        (update_result, new_status, new_role): (
            ValidationResultWithFields<UserUpdate>,
            ExternalInput<bool>,
            ExternalInput<String>,
        ),
    ) -> UserUseCaseResult<User> {
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
            .user()
            .update(updated_user)
            .await
            .map_err(UserUseCaseError::Infrastructure)
    }

    async fn deactivate_by_id(
        &self,
        (requester_id, requester_role): (Id<User>, UserRole),
        user_id: Id<User>,
    ) -> UserUseCaseResult<User> {
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
            .user()
            .update(updated_user)
            .await
            .map_err(UserUseCaseError::Infrastructure)
    }

    async fn record_activity(
        &self,
        user_id: Id<User>,
    ) -> UserUseCaseResult<DateTime<Utc>> {
        self.repositories
            .user_activity()
            .record(user_id)
            .await
            .map_err(UserUseCaseError::Infrastructure)
    }
}
