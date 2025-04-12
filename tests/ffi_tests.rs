#[cfg(test)]
mod tests {
    use rust_bindgen_cmathlib::mathlib_rust::*;

    #[test]
    fn test_add() {
        let a = 3;
        let b = 5;
        assert_eq!(a + b, lib_add(a, b));
    }

    #[test]
    fn test_subtract() {
        let a = 3;
        let b = 5;
        assert_eq!(a - b, lib_subtract(a, b));
    }

    #[test]
    fn test_multiply() {
        let a = 3;
        let b = 5;
        assert_eq!(a * b, lib_multiply(a, b));
    }

    #[test]
    fn test_divide() {
        let a = 3;
        let b = 5;
        assert_eq!(a as f64 / b as f64, lib_divide(a, b));
    }

    #[test]
    fn test_divide_by_zero() {
        let a = 3;
        let b = 0;
        assert_eq!(0.0, lib_divide(a, b));
    }
}
