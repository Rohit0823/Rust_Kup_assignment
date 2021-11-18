/// remove_no is a function which can remove the continuously occurring duplicates from a Lis
///
/// #Arguments
///
/// vect:vector is containing the data which is integer i32 type
///
/// #Return
///
/// Returns new vector which is Vec<i32> type
pub fn remove_no(vect: Vec<i32>) -> Vec<i32> {
    let mut vec_2 = Vec::new();
    let length = vect.len();
    vec_2.push(vect[0]);
    for loop1 in 1..length {
        if vect[loop1] != vect[loop1 - 1] {
            vec_2.push(vect[loop1]);
        }
    }
    vec_2
}
