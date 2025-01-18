//! This file is heavily inspired by jonhoo's `ibverbs-sys` crate <https://github.com/jonhoo/rust-ibverbs/blob/v0.9.1/ibverbs-sys/build.rs>

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("failed to get current directory");
    let rdma_core_dir = format!("{manifest_dir}/../rdma-core-v55");
    println!("cargo:include={rdma_core_dir}/build/include");
    println!("cargo:rustc-link-search=native={rdma_core_dir}/build/lib");
    println!("cargo:rustc-link-lib=ibverbs");

    if Path::new(&format!("{rdma_core_dir}/CMakeLists.txt")).exists() {
        // don't touch source dir if not necessary
    } else if Path::new(".git").is_dir() {
        // initialize and update submodules
        Command::new("git")
            .args(["submodule", "update", "--init"])
            .status()
            .expect("Failed to update submodules.");
    } else {
        assert!(Path::new(&rdma_core_dir).is_dir(), "vendor source not included");
    }

    // build rdma-core
    // note that we only build it to generate the bindings!
    eprintln!("run cmake");
    let built_in = cmake::Config::new(&rdma_core_dir)
        .define("NO_MAN_PAGES", "1")
        // cmake crate defaults CMAKE_INSTALL_PREFIX to the output directory
        //
        //   https://github.com/rust-lang/cmake-rs/blob/94da9de2ea79ab6cad572e908864a160cf4847a9/src/lib.rs#L699-L703
        //
        // this results in overly long runtime paths on docs.rs, which then fail the build. it also
        // causes sadness for users trying to build since the bindings may fail to build for the
        // same reason (see https://github.com/jonhoo/rust-ibverbs/pull/41 for what was an
        // incomplete fix).
        //
        // since we never actually _install_ anything when building here, we should be able to
        // safely set this to any short path. simply by convention we set it to `/usr`.
        .define("CMAKE_INSTALL_PREFIX", "/usr")
        .no_build_target(true)
        .build();
    let built_in = built_in.to_str().expect("build directory path is not valid UTF-8");

    // generate the bindings
    eprintln!("run bindgen");
    let bindings = bindgen::Builder::default()
        .header(format!("{rdma_core_dir}/libibverbs/verbs.h"))
        .header(format!("{rdma_core_dir}/libibverbs/driver.h"))
        .clang_arg(format!("-I{built_in}/build/include/"))
        .allowlist_function("ibv_.*")
        .allowlist_type("ibv_.*")
        .allowlist_type("verbs_.*")
        .allowlist_var("IBV_LINK_LAYER_.*")
        .bitfield_enum("ibv_access_flags")
        .bitfield_enum("ibv_qp_attr_mask")
        .bitfield_enum("ibv_wc_flags")
        .bitfield_enum("ibv_send_flags")
        .bitfield_enum("ibv_port_cap_flags")
        .constified_enum_module("ibv_qp_type")
        .constified_enum_module("ibv_qp_state")
        .constified_enum_module("ibv_port_state")
        .constified_enum_module("ibv_wc_opcode")
        .constified_enum_module("ibv_wr_opcode")
        .constified_enum_module("ibv_wc_status")
        //.constified_enum_module("IBV_WC_.*")
        //.constified_enum_module("IBV_WR_.*")
        //.constified_enum_module("IBV_QPS_.*")
        //.constified_enum_module("IBV_PORT_.*")
        .derive_default(true)
        .derive_debug(true)
        .prepend_enum_name(false)
        .blocklist_type("ibv_wc")
        .size_t_is_usize(true)
        .generate()
        .expect("Unable to generate bindings");

    // write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings");
}
