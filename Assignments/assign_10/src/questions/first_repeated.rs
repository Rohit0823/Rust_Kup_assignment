use log::*;

use crate::datastore::Store;
use crate::datastore::Store::{Nil, Cons};

/// first_repeat function find the first repeated number in sequence
///
/// #Arguments
///
/// list: It can store the data of number of sequence
///
/// #Return
///
/// It return the Result <T,E>

pub fn first_repeat(list: Store) -> Result<i32, String> {
    if list == Nil {
        error!("It is Empty Box");
        return Err("provide the Input".to_string());
    }
    let result = consecutive(list, -1);
    Ok(result)
}
/// consecutive function can match the number and search first repeated number
///
/// #Arguments
///
/// previous_no: It contain previous number which is int, i32 type in the sequence
///
/// list: It can store the data of number of sequence
///
/// #Return
///
/// the Return type is int i32 

fn consecutive(list: Store, previous_no: i32) -> i32 {
    info!("finds the number");
    match list {
        Nil => -1,

        Cons(initial, _list) if initial == previous_no => initial,

        Cons(initial, list) => consecutive(*list, initial),
    }
}
