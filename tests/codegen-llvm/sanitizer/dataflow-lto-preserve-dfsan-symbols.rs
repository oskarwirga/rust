// Verifies that DFSan-instrumented functions (with `.dfsan` suffix) are preserved
// during LTO and not stripped by dead code elimination.
//
// When DFSan instruments a function, it renames it by appending `.dfsan` and creates
// a wrapper with the original name. Without the fix in `prepare_lto`, ThinLTO would
// not recognize the `.dfsan` suffixed functions as exported and would strip them,
// causing linker errors when C/C++ code compiled with DFSan tries to call the
// instrumented versions.
//
//@ needs-sanitizer-dataflow
//@ only-linux
//
//@ revisions: DFSAN DFSAN-FAT-LTO DFSAN-THIN-LTO
//@ compile-flags: -Zsanitizer=dataflow -Ctarget-feature=-crt-static -C unsafe-allow-abi-mismatch=sanitizer
//@[DFSAN] compile-flags: -Copt-level=1
//@[DFSAN-FAT-LTO] compile-flags: -Cprefer-dynamic=false -Clto=fat
//@[DFSAN-THIN-LTO] compile-flags: -Cprefer-dynamic=false -Clto=thin

#![crate_type = "staticlib"]

// CHECK: define {{.*}}my_rust_func{{.*}}.dfsan
#[no_mangle]
pub unsafe extern "C" fn my_rust_func(x: i32) -> i32 {
    x + 1
}

// CHECK: define {{.*}}another_func{{.*}}.dfsan
#[no_mangle]
pub unsafe extern "C" fn another_func(a: i32, b: i32) -> i32 {
    a + b
}
