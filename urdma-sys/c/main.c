/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */

#include <linux/module.h>

#include "urdma.h"

MODULE_AUTHOR("Hange Shen <Foreverhighness@gmail.com>");
MODULE_DESCRIPTION("User space RDMA driver");
MODULE_LICENSE("Dual BSD/GPL");

#define NUM_DEV 2
static struct urdma_dev *urdma_devs[NUM_DEV] = {};

#pragma region operations

static int urdma_query_port(struct ib_device *ibdev, u32 port_num,
			    struct ib_port_attr *attr)
{
	memset(attr, 0, sizeof(*attr));
	attr->gid_tbl_len = 1;
	attr->state = IB_PORT_ACTIVE;
	attr->phys_state = IB_PORT_PHYS_STATE_LINK_UP;
	return 0;
}
static int urdma_get_port_immutable(struct ib_device *ibdev, u32 port_num,
				    struct ib_port_immutable *immutable)
{
	memset(immutable, 0, sizeof(*immutable));
	immutable->gid_tbl_len = 1;
	return 0;
}

static int urdma_alloc_ucontext(struct ib_ucontext *ibuc,
				struct ib_udata *udata)
{
	pr_info("urdma alloc ucontext\n");
	return 0;
}

static void urdma_dealloc_ucontext(struct ib_ucontext *ibuc)
{
	pr_info("urdma dealloc ucontext\n");
}

int urdma_query_gid(struct ib_device *ibdev, u32 port_num, int index,
		    union ib_gid *gid)
{
	struct urdma_dev *urdma = to_udev(ibdev);
	memcpy(gid, &urdma->gid, sizeof(*gid));
	return 0;
}

static int urdma_query_device(struct ib_device *ibdev,
			      struct ib_device_attr *attr,
			      struct ib_udata *udata)
{
	return 0;
}

static int urdma_alloc_pd(struct ib_pd *pd, struct ib_udata *udata)
{
	return 0;
}
static int urdma_dealloc_pd(struct ib_pd *pd, struct ib_udata *udata)
{
	return 0;
}

static int urdma_create_qp(struct ib_qp *qp, struct ib_qp_init_attr *init_attr,
			   struct ib_udata *udata)
{
	return 0;
}
static int urdma_destroy_qp(struct ib_qp *qp, struct ib_udata *udata)
{
	return 0;
}
static int urdma_modify_qp(struct ib_qp *qp, struct ib_qp_attr *attr,
			   int attr_mask, struct ib_udata *udata)
{
	return 0;
}

static int urdma_post_send(struct ib_qp *ibqp, const struct ib_send_wr *wr,
			   const struct ib_send_wr **bad_wr)
{
	return 0;
}
static int urdma_post_recv(struct ib_qp *ibqp, const struct ib_recv_wr *wr,
			   const struct ib_recv_wr **bad_wr)
{
	return 0;
}

static int urdma_create_cq(struct ib_cq *ibcq,
			   const struct ib_cq_init_attr *attr,
			   struct ib_udata *udata)
{
	return 0;
}
static int urdma_destroy_cq(struct ib_cq *cq, struct ib_udata *udata)
{
	return 0;
}
static int urdma_poll_cq(struct ib_cq *ibcq, int num_entries, struct ib_wc *wc)
{
	return 0;
}

static int urdma_req_notify_cq(struct ib_cq *ibcq,
			       enum ib_cq_notify_flags flags)
{
	return 0;
}

static struct ib_mr *urdma_get_dma_mr(struct ib_pd *ibpd, int access)
{
	struct ib_mr *mr;
	mr = kzalloc(sizeof(*mr), GFP_KERNEL);
	if (!mr)
		return ERR_PTR(-ENOMEM);
	return mr;
}
static struct ib_mr *urdma_reg_user_mr(struct ib_pd *pd, u64 start, u64 length,
				       u64 virt_addr, int access_flags,
				       struct ib_udata *udata)
{
	struct ib_mr *mr;
	mr = kzalloc(sizeof(*mr), GFP_KERNEL);
	if (!mr)
		return ERR_PTR(-ENOMEM);
	return mr;
}
static int urdma_dereg_mr(struct ib_mr *mr, struct ib_udata *udata)
{
	kfree(mr);
	return 0;
}

#pragma endregion operations

