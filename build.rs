fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/depthai_wrapper.cc")
        .includes(&[
            ".",
            "/usr/include/opencv4",
            "./deps/depthai-core/build/install/include",
            "./deps/depthai-core/build/install/include/depthai-shared/3rdparty",
            "./deps/depthai-core/build/install/lib/cmake/depthai/dependencies/include",
        ])
        .flag_if_supported("-std=c++14")
        .compile("depthai-rust");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/depthai.cc");
    println!("cargo:rerun-if-changed=include/depthai.h");
    println!(
        "cargo:rustc-link-search={}",
        "./deps/depthai-core/build/install/lib"
    );
    println!("cargo:rustc-link-lib=dylib=depthai-core");
    println!("cargo:rustc-link-lib=dylib=depthai-opencv");
}
