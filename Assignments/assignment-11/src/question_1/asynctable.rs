use log::*;
use async_std::task;
use std::time::Duration;
/// Async Function 'print_table' prints multiplication table of 2 and 3 asynchronously
///
/// #Arguments
///
/// There is no arguments passed here
///
/// #Return
///
/// there is no return type

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
