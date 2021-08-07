#[allow(dead_code)]
fn divider(num: i64) {
    panic!("{} <= 0 não é válido", num)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn zero_is_not_valid() {
        divider(0)
    }

    #[test]
    #[should_panic]
    fn negatives_are_not_valid() {
        divider(-10)
    }
}
