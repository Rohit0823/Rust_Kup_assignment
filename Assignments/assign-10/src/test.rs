#[cfg(test)]
use crate::smart_questions::enum_list::List::{Cons, Nil};
use crate::smart_questions::first_repeated::first_repeated_searching;
use crate::smart_questions::second_repeated::second_repeated_searching;
use crate::smart_questions::third_odd::third_odd_searching;
use crate::smart_questions::nth_element::nth_element_finder;
#[test]
fn first_repeated_searching_success() {


    let input = Cons(
        1,
        Box::new(Cons(
            21,
            Box::new(Cons(
                21,
                Box::new(Cons(4, Box::new(Cons(5, Box::new(Cons(5, Box::new(Nil))))))),
            )),
        )),
    );
    assert_eq!(first_repeated_searching(0, input), 21);
}
fn first_repeated_searching_next() {


    let input = Cons(
        1,
        Box::new(Cons(
            2,
            Box::new(Cons(
                21,
                Box::new(Cons(4, Box::new(Cons(5, Box::new(Cons(5, Box::new(Nil))))))),
            )),
        )),
    );
    assert_eq!(first_repeated_searching(0, input), 21);
}
#[test]
fn second_repeated_searching_success() {


    let input = Cons(
        2,
        Box::new(Cons(
            3,
            Box::new(Cons(
                5,
                Box::new(Cons(
                    5,
                    Box::new(Cons(
                        2,
                        Box::new(Cons(7, Box::new(Cons(7, Box::new(Cons(8, Box::new(Nil))))))),
                    )),
                )),
            )),
        )),
    );
    assert_eq!(second_repeated_searching(0, 0, input), 7);

}
#[test]
fn third_odd_searching_success() {


    let input = Cons(
        1,
        Box::new(Cons(
            21,
            Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))),
        )),
    );
    assert_eq!(third_odd_searching(0, input), 3);

}
#[test]
fn nth_number_finder_success() {


    let input = Cons(
        1,
        Box::new(Cons(
            2,
            Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))),
        )),
    );
    assert_eq!(nth_element_finder(3, 0, input), 4);

}