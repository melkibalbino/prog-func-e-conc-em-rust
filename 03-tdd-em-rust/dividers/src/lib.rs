fn divider(num: i32) {

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn zero_is_not_valid() {
        divider(0)
    }
}
