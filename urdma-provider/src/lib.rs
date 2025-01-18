mod macros;
mod provider;
mod raw;

pub use provider::Provider;

pub type VerbsError = ::std::os::raw::c_int;
pub type Result<T = ()> = core::result::Result<T, VerbsError>;
