/* SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause */

#ifndef __URDMA_H__
#define __URDMA_H__

#include <rdma/ib_verbs.h>

struct urdma_dev {
	struct ib_device ibdev;
	int id;

	union ib_gid gid;
};

static inline struct urdma_dev *to_udev(struct ib_device *ibdev)
{
	return container_of(ibdev, struct urdma_dev, ibdev);
}

struct urdma_pd {
	struct ib_pd ibpd;
};

struct urdma_cq {
	struct ib_cq ibcq;
};

struct urdma_qp {
	struct ib_qp ibqp;
};

struct urdma_ucontext {
	struct ib_ucontext ibuc;
};

#endif /* __URDMA_H__ */
