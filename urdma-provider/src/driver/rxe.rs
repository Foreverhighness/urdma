use std::sync::Arc;

use crate::{Provider, Result};

const RXE_DEVICE_NAME: &str = "rxe";

struct Rxe {
    rxe_context: *mut ffi::ibv_context,
}

impl Drop for Rxe {
    fn drop(&mut self) {
        unsafe { ffi::ibv_close_device(self.rxe_context) };
    }
}

impl Provider for Rxe {
    type Driver = Rxe;

    fn init() -> Result {
        let _ = env_logger::try_init();
        Ok(())
    }

    fn new(sysfs_name: &str) -> Result<Arc<Self::Driver>> {
        log::info!("Creating new RDMA device with sysfs name: {sysfs_name}");

        let mut num_devices = 0;

        // Safety: `num_devices` is a valid pointer.
        let list = unsafe { ffi::ibv_get_device_list(&raw mut num_devices) };

        // Safety: `list` is at least `num_devices` long.
        let device_list = unsafe { std::slice::from_raw_parts(list, num_devices.try_into().unwrap()) };

        let Some(rxe_device) = device_list.iter().copied().find(|dev_ptr| {
            unsafe { dev_ptr.as_ref() }
                .map(|dev| dev.name.as_ptr())
                .map(|name| unsafe { core::ffi::CStr::from_ptr(name) })
                .map(|name| name.to_str().map(|name| name.contains(RXE_DEVICE_NAME)))
                == Some(Ok(true))
        }) else {
            return Err(-1);
        };

        let rxe = unsafe { ffi::ibv_open_device(rxe_device) };

        unsafe { ffi::ibv_free_device_list(list) };

        Ok(Arc::new(Rxe { rxe_context: rxe }))
    }

    fn alloc_pd(&self) -> *mut ffi::ibv_pd {
        log::info!("Allocating protection domain");

        unsafe { ffi::ibv_alloc_pd(self.rxe_context) }
    }

    fn dealloc_pd(&self, pd: *mut ffi::ibv_pd) -> Result {
        log::info!("Deallocating protection domain");

        let _old_context = unsafe { pd.as_ref() }.unwrap().context;
        let rxe_context = self.rxe_context;

        let pd_mut = unsafe { pd.as_mut() }.unwrap();

        pd_mut.context = rxe_context;
        let rc = unsafe { ffi::ibv_dealloc_pd(pd) };

        if rc == 0 { Ok(()) } else { Err(rc) }
    }

    fn query_device(
        &self,
        _input: *const ffi::ibv_query_device_ex_input,
        device_attr: *mut ffi::ibv_device_attr,
        _attr_size: usize,
    ) -> Result {
        log::info!("Querying device attributes");

        let rxe_context = self.rxe_context;
        let ctx = unsafe { rxe_context.as_ref() }.unwrap();

        let rc = unsafe { ctx.ops._compat_query_device.unwrap()(rxe_context, device_attr) };

        if rc == 0 { Ok(()) } else { Err(rc) }
    }

    fn query_port(&self, port_num: u8, port_attr: *mut ffi::ibv_port_attr) -> Result {
        log::info!("Querying port attributes");

        let rxe_context = self.rxe_context;
        let ctx = unsafe { rxe_context.as_ref() }.unwrap();

        let rc = unsafe { ctx.ops._compat_query_port.unwrap()(rxe_context, port_num, port_attr.cast()) };

        if rc == 0 { Ok(()) } else { Err(rc) }
    }

    fn create_cq(
        &self,
        cqe: core::ffi::c_int,
        channel: *mut ffi::ibv_comp_channel,
        comp_vector: core::ffi::c_int,
    ) -> *mut ffi::ibv_cq {
        log::info!("Creating completion queue");

        unsafe { ffi::ibv_create_cq(self.rxe_context, cqe, core::ptr::null_mut(), channel, comp_vector) }
    }

    fn destroy_cq(&self, cq: *mut ffi::ibv_cq) -> Result {
        log::info!("Destroying completion queue");

        let _old_context = unsafe { cq.as_ref() }.unwrap().context;
        let rxe_context = self.rxe_context;

        let cq_mut = unsafe { cq.as_mut() }.unwrap();

        cq_mut.context = rxe_context;
        let rc = unsafe { ffi::ibv_destroy_cq(cq) };

        if rc == 0 { Ok(()) } else { Err(rc) }
    }

    fn create_qp(&self, pd: *mut ffi::ibv_pd, init_attr: *mut ffi::ibv_qp_init_attr) -> *mut ffi::ibv_qp {
        log::info!("Creating queue pair");

        let old_context = unsafe { pd.as_ref() }.unwrap().context;
        let rxe_context = self.rxe_context;

        let pd_mut = unsafe { pd.as_mut() }.unwrap();

        pd_mut.context = rxe_context;
        let qp = unsafe { ffi::ibv_create_qp(pd, init_attr) };
        pd_mut.context = old_context;

        let qp_mut = unsafe { qp.as_mut() }.unwrap();
        qp_mut.context = old_context;

        qp
    }

