use console::style;

fn main() {
    if let Err(error) = universal2::run() {
        println!("{} {}", style("error:").bold().red(), error);
        std::process::exit(1);
    }
}
