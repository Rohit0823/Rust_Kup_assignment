#[derive(PartialEq, Eq, Debug)]
pub enum Store {
    Cons(i32, Box<Store>),
    Nil,
}