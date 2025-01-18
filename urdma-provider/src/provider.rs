use std::sync::Arc;

use super::Result;

/// verbs provider
pub trait Provider {
    /// Sized because we do not want store a fat pointer
    /// 'static because it will be stored in C side
    type Driver: Sized + 'static;

    /// init context
    ///
    /// guarantee to be called only once
    fn init(/* TODO(fh): args? */) -> Result;

    /// new driver
    fn new(_sysfs_name: &str) -> Result<Arc<Self::Driver>>;

    /// alloc pd
    fn alloc_pd(&self) -> *mut ffi::ibv_pd {
        unimplemented!()
    }

    /// dealloc pd
    fn dealloc_pd(&self, _pd: *mut ffi::ibv_pd) -> Result {
        unimplemented!()
    }

    // TODO(fh): wrap pointers
    /// query device
    fn query_device(
        &self,
        _input: *const ffi::ibv_query_device_ex_input,
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
        _cqe: core::ffi::c_int,
        _channel: *mut ffi::ibv_comp_channel,
        _comp_vector: core::ffi::c_int,
    ) -> *mut ffi::ibv_cq {
        unimplemented!()
    }

    // TODO(fh): API design
    fn destroy_cq(&self, _cq: *mut ffi::ibv_cq) -> Result {
        unimplemented!()
    }

    // TODO(fh): API design
    fn create_qp(&self, _pd: *mut ffi::ibv_pd, _init_attr: *mut ffi::ibv_qp_init_attr) -> *mut ffi::ibv_qp {
        unimplemented!()
    }

    // TODO(fh): API design
    fn destroy_qp(&self, _qp: *mut ffi::ibv_qp) -> Result {
        unimplemented!()
    }

    // TODO(fh): API design
    fn modify_qp(&self, _qp: *mut ffi::ibv_qp, _attr: *mut ffi::ibv_qp_attr, _attr_mask: core::ffi::c_int) -> Result {
        unimplemented!()
    }

    // TODO(fh): API design
    fn query_qp(
        &self,
        _qp: *mut ffi::ibv_qp,
        _attr: *mut ffi::ibv_qp_attr,
        _attr_mask: core::ffi::c_int,
        _init_attr: *mut ffi::ibv_qp_init_attr,
    ) -> Result {
        unimplemented!()
    }

    // TODO(fh): API design
    fn reg_mr(
        &self,
        _pd: *mut ffi::ibv_pd,
        _addr: *mut ::std::os::raw::c_void,
        _length: usize,
        _hca_va: u64,
        _access: core::ffi::c_int,
    ) -> *mut ffi::ibv_mr {
        unimplemented!()
    }

    // TODO(fh): API design
    fn dereg_mr(&self, _mr: *mut ffi::ibv_mr) -> Result {
        unimplemented!()
    }

    // TODO(fh): API design
    fn post_send(
        &self,
        _qp: *mut ffi::ibv_qp,
        _wr: *mut ffi::ibv_send_wr,
        _bad_wr: *mut *mut ffi::ibv_send_wr,
    ) -> Result {
        unimplemented!()
    }

    // TODO(fh): API design
    fn post_recv(
        &self,
        _qp: *mut ffi::ibv_qp,
        _wr: *mut ffi::ibv_recv_wr,
        _bad_wr: *mut *mut ffi::ibv_recv_wr,
    ) -> Result {
        unimplemented!()
    }

    // TODO(fh): API design
    fn poll_cq(&self, _cq: *mut ffi::ibv_cq, _num_entries: ::std::os::raw::c_int, _wc: *mut ffi::ibv_wc) -> Result {
        unimplemented!()
    }
}
