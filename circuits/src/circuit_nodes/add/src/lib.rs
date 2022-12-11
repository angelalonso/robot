use anyhow::Result;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn lib_cxxbridge_integer(some: i32) -> i32;
    }
}

pub fn lib_cxxbridge_integer(some: i32) -> i32 {
    some + 10
}