static const struct ib_device_ops urdma_device_ops = {
	.owner = THIS_MODULE,
	.driver_id = RDMA_DRIVER_UNKNOWN,
	.uverbs_abi_ver = 1,

	// mandatory methods <https://elixir.bootlin.com/linux/v6.8/source/drivers/infiniband/core/device.c#L267>
	.query_device = urdma_query_device,
	.query_port = urdma_query_port,

	.alloc_pd = urdma_alloc_pd,
	.dealloc_pd = urdma_dealloc_pd,
	INIT_RDMA_OBJ_SIZE(ib_pd, urdma_pd, ibpd),

	.create_qp = urdma_create_qp,
	.modify_qp = urdma_modify_qp,
	.destroy_qp = urdma_destroy_qp,
	INIT_RDMA_OBJ_SIZE(ib_qp, urdma_qp, ibqp),

	.post_send = urdma_post_send,
	.post_recv = urdma_post_recv,

	.create_cq = urdma_create_cq,
	.destroy_cq = urdma_destroy_cq,
	.poll_cq = urdma_poll_cq,
	INIT_RDMA_OBJ_SIZE(ib_cq, urdma_cq, ibcq),

	.req_notify_cq = urdma_req_notify_cq,

	.get_dma_mr = urdma_get_dma_mr,
	.reg_user_mr = urdma_reg_user_mr,
	.dereg_mr = urdma_dereg_mr,

	.get_port_immutable = urdma_get_port_immutable,

	// uverbs required methods
	.alloc_ucontext = urdma_alloc_ucontext,
	.dealloc_ucontext = urdma_dealloc_ucontext,
	INIT_RDMA_OBJ_SIZE(ib_ucontext, urdma_ucontext, ibuc),

	// rc_pingpong required methods
	.query_gid = urdma_query_gid,
};

static int urdma_register_device(struct urdma_dev *urdma)
{
	struct ib_device *dev = &urdma->ibdev;
	int err;

	strscpy(dev->node_desc, "urdma", sizeof(dev->node_desc));

	dev->node_type = RDMA_NODE_UNSPECIFIED;
	dev->phys_port_cnt = 1;
	dev->num_comp_vectors = num_possible_cpus();
	dev->local_dma_lkey = 0;

	ib_set_device_ops(dev, &urdma_device_ops);

	err = ib_register_device(dev, "urdma%d", NULL);
	if (err) {
		pr_err("register device failed");
	}

	return err;
}

#pragma region alloc

static struct urdma_dev *urdma_alloc_device(const int id)
{
	struct urdma_dev *urdma;

	urdma = ib_alloc_device(urdma_dev, ibdev);
	if (!urdma) {
		pr_err("alloc failed on id: %d\n", id);
		return NULL;
	}

	urdma->id = id;
	pr_info("alloc for id: %d\n", id);
	ib_device_put(&urdma->ibdev);
	return urdma;
}

static void urdma_dealloc_device(struct urdma_dev *urdma)
{
	WARN_ON(urdma == NULL);
	pr_info("dealloc for id: %d\n", urdma->id);
	ib_dealloc_device(&urdma->ibdev);
}

#pragma endregion alloc

static int __init urdma_init_module(void)
{
	int err;
	int i;

	pr_info("urdma module loaded\n");

	err = request_module("ib_uverbs");
	if (err) {
		pr_err("failed request ib_uverbs");
		return err;
	}

	for (i = 0; i < NUM_DEV; i++) {
		urdma_devs[i] = urdma_alloc_device(i);
		if (!urdma_devs[i]) {
			err = -ENOMEM;
			goto err_dealloc;
		}

		err = urdma_register_device(urdma_devs[i]);
		if (err) {
			urdma_dealloc_device(urdma_devs[i]);
			goto err_dealloc;
		}
	}

	return 0;

err_dealloc:
	while (--i) {
		urdma_dealloc_device(urdma_devs[i]);
		urdma_devs[i] = NULL;
	}
	return err;
}

static void __exit urdma_exit_module(void)
{
	int i;
	pr_info("urdma module unloaded\n");

	for (i = 0; i < NUM_DEV; i++) {
		if (!urdma_devs[i]) {
			urdma_dealloc_device(urdma_devs[i]);
			urdma_devs[i] = NULL;
		}
	}
}

module_init(urdma_init_module);
module_exit(urdma_exit_module);
