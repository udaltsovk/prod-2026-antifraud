use fromenv::FromEnv;

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
