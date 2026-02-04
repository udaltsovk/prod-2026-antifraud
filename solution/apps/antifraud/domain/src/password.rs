use lib::{DomainType, redact::Secret, tap::Pipe as _};

#[derive(DomainType, Debug)]
pub struct Password(pub(super) Secret<String>);

#[derive(Debug)]
pub struct PasswordHash(pub Secret<String>);

impl From<String> for PasswordHash {
    fn from(value: String) -> Self {
        value.pipe(Secret::new).pipe(Self)
    }
}
