use domain::user::{CreateUser, role::UserRole};
use fromenv::FromEnv;
use lib::{
    presentation::api::rest::{
        errors::validation::FieldErrors, into_validators, validation::UserInput,
    },
    redact::Secret,
    tap::Conv as _,
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
    pub password: Secret<String>,
}

impl TryFrom<&InitialStateAdminConfig> for CreateUser {
    type Error = FieldErrors;

    fn try_from(config: &InitialStateAdminConfig) -> Result<Self, Self::Error> {
        let (errors, (email, full_name, password)) = into_validators!(
            field!(
                config.email.clone().conv::<UserInput<_>>(),
                required,
                "email"
            ),
            field!(
                config.fullname.clone().conv::<UserInput<_>>(),
                required,
                "fullname"
            ),
            field!(
                config.password.clone().conv::<UserInput<_>>(),
                required,
                "password"
            )
        );

        errors.into_result(|ok| Self {
            email: email.validated(ok),
            full_name: full_name.validated(ok),
            password: password.validated(ok),
            role: UserRole::Admin,
            age: None,
            gender: None,
            marital_status: None,
            region: None,
        })
    }
}
