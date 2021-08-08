#[allow(dead_code)]
fn divider(num: i64) -> Vec<i64> {
    if num <= 1 { panic!("{} <= 1 não é válido", num); }

    (2..=num).filter(|x| num % x == 0).collect::<Vec<i64>>()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn zero_is_not_valid() {
        divider(0);
    }

    #[test]
    #[should_panic]
    fn one_is_not_valid() {
        divider(1);
    }

    #[test]
    #[should_panic]
    fn negatives_are_not_valid() {
        divider(-10);
    }

    #[test]
    fn two_divides_two() {
        assert_eq!(divider(2), vec![2])
    }

    #[test]
    fn three_divides_three() {
        assert_eq!(divider(3), vec![3])
    }

    #[test]
    fn dividing_numbers_of_four() {
        assert_eq!(divider(4), vec![2, 4])
    }

    #[test]
    fn list_of_dividers_twentyfour() {
        assert_eq!(divider(24), vec![2, 3, 4, 6, 8, 12, 24])
    }
}
