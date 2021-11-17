#[cfg(test)]
pub use crate::question_1::hash_map::sum_conditional;
pub use crate::question_2::palindrome_check::check_palindrome;
pub use crate::question_2::reverse_check::reverse_list;
pub use crate::question_2::even_check::even;
pub use crate::question_2::remove_check::remove_no;
pub use crate::question_2::duplicate_check::duplicate_no;
pub use crate::question_2::remove_nth::remove_element;
#[test]
pub fn sum_conditional_ok() {
    use std::collections::HashMap;
    let mut names_age = HashMap::new();
    names_age.insert(String::from("anurag"), 24);
    names_age.insert(String::from("daniel"), 23);
    names_age.insert(String::from("anushka"), 30);

    assert_eq!(sum_conditional(names_age, String::from("anu")), 54);
}
#[test]
pub fn sum_conditional_success() {
    use std::collections::HashMap;
    let mut names_age = HashMap::new();
    names_age.insert(String::from("mohan"), 24);
    names_age.insert(String::from("mohani"), 23);
    names_age.insert(String::from("rohan"), 30);

    assert_eq!(sum_conditional(names_age, String::from("moh")), 47);
}
#[test]
pub fn check_palindrome_ok() {
    assert_eq!(check_palindrome(vec![2, 3, 2]), true);
}
#[test]
pub fn check_palindrome_success() {
    assert_eq!(check_palindrome(vec![1, 2, 5,6,7]), false);
}


#[test]
pub fn reverse_ok() {
    assert_eq!(reverse_list(vec![1, 2,]), vec![2, 1]);
}
#[test]
pub fn reverse_success() {
    assert_eq!(reverse_list(vec![3, 4, 5]), vec![5, 4, 3]);
}


#[test]
pub fn even_ok() {
    assert_eq!(even(vec![1, 6, 3, 4, 5]), 6);
}
#[test]
pub fn even_success() {
    assert_eq!(even(vec![1, 7, 4, 2, 1]), 4);
}


#[test]
pub fn remove_ok() {
    assert_eq!(
        remove_no(vec![1, 1, 1, 1, 2, 2, 1, 4, 4, 5, 1]),
        vec![1, 2, 1, 4, 5, 1]
    );
}
#[test]
pub fn remove_success() {
    assert_eq!(
        remove_no(vec![1, 1, 1, 1, 2, 3, 3, 1, 1, 4, 5]),
        vec![1, 2, 3, 1, 4, 5]
    );
}


#[test]
pub fn add_duplicate_ok() {
    assert_eq!(
        duplicate_no(vec![1, 2, 3, 4]),
        vec![1, 1, 2, 2, 3, 3, 4, 4]
    );
}
#[test]
pub fn add_duplicate_success() {
    assert_eq!(
        duplicate_no(vec![2, 1, 5, ]),
        vec![2, 2, 1, 1, 5, 5,]
    );
}


#[test]
pub fn remove_element_ok() {
    assert_eq!(remove_element(vec![1, 2, 3, 3, 3], 3), vec![1, 2]);
}

#[test]
pub fn remove_element_success() {
    assert_eq!(remove_element(vec![7, 7, 1, 2, 5,], 9), vec![1, 2, 5]);
}

