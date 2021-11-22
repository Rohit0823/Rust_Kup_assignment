#[cfg(test)]
mod tests {
    use crate::datastore::Store::{Nil, Cons};
    use crate::questions::first_repeated::first_repeat;
    #[test]
    fn first_repeat_check() {
        let list_val = Cons(
            1,
            Box::new(Cons(
                23,
                Box::new(Cons(
                    23,
                    Box::new(Cons(
                        6,
                        Box::new(Cons(5, Box::new(Cons(5, Box::new(Nil))))),
                    )),
                )),
            )),
        );
        assert_eq!(first_repeat(list_val), Ok(23));
    }

    #[test]
    fn first_repeat_check_next_times() {
        let list_val = Cons(
            1,
            Box::new(Cons(
                23,
                Box::new(Cons(
                    1,
                    Box::new(Cons(
                        6,
                        Box::new(Cons(5, Box::new(Cons(7, Box::new(Nil))))),
                    )),
                )),
            )),
        );
        assert_eq!(first_repeat(list_val), Ok(-1));
    }
    use crate::questions::second_repeated::second_repeat;
    #[test]
    fn second_repeat_check() {
        let box_array = Cons(
            1,
            Box::new(Cons(
                23,
                Box::new(Cons(
                    23,
                    Box::new(Cons(
                        6,
                        Box::new(Cons(7, Box::new(Cons(7, Box::new(Nil))))),
                    )),
                )),
            )),
        );
        assert_eq!(second_repeat(box_array), Ok(7));
    }

    #[test]
    fn second_repeat_check_next_time() {
        let box_array = Cons(
            1,
            Box::new(Cons(
                23,
                Box::new(Cons(
                    23,
                    Box::new(Cons(
                        6,
                        Box::new(Cons(7, Box::new(Cons(9, Box::new(Nil))))),
                    )),
                )),
            )),
        );
        assert_eq!(second_repeat(box_array), Ok(-1));
    }
    use crate::questions::nth_element::find_nth;
    #[test]
    fn element_check() {
        let test_list = Cons(
            1,
            Box::new(Cons(
                2,
                Box::new(Cons(
                    3,
                    Box::new(Cons(
                        4,
                        Box::new(Cons(5, Box::new(Cons(6, Box::new(Nil))))),
                    )),
                )),
            )),
        );
        assert_eq!(find_nth(4, test_list), Ok(4));
    }

    #[test]
    fn element_check_next_time() {
        let test_list = Cons(
            1,
            Box::new(Cons(
                2,
                Box::new(Cons(
                    3,
                    Box::new(Cons(
                        4,
                        Box::new(Cons(5, Box::new(Cons(6, Box::new(Nil))))),
                    )),
                )),
            )),
        );
        assert_eq!(find_nth(7, test_list), Ok(-1));
    }
    use crate::questions::third_odd::third_odd;
    #[test]
    fn third_odd_check() {
        let test_data = Cons(
            1,
            Box::new(Cons(
                23,
                Box::new(Cons(
                    3,
                    Box::new(Cons(
                        6,
                        Box::new(Cons(5, Box::new(Cons(6, Box::new(Nil))))),
                    )),
                )),
            )),
        );
        assert_eq!(third_odd(test_data), Ok(3));
    }

    #[test]
    fn third_odd_check_next_time() {
        let test_data = Cons(
            1,
            Box::new(Cons(
                2,
                Box::new(Cons(
                    3,
                    Box::new(Cons(
                        4,
                        Box::new(Cons(8, Box::new(Cons(6, Box::new(Nil))))),
                    )),
                )),
            )),
        );
        assert_eq!(third_odd(test_data), Ok(-1));
    }
}