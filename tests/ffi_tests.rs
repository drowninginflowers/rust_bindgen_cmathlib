#[cfg(test)]
mod tests {
    use rust_bindgen_cmathlib::mathlib_rust::*;

    #[test]
    fn test_add() {
        let a = 3;
        let b = 5;
        assert_eq!(a + b, add(a, b));
    }

    #[test]
    fn test_subtract() {
        let a = 3;
        let b = 5;
        assert_eq!(a - b, subtract(a, b));
    }

    #[test]
    fn test_multiply() {
        let a = 3;
        let b = 5;
        assert_eq!(a * b, multiply(a, b));
    }

    #[test]
    fn test_divide() {
        let a = 3;
        let b = 5;
        assert_eq!(a as f64 / b as f64, divide(a, b));
    }

    #[test]
    fn test_divide_by_zero() {
        let a = 3;
        let b = 0;
        assert_eq!(0.0, divide(a, b));
    }
}
