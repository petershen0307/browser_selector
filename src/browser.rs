use std::process::Command;
use url::Url;

use crate::configuration::Configuration;

pub fn launch_browser(config: Configuration, url: String) {
    // choose the browser
    let parsed_url = Url::parse(&url).unwrap();
    let browser_path: &String = match config.urls.get_key_value(parsed_url.host_str().unwrap()) {
        Some((_, v)) => config.browsers.get(v).unwrap(),
        None => &config.default_browser,
    };
    let _ = Command::new(browser_path).arg(url).status();
}

#[ignore]
#[test]
fn test_launch_default() {
    use std::collections::HashMap;
    let config = Configuration {
        default_browser: String::from(r"C:\Program Files\Google\Chrome\Application\chrome.exe"),
        browsers: HashMap::new(),
        urls: HashMap::new(),
    };
    launch_browser(config, "https://google.com".to_string());
}

#[ignore]
#[test]
fn test_launch_with_rule() {
    use std::collections::HashMap;
    let chrome = r"C:\Program Files\Google\Chrome\Application\chrome.exe";
    let msedge = r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe";
    let config = Configuration {
        default_browser: chrome.to_string(),
        browsers: HashMap::from([
            (String::from("chrome"), chrome.to_string()),
            (String::from("msedge"), msedge.to_string()),
        ]),
        urls: HashMap::from([(String::from("www.google.com"), "msedge".to_string())]),
    };
    launch_browser(
        config,
        r"https://www.google.com/search?q=rust+url+parse".to_string(),
    );
}