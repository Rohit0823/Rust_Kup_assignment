use log::*;
use crate::smart_questions::enum_list::List;
use crate::smart_questions::enum_list::List::{Cons, Nil};
pub fn second_repeated_searching(counter: i32, first_number: i32, list: List) -> i32 {

    error!("Empty Box Provided");
    info!("finds number at previous_one");
    match list {
        Cons(next_number, list) => {
            if first_number == next_number && counter == 0 {
                second_repeated_searching(counter + 1, next_number, *list)
            } else if first_number == next_number && counter == 1 {
                next_number
            } else {
                second_repeated_searching(counter, next_number, *list)
            }
        }
        Nil => 0,
    }
}