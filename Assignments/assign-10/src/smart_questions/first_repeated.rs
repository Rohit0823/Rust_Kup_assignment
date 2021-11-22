use log::*;
use crate::smart_questions::enum_list::List;
use crate::smart_questions::enum_list::List::{Cons, Nil};
pub fn first_repeated_searching(first_number: i32, list: List) -> i32 {
    error!("Empty Box Provided");
    info!("finds number at previous_one");
    match list {
        Cons(next_number, list) => match first_number == next_number {
            true => next_number,
            false => first_repeated_searching(next_number, *list),
        },
        Nil => 0,
    }
}