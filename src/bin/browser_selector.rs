use browser_selector::configuration;
use clap::Parser;
use std::cell::RefCell;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Register this program to default browser
    #[arg(long, default_value_t = false)]
    register: bool,

    /// Unregister this program to default browser
    #[arg(long, default_value_t = false)]
    unregister: bool,

    /// Url is the raw string
    #[arg(index = 1)]
    url: Option<String>,
}

fn _test_input() {
    let args = Args::parse();
    println!("register: {}!", args.register);
    println!("unregister: {}!", args.unregister);
    println!("url: {}!", args.url.unwrap_or_default());

    let file = std::fs::File::open("configuration.toml");
    let config = configuration::parse(RefCell::new(file.unwrap())).unwrap();
    println!("{:?}", config);
}

fn main() {
    let args = Args::parse();
    if args.register {
        browser_selector::register::register();
    }
    if args.unregister {
        browser_selector::register::unregister();
    }
}
