use std::thread;
use std::time::Duration;
use regex::Regex;
use once_cell::sync::Lazy;

const WAIT_BETWEEN_MODIFICATIONS_MILLISECONDS: u64 = 100;
const CHUNK_SIZE: usize = 4096; // Process 4KB at a time for better cache performance

static MEMORY_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(\d+)([KMGTP]?B)$").expect("Failed to compile regex")
});

pub fn allocate_memory(memory: &str) -> Result<usize, String> {
    let bytes = parse_memory_string(memory)?;
    
    let mut data = Vec::with_capacity(bytes);
    for i in 0..bytes {
        data.push((i % 256) as u8);
    }
    
    let data = std::sync::Arc::new(std::sync::Mutex::new(data));
    keep_modifying_data(bytes, data);
    
    Ok(bytes)
}

/// This function will keep modifying the data in the vector
/// by adding 1 and then subtracting 1 from each element
/// in the vector. This will keep the memory occupied
/// and make it harder for the OS to move it to file cache or swap.
fn keep_modifying_data(bytes: usize, data: std::sync::Arc<std::sync::Mutex<Vec<u8>>>) {
    let data_clone = std::sync::Arc::clone(&data);

    thread::spawn(move || {
        let data = data_clone;
        loop {
            // Increment all bytes
            {
                let mut data = data.lock().unwrap();
                for chunk_start in (0..bytes).step_by(CHUNK_SIZE) {
                    let chunk_end = (chunk_start + CHUNK_SIZE).min(bytes);
                    for i in chunk_start..chunk_end {
                        data[i] = data[i].wrapping_add(1);
                    }
                }
            }
            
            thread::sleep(Duration::from_millis(WAIT_BETWEEN_MODIFICATIONS_MILLISECONDS));
            
            // Decrement all bytes
            {
                let mut data = data.lock().unwrap();
                for chunk_start in (0..bytes).step_by(CHUNK_SIZE) {
                    let chunk_end = (chunk_start + CHUNK_SIZE).min(bytes);
                    for i in chunk_start..chunk_end {
                        data[i] = data[i].wrapping_sub(1);
                    }
                }
            }
            
            thread::sleep(Duration::from_millis(WAIT_BETWEEN_MODIFICATIONS_MILLISECONDS));
        }
    });
}

fn parse_memory_string(memory_str: &str) -> Result<usize, String> {
    let captures = MEMORY_REGEX
        .captures(memory_str)
        .ok_or_else(|| format!("Invalid memory string format: '{}'. Expected format: <number><unit> (e.g., 1B, 100KB, 2GB)", memory_str))?;

    let value: usize = captures
        .get(1)
        .ok_or("Invalid capture group")?
        .as_str()
        .parse::<usize>()
        .map_err(|e| format!("Failed to parse number: {}", e))?;
    
    let unit = &captures[2];

    let bytes = match unit {
        "B" => value,
        "KB" => value.checked_mul(1024)
            .ok_or("Memory size overflow")?,
        "MB" => value.checked_mul(1024 * 1024)
            .ok_or("Memory size overflow")?,
        "GB" => value.checked_mul(1024 * 1024 * 1024)
            .ok_or("Memory size overflow")?,
        "TB" => value.checked_mul(1024 * 1024 * 1024 * 1024)
            .ok_or("Memory size overflow")?,
        "PB" => value.checked_mul(1024 * 1024 * 1024 * 1024 * 1024)
            .ok_or("Memory size overflow")?,
        _ => return Err(format!("Invalid memory unit: '{}'. Valid units: B, KB, MB, GB, TB, PB", unit)),
    };

    // Sanity check - warn if allocation is very large
    if bytes > 100 * 1024 * 1024 * 1024 {
        eprintln!("Warning: Attempting to allocate {} bytes ({}). This may cause system instability.", bytes, memory_str);
    }

    Ok(bytes)
}