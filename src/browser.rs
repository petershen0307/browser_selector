use std::process::Command;

use crate::configuration::Configuration;

pub fn launch_browser(config: Configuration, url: String) {
    // choose the browser
    let browser_path: &String = match config.urls.get_key_value(&url) {
        Some((_, v)) => v,
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
        urls: HashMap::from([(String::from("https://google.com"), msedge.to_string())]),
    };
    launch_browser(config, "https://google.com".to_string());
}
