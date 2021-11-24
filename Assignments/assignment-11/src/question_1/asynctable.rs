use log::*;
/// import the necessary modules
use async_std::task;
use std::time::Duration;
/// print_table: asynchronous function is used to print two tables asynchronously
///
/// #Arguments
///
/// There is no arguments pass here
///
/// #Return
///
/// the return type is tuple of both the tables

pub async fn print_table() {
    use futures::future::join;
    let first_table = async {
        for element in 1..10 {
            let table = 2 * element;
            info!("2*{} = {} ", element, table);
        }
        task::sleep(Duration::from_millis(1)).await;
    };
    let second_table = async {
        for element in 1..10 {
            let table = 3 * element;
            warn!("3*{} = {} ", element, table);
        }
        task::sleep(Duration::from_millis(1)).await;
    };
    join(first_table, second_table).await;
}