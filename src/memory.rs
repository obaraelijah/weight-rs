use std::thread;
use std::time::Duration;

use regex::Regex;

const WAIT_BETWEEN_MODIFICATIONS_MILLISECONDS: u64 = 100;

pub fn allocate_memory(memory: &str) -> Result<usize, String> {
    match parse_memory_string(memory) {
        Ok(bytes) => {
            let mut data = Vec::with_capacity(bytes);
            for i in 0..bytes {
                data.push((i % 10) as u8);
            }
            let data = std::sync::Arc::new(std::sync::Mutex::new(data));
            keep_modifying_data(bytes, data);

            Ok(bytes)
        }
        Err(err) => Err(format!("Error parsing memory string: {}", err)),
    }
}

// This function will keep modifying the data in the vector
// by adding 1 and then subtracting 1 from each element
// in the vector. This will keep the memory occupied
// and make it easier to see the memory usage in the system
// by making it harder for the OS to move it to file cache or swap.
fn keep_modifying_data(bytes: usize, data: std::sync::Arc<std::sync::Mutex<Vec<u8>>>) {
    let data_clone = std::sync::Arc::clone(&data);

    thread::spawn(move || {
        let data = data_clone;
        loop {
            let mut data = data.lock().unwrap();
            for i in 0..bytes {
                data[i] = data[i] + 1;
                // sleep to reduce CPU usage
                thread::sleep(Duration::from_millis(WAIT_BETWEEN_MODIFICATIONS_MILLISECONDS));
                data[i] = data[i] - 1;
            }
        }
    });
}

fn parse_memory_string(memory_str: &str) -> Result<usize, String> {
    let re = Regex::new(r"(\d+)([KMGTP]?B)").map_err(|e: regex::Error| e.to_string())?;
    let captures = re.captures(memory_str).ok_or("Invalid memory string format")?;

    let value: usize = captures.get(1).ok_or("Invalid capture group")?.as_str().parse::<usize>().map_err(|e| e.to_string())?;
    let unit: &str = &captures[2];

    match unit {
        "B" => Ok(value),
        "KB" => Ok(value * 1024),
        "MB" => Ok(value * 1024 * 1024),
        "GB" => Ok(value * 1024 * 1024 * 1024),
        "TB" => Ok(value * 1024 * 1024 * 1024 * 1024),
        "PB" => Ok(value * 1024 * 1024 * 1024 * 1024 * 1024),
        _ => Err("Invalid memory unit".to_string()),
    }
}