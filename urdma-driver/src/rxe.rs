pub struct Rxe {
    pub(crate) rxe_context: *mut ffi::ibv_context,
}

impl Drop for Rxe {
    fn drop(&mut self) {
        unsafe { ffi::ibv_close_device(self.rxe_context) };
    }
}
