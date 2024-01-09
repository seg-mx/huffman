#[cfg(feature = "encode")]
pub mod encode;

pub mod prelude {
    #[cfg(feature = "encode")]
    pub use super::encode::prelude::*;
}
