#[cfg(test)]
mod tests {
    use bindgen_simple::mathlib_rust;

    #[test]
    fn test_add() {
        let a = 3;
        let b = 5;
        assert_eq!(a + b, mathlib_rust::lib_add(a, b));
    }

    #[test]
    fn test_subtract() {
        let a = 3;
        let b = 5;
        assert_eq!(a - b, mathlib_rust::lib_subtract(a, b));
    }

    #[test]
    fn test_multiply() {
        let a = 3;
        let b = 5;
        assert_eq!(a * b, mathlib_rust::lib_multiply(a, b));
    }

    #[test]
    fn test_divide() {
        let a = 3;
        let b = 5;
        assert_eq!(a as f64 / b as f64, mathlib_rust::lib_divide(a, b));
    }

    #[test]
    fn test_divide_by_zero() {
        let a = 3;
        let b = 0;
        assert_eq!(0.0, mathlib_rust::lib_divide(a, b));
    }
}
