use std::thread;
use std::time::Duration;

pub fn wait(seconds: i32) {
    if seconds == 0 {
        println!("CTRL + C to exit.");
        loop {
            thread::sleep(Duration::from_secs(10));
        }
    } else {
        println!("Waiting for {} seconds before exiting.", seconds);
        thread::sleep(Duration::from_secs(seconds as u64));
        println!("Exiting after waiting for {} seconds.", seconds);
    }
}