use log::*;
use std::thread;
use std::time::Duration;
/// Function 'return_data' prints data in asynchronous fashion
///
/// #Arguments
///
/// There is no arguments passed here
///
/// #Return
///
/// there is no return type
pub fn return_data() {
    let element = thread::spawn(|| {
        for data in 1..10 {
            info!("{}", data);
        }
        thread::sleep(Duration::from_millis(1));
    });
    for data in 1..10 {
        warn!("{}", data);
    }
    thread::sleep(Duration::from_millis(1));
    element.join().unwrap()
}
