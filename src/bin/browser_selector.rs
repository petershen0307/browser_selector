#![windows_subsystem = "windows"]

use anyhow::Ok;
use browser_selector::configuration;
use clap::Parser;
use log::debug;
use log::LevelFilter;
use std::cell::RefCell;

const CONFIGURATION_FILE_NAME: &str = "configuration.toml";

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

fn main() -> anyhow::Result<()> {
    let executable_path = std::env::current_exe().unwrap();
    let log_file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(format!(
            "{}\\log.log",
            executable_path.parent().unwrap().display()
        ))
        .unwrap();
    simple_logging::log_to(log_file, LevelFilter::Error);

    debug!("{:?}", std::env::args());

    let args = Args::parse();

    debug!("{:?}", args);
    if args.register {
        browser_selector::register::register();
    }
    if args.unregister {
        browser_selector::register::unregister();
    }
    if args.url.is_some() {
        let config_file = format!(
            "{}\\{}",
            executable_path.parent().unwrap().display(),
            CONFIGURATION_FILE_NAME
        );
        debug!("configuration file path={}", config_file);
        let config_file = std::fs::File::open(config_file);
        let config = configuration::parse(RefCell::new(
            config_file.expect("configuration file should be opened"),
        ))
        .expect("configuration file should be parsed");
        browser_selector::browser::launch_browser(config, args.url.unwrap())?;
    }
    Ok(())
}
