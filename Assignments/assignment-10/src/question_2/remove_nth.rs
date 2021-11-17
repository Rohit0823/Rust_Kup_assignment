/// remove_element is a function remove nth element from a list
///
/// #Arguments
///
/// vector is containing the data which is integer i32 type
///
/// #Return
///
/// Returns new vector which is Vec<i32> type with removing the nth element
pub fn remove_element(vect: Vec<i32>, drop_value: i32) -> Vec<i32> {
    let mut vec_2 = Vec::new();
    for values in &vect {
        if drop_value != *values {
            vec_2.push(*values);
        }
        continue;
    }
    vec_2
}
