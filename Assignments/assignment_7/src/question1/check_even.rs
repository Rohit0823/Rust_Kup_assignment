/// number function check the number is "Even".
///
/// #Arguments
///
/// value : taking i32 as input
///
/// #Return
///
/// Return Result enum which is used to handle error and value
pub fn is_even(value: i32) -> Result<String, String> {
    if value % 2 == 0 {
        Ok("Even".to_string())
    } else {
        Err("Not even".to_string())
    }
}
/// check_test function is used to handle the response of a function
///
/// #Arguments
///
/// value is a integer of i32 type Whose check even or not
///
/// #Return
///
/// Return String which handle error and value of output of function.
pub fn check_test(value: i32) -> String {
    let result = is_even(value);

    match result {
        Ok(result) => result,

        Err(_) => "Invalid".to_string(),
    }
}
