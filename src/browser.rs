use log::{debug, info};
use std::process::Command;
use url::Url;
use urlencoding::decode as url_decode;

use crate::configuration::Configuration;

pub fn launch_browser(config: Configuration, url: String) -> anyhow::Result<()> {
    // choose the browser
    info!("url: {}", url);
    let mut parsed_url = Url::parse(&url)?;
    let mut a = parsed_url.query_pairs().filter(|x| &*x.0 == "url");
    if let Some((_, url)) = a.next() {
        let url = url_decode(&url)?;
        parsed_url = Url::parse(&url)?;
    }
    let url_pattern: Vec<(&String, &String)> = config
        .urls
        .iter()
        .filter(|(key, _)| -> bool {
            wildmatch::WildMatch::new(key.as_str()).matches(parsed_url.host_str().unwrap())
        })
        .collect();

    let browser_path: &String = if url_pattern.is_empty() {
        config.browsers.get(&config.default_browser).unwrap()
    } else {
        config.browsers.get(url_pattern.first().unwrap().1).unwrap()
    };
    debug!("browser path: {}", browser_path);
    Command::new(browser_path)
        .arg(parsed_url.to_string())
        .status()?;
    Ok(())
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
    assert!(launch_browser(config, "https://google.com".to_string()).is_ok());
}

#[ignore]
#[test]
fn test_launch_with_rule() {
    use std::collections::HashMap;
    let chrome = r"C:\Program Files\Google\Chrome\Application\chrome.exe";
    let msedge = r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe";
    let config = Configuration {
        default_browser: "chrome".to_string(),
        browsers: HashMap::from([
            (String::from("chrome"), chrome.to_string()),
            (String::from("msedge"), msedge.to_string()),
        ]),
        urls: HashMap::from([
            (String::from("google.com"), "msedge".to_string()),
            ("microsoft.com".to_string(), "msedge".to_string()),
        ]),
    };
    assert!(launch_browser(
        config,
        r"https://www.google.com/search?q=rust+url+parse".to_string(),
    )
    .is_ok());
}

#[ignore]
#[test]
fn test_launch_skip_ms_safe_link() {
    use std::collections::HashMap;
    let chrome = r"C:\Program Files\Google\Chrome\Application\chrome.exe";
    let msedge = r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe";
    let config = Configuration {
        default_browser: "chrome".to_string(),
        browsers: HashMap::from([
            (String::from("chrome"), chrome.to_string()),
            (String::from("msedge"), msedge.to_string()),
        ]),
        urls: HashMap::from([
            (String::from("google.com"), "msedge".to_string()),
            ("microsoft.com".to_string(), "msedge".to_string()),
        ]),
    };
    assert!(launch_browser(
        config,
        r"https://statics.teams.cdn.office.net/evergreen-assets/safelinks/1/atp-safelinks.html?url=https%3A%2F%2Fwww.google.com%2Fsearch%3Fq%3Drust%2Burl%2Bparse".to_string(),
    )
    .is_ok());
}
