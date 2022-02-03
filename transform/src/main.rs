use std::process;

fn main() {
    if let Err(err) = vtex_transform::run() {
        println!("{}", err);
        process::exit(1);
    }
}
