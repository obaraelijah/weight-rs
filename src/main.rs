use clap::Parser;

mod memory;
mod wait;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Amount of memory to occupy in string format e.g. 1B, 1KB, 1MB, 1GB, 1TB, 1PB
    #[arg(short, long, default_value_t = String::from("1B"))]
    memory: String,

    /// Amount of time in seconds wait before exiting the program. 0 means wait forever
    #[arg(short, long, default_value_t = 60)]
    wait: u32,
}

fn main() {
    let args = Args::parse();
    allocate_memory(&args.memory);
    wait::wait(args.wait);
}

fn allocate_memory(memory: &str) {
    println!("Writing {} of memory...", memory);
    match memory::allocate_memory(memory) {
        Ok(bytes) => {
            println!("Continuously modifying {} = {}B of memory.", memory, bytes);
        }
        Err(err) => {
            eprintln!("Error parsing memory string: {} use -h to see examples", err);
            std::process::exit(1);
        }
    }
}