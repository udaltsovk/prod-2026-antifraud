// use std::sync::LazyLock;

use lib::infrastructure::persistence::repository_impl_struct;
// use lib::infrastructure::persistence::redis::Namespace;
use mobc_redis::{RedisConnectionManager, mobc};

// static META_NAMESPACE: LazyLock<Namespace> =
//     LazyLock::new(|| Namespace::new("antifraud").nest("monolyth"));

repository_impl_struct!(Redis, RedisConnectionManager);
