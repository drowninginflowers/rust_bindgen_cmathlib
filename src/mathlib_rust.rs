use super::bindings::{
    add as mathlib_add,
    divide as mathlib_divide,
    multiply as mathlib_multiply,
    subtract as mathlib_subtract,
};

pub fn add(a: i32, b: i32) -> i32 {
    unsafe { mathlib_add(a, b) }
}

pub fn subtract(a: i32, b: i32) -> i32 {
    unsafe { mathlib_subtract(a, b) }
}

pub fn multiply(a: i32, b: i32) -> i32 {
    unsafe { mathlib_multiply(a, b) }
}

pub fn divide(a: i32, b: i32) -> f64 {
    unsafe { mathlib_divide(a, b) }
}
