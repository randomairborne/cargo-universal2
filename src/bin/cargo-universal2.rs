use console::style;
use std::process;

fn main() {
    if let Err(error) = universal2::run() {
        println!("{} {}", style("error:").bold().red(), error);
        process::exit(1);
    }
}
