// Test that arm64e pointer authentication attributes and operand bundles
// are correctly emitted in the LLVM IR.

//@ add-minicore
//@ compile-flags: --target arm64e-apple-ios
//@ needs-llvm-components: aarch64

#![crate_type = "lib"]
#![feature(no_core, lang_items)]
#![no_core]

extern crate minicore;
use minicore::*;

extern "C" {
    fn opaque();
}

// CHECK: @GLOBAL_FN_PTR = {{.*}}global {{.*}} ptrauth (ptr @my_fn, i32 0
// CHECK: define {{.*}}void @my_fn(
// CHECK-SAME: #[[ATTRS:[0-9]+]]
#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn my_fn() {
    opaque();
}

// Verify that function pointers stored in globals are signed with ConstantPtrAuth.
#[no_mangle]
pub static mut GLOBAL_FN_PTR: unsafe extern "C" fn() = my_fn;

// Verify that indirect calls through function pointers use ptrauth operand bundles.
// CHECK-LABEL: define {{.*}}void @call_through_fn_ptr(
// CHECK: call void %{{.*}}(){{.*}} [ "ptrauth"(i32 0, i64 0) ]
#[no_mangle]
pub unsafe extern "C" fn call_through_fn_ptr(f: unsafe extern "C" fn()) {
    f();
}

// Verify that direct calls do NOT get ptrauth operand bundles.
// CHECK-LABEL: define {{.*}}void @call_direct(
// CHECK-NOT: ptrauth
// CHECK: call void @my_fn()
#[no_mangle]
pub unsafe extern "C" fn call_direct() {
    my_fn();
}

// CHECK: attributes #[[ATTRS]] = {
// CHECK-SAME: "ptrauth-auth-traps"
// CHECK-SAME: "ptrauth-calls"
// CHECK-SAME: "ptrauth-returns"
