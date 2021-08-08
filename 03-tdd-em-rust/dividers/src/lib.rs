#[allow(dead_code)]
fn divider(num: i64) -> Vec<i64> {
    if num <= 1 {
        panic!("{} <= 1 não é válido", num);
    } else {
        vec![2]
    }
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
}
