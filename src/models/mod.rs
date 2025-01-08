
// By declaring `mod user;` we include the user.rs file as a sub-module.
// `pub use user::*;` re-exports types so they can be accessed from `crate::models::user::...` 
// or directly through `crate::models::...`.

pub(crate) mod user;
pub use user::*;
        