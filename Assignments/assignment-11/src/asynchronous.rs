use log::*;
/// import the necessary modules
use std::thread;
use std::time::Duration;
/// return_data: function returns data in an asynchronous manner
///
/// #Arguments
///
/// There is no arguments pass here
///
/// #Return
///
/// the return type is JoinHandle, which can owned the value
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
