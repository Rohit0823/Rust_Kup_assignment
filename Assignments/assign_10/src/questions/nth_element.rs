use log::*;
use crate::datastore::Store;
use crate::datastore::Store::{Nil, Cons};

/// find_nth function finds the nth element from the List
///
/// #Arguments
///
/// List: It can store the data of number of sequence
///
/// position: It can store the find value of position
///
/// #Return
///
/// It can return the Result in the form of T,E variants which can handle the error

pub fn find_nth(position: i32, list: Store) -> Result<i32, String> {
    if list == Nil {
        error!("It is Empty Box");
        return Err("provide the Input".to_string());
    }
    let result = recursion(position - 1, list, 0);
    Ok(result)
}
/// find_nth function finds the nth element from the List
///
/// #Arguments
///
/// List: It can store the data of numbers
///
/// position: It can store the find value of position
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
