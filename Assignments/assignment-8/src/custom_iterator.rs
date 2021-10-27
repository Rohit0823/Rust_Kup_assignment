pub trait Iterator {
    fn next(&mut self) -> Option<i32>;
    fn take(&mut self, size: i32) -> Result<Vec<i32>, String>;
}
/// GeometricSeries structure which used for encapsulation
///
/// #field
///
/// first_number: It is consider first term that is i32 type
///
/// current_number: It is i32 type integer which consider current_term
///
/// ratio:a ration is integer value that is i32 type
pub struct GeometricSeries {
    pub first_number: i32,
    pub current_number: i32,
    pub ratio: i32,
}

impl Iterator for GeometricSeries {
    /// next is used to solve the precedence
    ///
    /// #Arguments
    ///
    /// self: self means calling method through structure parameter
    ///
    /// #Return
    ///
    /// Returns Option<i32> is a enum type and it can the handle error
    fn next(&mut self) -> Option<i32> {
        let result: Option<i32> = Some(self.current_number);
        self.current_number *= self.ratio;
        match result {
            Some(result) => Option::from(result),
            None => panic!("put wrong number"),
        }
    }
    /// take is used to get next(11) with the help of count
    ///
    /// #Arguments
    ///
    /// count: count is an integer which is used to next preceding term
    ///
    /// #Return
    ///
    /// Returns Result<Vec<i32>,String and handle the Error
    fn take(&mut self, next_to: i32) -> Result<Vec<i32>, String> {
        if next_to == 0 {
            return Err("sample input".to_string());
        }
        let mut value: Vec<i32> = Vec::new();
        for _ in 0..next_to {
            value.push(self.next().unwrap());
        }
        Ok(value)
    }
}
