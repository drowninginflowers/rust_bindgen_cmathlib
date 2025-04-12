mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod mathlib_rust {
    use super::bindings::{add, divide, multiply, subtract};

    pub fn lib_add(a: i32, b: i32) -> i32 {
        unsafe { add(a, b) }
    }

    pub fn lib_subtract(a: i32, b: i32) -> i32 {
        unsafe { subtract(a, b) }
    }

    pub fn lib_multiply(a: i32, b: i32) -> i32 {
        unsafe { multiply(a, b) }
    }

    pub fn lib_divide(a: i32, b: i32) -> f64 {
        unsafe { divide(a, b) }
    }
}
