use std::sync::LazyLock;

use lib::infrastructure::persistence::{
    redis::Namespace, repository_impl_struct,
};

mod user_activity;

static META_NAMESPACE: LazyLock<Namespace> =
    LazyLock::new(|| Namespace::new("antifraud").nest("monolyth"));

repository_impl_struct!(Redis);