    fn destroy_qp(&self, qp: *mut ffi::ibv_qp) -> Result {
        log::info!("Destroying queue pair");

        let _old_context = unsafe { qp.as_ref() }.unwrap().context;
        let rxe_context = self.rxe_context;

        let qp_mut = unsafe { qp.as_mut() }.unwrap();

        qp_mut.context = rxe_context;
        let rc = unsafe { ffi::ibv_destroy_qp(qp) };

        if rc == 0 { Ok(()) } else { Err(rc) }
    }

    fn modify_qp(&self, qp: *mut ffi::ibv_qp, attr: *mut ffi::ibv_qp_attr, attr_mask: core::ffi::c_int) -> Result {
        log::info!("Modifying queue pair");

        let old_context = unsafe { qp.as_ref() }.unwrap().context;
        let rxe_context = self.rxe_context;

        let qp_mut = unsafe { qp.as_mut() }.unwrap();

        qp_mut.context = rxe_context;
        let rc = unsafe { ffi::ibv_modify_qp(qp, attr, attr_mask) };
        qp_mut.context = old_context;

        if rc == 0 { Ok(()) } else { Err(rc) }
    }

    fn query_qp(
        &self,
        qp: *mut ffi::ibv_qp,
        attr: *mut ffi::ibv_qp_attr,
        attr_mask: core::ffi::c_int,
        init_attr: *mut ffi::ibv_qp_init_attr,
    ) -> Result {
        log::info!("Querying queue pair");

        let old_context = unsafe { qp.as_ref() }.unwrap().context;
        let rxe_context = self.rxe_context;

        let qp_mut = unsafe { qp.as_mut() }.unwrap();

        qp_mut.context = rxe_context;
        let rc = unsafe { ffi::ibv_query_qp(qp, attr, attr_mask, init_attr) };
        qp_mut.context = old_context;

        if rc == 0 { Ok(()) } else { Err(rc) }
    }

    fn reg_mr(
        &self,
        pd: *mut ffi::ibv_pd,
        addr: *mut ::std::os::raw::c_void,
        length: usize,
        _hca_va: u64,
        access: core::ffi::c_int,
    ) -> *mut ffi::ibv_mr {
        log::info!("Registering memory region");

        let old_context = unsafe { pd.as_ref() }.unwrap().context;
        let rxe_context = self.rxe_context;

        let pd_mut = unsafe { pd.as_mut() }.unwrap();

        pd_mut.context = rxe_context;
        let mr = unsafe { ffi::ibv_reg_mr(pd, addr, length, access) };
        pd_mut.context = old_context;

        let mr_mut = unsafe { mr.as_mut() }.unwrap();
        mr_mut.context = old_context;

        mr
    }

    fn dereg_mr(&self, mr: *mut ffi::ibv_mr) -> Result {
        log::info!("Deregistering memory region");

        let _old_context = unsafe { mr.as_ref() }.unwrap().context;
        let rxe_context = self.rxe_context;

        let mr_mut = unsafe { mr.as_mut() }.unwrap();

        mr_mut.context = rxe_context;
        let rc = unsafe { ffi::ibv_dereg_mr(mr) };

        if rc == 0 { Ok(()) } else { Err(rc) }
    }

    fn post_send(&self, qp: *mut ffi::ibv_qp, wr: *mut ffi::ibv_send_wr, bad_wr: *mut *mut ffi::ibv_send_wr) -> Result {
        log::trace!("Posting send work request");

        let old_context = unsafe { qp.as_ref() }.unwrap().context;
        let rxe_context = self.rxe_context;
        let ctx = unsafe { rxe_context.as_ref() }.unwrap();

        let qp_mut = unsafe { qp.as_mut() }.unwrap();

        qp_mut.context = rxe_context;
        let rc = unsafe { ctx.ops.post_send.unwrap()(qp, wr, bad_wr) };
        qp_mut.context = old_context;

        if rc == 0 { Ok(()) } else { Err(rc) }
    }

    fn post_recv(&self, qp: *mut ffi::ibv_qp, wr: *mut ffi::ibv_recv_wr, bad_wr: *mut *mut ffi::ibv_recv_wr) -> Result {
        log::trace!("Posting receive work request");

        let old_context = unsafe { qp.as_ref() }.unwrap().context;
        let rxe_context = self.rxe_context;
        let ctx = unsafe { rxe_context.as_ref() }.unwrap();

        let qp_mut = unsafe { qp.as_mut() }.unwrap();

        qp_mut.context = rxe_context;
        let rc = unsafe { ctx.ops.post_recv.unwrap()(qp, wr, bad_wr) };
        qp_mut.context = old_context;

        if rc == 0 { Ok(()) } else { Err(rc) }
    }

    fn poll_cq(&self, cq: *mut ffi::ibv_cq, num_entries: ::std::os::raw::c_int, wc: *mut ffi::ibv_wc) -> Result {
        log::trace!("Polling completion queue");

        let old_context = unsafe { cq.as_ref() }.unwrap().context;
        let rxe_context = self.rxe_context;
        let ctx = unsafe { rxe_context.as_ref() }.unwrap();

        let cq_mut = unsafe { cq.as_mut() }.unwrap();

        cq_mut.context = rxe_context;
        let rc = unsafe { ctx.ops.poll_cq.unwrap()(cq, num_entries, wc) };
        cq_mut.context = old_context;

        if rc == 0 { Ok(()) } else { Err(rc) }
    }
}
