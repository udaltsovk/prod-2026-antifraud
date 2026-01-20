use lib::bootstrap::bootstrapper_ext_trait;

use crate::Modules;

pub mod initial_state;
pub mod rest_api;

bootstrapper_ext_trait!(Modules);
