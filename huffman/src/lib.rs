#[cfg(feature = "encode")]
pub mod encode;

#[cfg(feature = "decode")]
pub mod decode;

mod prefix_code;

pub mod prelude {
    #[cfg(feature = "encode")]
    pub use super::encode::prelude::*;

    #[cfg(feature = "decode")]
    pub use super::decode::prelude::*;
}
