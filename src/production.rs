#[cfg(windows)]
#[doc(hidden)]
#[cfg(windows)]
#[cfg(feature = "registration")]
pub mod registration;

#[cfg(feature = "class")]
mod class;

#[doc(inline)]
#[cfg(feature = "class")]
pub use class::{Class, ClassAllocation};
