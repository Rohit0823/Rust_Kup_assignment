#[derive(PartialEq, Eq, Debug)]

/// Store: It is enum data type
///
/// #Varients
///
/// Cons: It is integer i32 type
///
/// Nil: It is end element of the list

pub enum Store {
    Cons(i32, Box<Store>),
    Nil,
}
