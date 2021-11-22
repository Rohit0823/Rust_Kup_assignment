use log::*;

use crate::datastore::Store;
use crate::datastore::Store::{Nil, Cons};

/// second_repeat function find the second repeated number in sequence
///
/// #Arguments
///
/// list: It can store the data of number of sequence
///
/// #Return
///
/// It return the Result<i32, String>




pub fn second_repeat(list: Store) -> Result<i32, String> {
    if list == Nil {
        error!("It is Empty Box");
        return Err("provide the Input".to_string());
    }
    let result = recursion(-1, list, 0);
    Ok(result)
}

/// recursion function use recursion to match list object and find second repeated number.
///
/// #Arguments
///
/// previous: It contain previous number which is int, i32 type in the sequence
///
/// list: It can store the data of number of sequence
///
/// occurrence: It is used to search the second number
///
/// #Return
///
/// Return type is int i32
pub fn recursion(previous: i32, list: Store, occurrence: i32) -> i32 {
    info!("finds the number");
    match list {
        Nil => -1,
        Cons(initial, _) if initial == previous && occurrence == 1 => initial,
        Cons(initial, list) if initial == previous => {
            recursion(initial, *list, occurrence + 1)
        }
        Cons(initial, list) => recursion(initial, *list, occurrence),
    }
}
