//! ffi.rs - FFI bindings to NovaOS ring-3 system calls.

#![allow(dead_code)]

extern "C" {
    pub fn sys_write(s: *const u8) -> i32;
    pub fn sys_exit(code: i32) -> !;
    pub fn sys_yield() -> i32;
}
