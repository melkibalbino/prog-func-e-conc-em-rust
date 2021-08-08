#[allow(dead_code)]
pub fn factorial(num: i32) -> i32{
    if num < 2 { return 1; }

    num * factorial(num - 1)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_zero_is_one() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_one_is_one() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn factorial_of_negative_equal_to_one() {
        assert_eq!(factorial(-10), 1);
    }

    #[test]
    fn factorial_of_three_is_six() {
        assert_eq!(factorial(3), 6);
    }

    #[test]
    fn factorial_of_four_is_twentyfour() {
        assert_eq!(factorial(4), 24);
    }
}
