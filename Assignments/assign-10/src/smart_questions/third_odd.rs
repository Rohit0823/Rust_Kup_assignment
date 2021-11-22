use log::*;
use crate::smart_questions::enum_list::List;
use crate::smart_questions::enum_list::List::{Cons, Nil};
pub fn third_odd_searching(index_travel: i32, list: List) -> i32 {

    error!("Empty Box Provided");
    info!("finds number at previous_one");
    match list {
        Cons(number, list) => {
            let value = number % 2 != 0;
            match value {
                true => {
                    let flag = index_travel == 1 || index_travel == 0;
                    match flag {
                        true => third_odd_searching(index_travel + 1, *list),
                        false => number,
                    }
                }
                false => third_odd_searching(index_travel, *list),
            }
        }
        Nil => 0,
    }
}