#[cfg(feature = "encode")]
pub mod encode;

mod prefix_code;

pub mod prelude {
    #[cfg(feature = "encode")]
    pub use super::encode::prelude::*;
}
