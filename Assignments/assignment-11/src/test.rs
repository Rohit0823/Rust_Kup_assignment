#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use crate::asynchronous::return_data;
    use crate::question_1::asynctable::print_table;


    #[test]
    fn tables_check() {
        assert_eq!(block_on(print_table()), ());
    }
    #[test]
    fn asynchronously_check() {
        assert_eq!(return_data(), ());
    }
}


