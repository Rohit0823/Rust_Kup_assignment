#[cfg(test)]
pub mod test {
    use crate::question1::check_even::check_test;

    #[test]
    pub fn test1() {
        assert_eq!(check_test(4), "Even");
    }

    #[test]
    pub fn test2() {
        assert_ne!(check_test(0), "Odd");
    }

    #[test]
    pub fn test3() {
        assert_ne!(check_test(5), "Even");
    }
}
