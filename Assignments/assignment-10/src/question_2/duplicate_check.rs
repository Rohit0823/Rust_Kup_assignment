/// duplicate_no is a function which can add duplicates of the elements in the list
///
/// #Arguments
///
/// vector is containing the data which is integer i32 type
///
/// #Return
///
/// new vector return which is Vec<i32> type
pub fn duplicate_no(vect: Vec<i32>) -> Vec<i32> {
    let mut vec_2 = Vec::new();
    for values in &vect {
        for _loop1 in 0..2 {
            vec_2.push(*values);
        }
    }
    vec_2
}
