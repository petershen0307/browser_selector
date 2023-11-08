use clap::Parser;

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

fn main() {
    let args = Args::parse();

    println!("register: {}!", args.register);
    println!("unregister: {}!", args.unregister);
    println!("url: {}!", args.url.unwrap_or_default());
}
