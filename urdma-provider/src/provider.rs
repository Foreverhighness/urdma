use core::ptr::NonNull;
use std::sync::Arc;

pub struct VerbsError;

type Result<T = ()> = core::result::Result<T, VerbsError>;

/// verbs provider
pub trait Provider {
    /// Sized because we do not want store a fat pointer
    /// 'static because it will be stored in C side
    type Driver: Sized + 'static;

    /// init context
    ///
    /// guarantee to be called only once
    fn init();

    /// new driver
    fn new() -> Arc<Self::Driver>;

    /// alloc pd
    fn alloc_pd(&self) -> NonNull<ffi::ibv_pd> {
        unimplemented!()
    }

    /// dealloc pd
    fn dealloc_pd(&self, _pd: NonNull<ffi::ibv_pd>) -> Result {
        unimplemented!()
    }

    // TODO(fh): wrap pointers
    /// query device
    fn query_device(
        &self,
        _input: Option<&ffi::ibv_query_device_ex_input>,
        _device_attr: *mut ffi::ibv_device_attr,
        _attr_size: usize,
    ) -> Result {
        unimplemented!()
    }

    // TODO(fh): wrap pointers
    fn query_port(&self, _port_num: u8, _port_attr: *mut ffi::ibv_port_attr) -> Result {
        unimplemented!()
    }

    // TODO(fh): wrap pointers
    fn create_cq(
        &self,
        cqe: core::ffi::c_int,
        channel: *mut ffi::ibv_comp_channel,
        comp_vector: core::ffi::c_int,
    ) -> *mut ffi::ibv_cq {
        unimplemented!()
    }

    fn destroy_cq(cq: *mut ffi::ibv_cq) -> Result {
        unimplemented!()
    }

    fn create_qp(pd: *mut ffi::ibv_pd, init_attr: *mut ffi::ibv_qp_init_attr) -> *mut ffi::ibv_qp {
        unimplemented!()
    }

    fn destroy_qp(qp: *mut ffi::ibv_qp) -> Result {
        unimplemented!()
    }

    fn modify_qp(qp: *mut ffi::ibv_qp, attr: *mut ffi::ibv_qp_attr, attr_mask: core::ffi::c_int) -> Result {
        unimplemented!()
    }

    fn query_qp(
        qp: *mut ffi::ibv_qp,
        attr: *mut ffi::ibv_qp_attr,
        attr_mask: core::ffi::c_int,
        init_attr: *mut ffi::ibv_qp_init_attr,
    ) -> Result {
        unimplemented!()
    }

    fn reg_mr(
        pd: *mut ffi::ibv_pd,
        addr: *mut ::std::os::raw::c_void,
        length: usize,
        _hca_va: u64,
        access: core::ffi::c_int,
    ) -> *mut ffi::ibv_mr {
        unimplemented!()
    }

    fn dereg_mr(mr: *mut ffi::ibv_mr) -> Result {
        unimplemented!()
    }

    fn post_send(qp: *mut ffi::ibv_qp, wr: *mut ffi::ibv_send_wr, bad_wr: *mut *mut ffi::ibv_send_wr) -> Result {
        unimplemented!()
    }

    fn post_recv(qp: *mut ffi::ibv_qp, wr: *mut ffi::ibv_recv_wr, bad_wr: *mut *mut ffi::ibv_recv_wr) -> Result {
        unimplemented!()
    }

    fn poll_cq(cq: *mut ffi::ibv_cq, num_entries: i32, wc: *mut ffi::ibv_wc) -> i32 {
        unimplemented!()
    }
}
