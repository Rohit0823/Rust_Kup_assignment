use log::*;
use crate::smart_questions::enum_list::List;
use crate::smart_questions::enum_list::List::{Cons, Nil};
pub fn nth_element_finder(index: i32, result: i32, list: List) -> i32 {

    error!("Empty Box Provided");
    info!("finds number at previous_one");
    match list {
        Cons(number, list) => {
            if index == result {
                number
            } else {
                nth_element_finder(index, result + 1, *list)
            }
        }
        Nil => 0,
    }
}