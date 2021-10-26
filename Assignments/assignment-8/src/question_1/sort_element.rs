use std::fmt::Display;
/// Create a generic function to sort the element
///
/// #Arguments
///
/// list: list contains the list of T type
///
/// #Return
///
/// Returns Option<T>
pub fn sort_item<T: std::cmp::PartialOrd + Display>(list: &mut [T]) -> Option<&mut [T]> {
    for index1 in 0..list.len() {
        let mut first = index1;
        for index2 in (index1 + 1)..list.len() {
            if list[index2] < list[first] {
                first = index2;
            }
        }
        list.swap(first, index1);
    }
    Some(list)
}
