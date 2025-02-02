use std::process;

fn main() {
    if let Err(e) = todo::run() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
