use derive_more::From;

#[derive(From)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct PasswordHash(pub String);
