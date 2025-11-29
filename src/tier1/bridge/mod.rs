pub mod swd;

#[cfg(all(not(feature = "std"), feature = "swd"))]
pub use swd::no_std::{BridgeSwdDown, BridgeSwdUp, RemoteSwd, SwdConnection};
#[cfg(feature = "std")]
pub use swd::std::{BridgeSwdDown, BridgeSwdUp, RemoteSwd, SwdConnection};
