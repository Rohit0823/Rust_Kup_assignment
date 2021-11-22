use log::*;
use crate::datastore::Store;
use crate::datastore::Store::{Nil, Cons};

/// find_nth function finds the nth element from the List
///
/// #Arguments
///
/// list: It can store the data of number of sequence
///
/// position: It can store the find value of position
///
/// #Return
///
/// It return the Result<i32, String>

pub fn find_nth(position: i32, list: Store) -> Result<i32, String> {
    if list == Nil {
        error!("It is Empty Box");
        return Err("provide the Input".to_string());
    }
    let result = recursion(position - 1, list, 0);
    Ok(result)
}
/// recursion function finds the nth element from the List
///
/// #Arguments
///
/// list: It can store the data of numbers
///
/// position: It can store the find value of position
///
/// counter: It can used to update the list
///
/// #Return
///
/// the Return type is int i32 which can store the number

pub fn recursion(position: i32, list: Store, counter: i32) -> i32 {
    info!("finds the number");
    match list {
        Nil => -1,
        Cons(current, _) if counter == position => current,
        Cons(_, list) => recursion(position, *list, counter + 1),
    }
}
