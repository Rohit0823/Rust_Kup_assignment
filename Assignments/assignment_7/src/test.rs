#[cfg(test)]
pub mod test {
    use crate::question1::check_even::check_test;

    #[test]
    fn check_number_success_first() {
        assert_eq!(check_test(18), "Even");
    }
    #[test]
    fn check_number_success_second() {
        assert_eq!(check_test(-24), "Even");
    }
    #[test]
    fn check_number_failure_first() {
        assert_eq!(check_test(13), "Invalid");
    }
    #[test]
    fn check_number_failure_second() {
        assert_eq!(check_test(17), "Invalid");
    }
    #[test]
    fn check_number_success_third() {
        assert_eq!(check_test(0), "Even");
    }
    #[test]
    fn check_number_failure_third() {
        assert_eq!(check_test(-45), "Invalid");
    }
}
