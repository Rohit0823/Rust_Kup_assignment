/// even is a function which find the first even number in a data list
///
/// #Arguments
///
/// vect: vector is containing the data which is integer i32 type
///
/// #Return
///
/// Return even number which is integer i32 type
pub fn even(vect: Vec<i32>) -> i32 {
    let mut result: i32 = 0;
    for values in &vect {
        if values % 2 == 0 {
            result = *values;
            break;
        } else {
            continue;
        }
    }
    result
}
