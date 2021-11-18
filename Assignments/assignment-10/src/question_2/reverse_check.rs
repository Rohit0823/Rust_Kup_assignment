/// reverse_list is a function which can reverse the list
///
/// #Arguments
///
/// vect: vector is containing the data which is integer i32 type
///
/// #Return
///
/// Returns new vector which is Vec<i32> type and it gives reverse list
pub fn reverse_list(vect: Vec<i32>) -> Vec<i32> {
    let mut vect_2 = Vec::new();
    let length = vect.len() - 1;
    for loop1 in 0..=length {
        vect_2.push(vect[length - loop1]);
    }
    vect_2
}
