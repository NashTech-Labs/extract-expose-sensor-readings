
use std::str;
use std::sync::mpsc::channel;
use std::time::Duration;

use ::log::{error, info};
use notify::{watcher, RecursiveMode, Watcher};
use read_file_data::utils::file_operations::get_last_line;


static SERVER_URL: &str = "https://connect.cloud.kaaiot.com:443/kp1/c4hnc5ad4ks1slmoohgg-v1/epmx/magnetometer/update/keys";
static FILE_LOCATION: &str = "/tmp/itm.txt";

/// This project pertains continuously to reading data from the file and posting the last updated
/// line into the server.
#[cfg(not(tarpaulin_include))]
fn main() {
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher =
        watcher(tx, Duration::from_secs(1)).expect("Unable create a watcher object");

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch(FILE_LOCATION, RecursiveMode::Recursive)
        .expect("Unable to set monitoring on the file");

    loop {
        match rx.recv() {
            Ok(_event) => match get_last_line(FILE_LOCATION) {
                Ok(reading) => info!("response: {}", post_request(reading.as_bytes(), SERVER_URL)),
                Err(error) => error!("{}", error),
            },
            Err(error) => println!("watch error: {:?}", error),
        }
    }
}
