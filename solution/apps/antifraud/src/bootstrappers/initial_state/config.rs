use domain::user::{CreateUser, role::UserRole};
use fromenv::FromEnv;
use lib::domain::{
    into_validators,
    validation::{Optional, error::ValidationErrors},
};

#[derive(FromEnv)]
pub struct InitialStateConfig {
    #[env(nested)]
    pub admin: InitialStateAdminConfig,
}

#[derive(FromEnv)]
#[env(prefix = "ADMIN_")]
pub struct InitialStateAdminConfig {
    pub email: String,
    pub fullname: String,
    pub password: String,
}

impl TryFrom<&InitialStateAdminConfig> for CreateUser {
    type Error = ValidationErrors;

    fn try_from(config: &InitialStateAdminConfig) -> Result<Self, Self::Error> {
        let (errors, (email, full_name, password)) = into_validators!(
            config.email.clone(),
            config.fullname.clone(),
            config.password.clone()
        );

        errors.into_result(|ok| Self {
            email: email.validated(ok),
            full_name: full_name.validated(ok),
            password: password.validated(ok),
            role: UserRole::Admin,
            age: Optional::Missing,
            gender: Optional::Missing,
            marital_status: Optional::Missing,
            region: Optional::Missing,
        })
    }
}
