/// check_palindrome is a function that check the vector is palindrome or not
///
/// #Arguments
///
/// vect: vector containing the data which is i32 type
///
/// #Return
///
/// Return type is boolean which give us true or false (if palindrome then true ,if not palindrome then false)
pub fn check_palindrome(vect: Vec<i32>) -> bool {
    let mut vect_1 = Vec::new();
    let length = vect.len() - 1;
    for loop1 in 0..=length {
        if vect[loop1] == vect[length - loop1] {
            vect_1.push(vect[loop1]);
        }
    }
    vect == vect_1
}
