/// create generic function to find minimum_value
///
/// #Arguments
///
/// num1: It is a T type object which hold Primitive value
/// num1: It is a T type object which hold Primitive value
///
/// #Return
///
/// Return Result<T,String>
pub fn min_number<T: std::cmp::PartialOrd>(num1: T, num2: T) -> Result<T, String> {
    if num1 == num2 {
        Err("sample input".to_string())
    } else if num1 < num2 {
        Ok(num1)
    } else {
        Ok(num2)
    }
}
