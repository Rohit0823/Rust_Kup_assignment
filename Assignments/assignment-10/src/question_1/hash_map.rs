pub use std::collections::HashMap;
/// sum_conditional is a function which returns the sum of ages for which the name contains the str
///
/// #Arguments
///
/// Map: It is a Hashmap which contains data in the pair of Key and value
///
/// #Return
///
/// Return integer type <String,i32>
pub fn sum_conditional(map: HashMap<String, i32>, str: String) -> i32 {
    let mut result = 0;
    let length = str.len();
    for (key, value) in &map {
        if key[0..length] == str {
            result += value;
        }
    }
    result
}
