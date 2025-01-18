mod macros;
mod provider;

pub use provider::Provider;
pub type VerbsError = ::std::os::raw::c_int;
pub type Result<T = ()> = core::result::Result<T, VerbsError>;

#[repr(C)]
pub struct URdmaDevice {
    verbs_dev: ffi::verbs_device,
    driver: *const core::ffi::c_void,
}

mod driver;
