use std::process;

fn main() {
    if let Err(err) = transform::run() {
        println!("{}", err);
        process::exit(1);
    }
}
