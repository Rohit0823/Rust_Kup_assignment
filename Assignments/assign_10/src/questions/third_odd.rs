

use log::*;

use crate::datastore::Store;
use crate::datastore::Store::{Nil, Cons};

/// third_odd function find the third odd number in sequence
///
/// #Arguments
///
/// iterable:It can store the data of numbers in sequence
///
/// #Return
///
/// Return type is int i32 which can store third odd number in the sequence


pub fn third_odd(iterable: Store) -> Result<i32, String> {
    if iterable == Nil {
        error!("It is Empty Box");
        return Err("provide the Input".parse().unwrap());
    }
    let result = find_odd(iterable, 3);
    Ok(result)
}

/// third_odd function find the third odd number in sequence
///
/// #Arguments
///
/// iterable:It can store the data of numbers in sequence
///
/// iterator: It is use  to find the odd number
///
/// #Return
///
/// It can return the Result in the form of T,E variants which can handle the error

pub fn find_odd(iterable: Store, iterator: i32) -> i32 {
    info!("finds the number");
    match iterable {
        Nil => -1,

        Cons(initial, _iterable) if iterator == 1 && &initial & 1 == 1 => initial,

        Cons(initial, iterable) if &initial & 1 == 1 => find_odd(*iterable, iterator - 1),

        Cons(_initial, iterable) => find_odd(*iterable, iterator),
    }
}