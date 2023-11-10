use crate::{Error, Result};
use serde_derive::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub default_browser: String,
    pub browsers: HashMap<String, String>,
    pub urls: HashMap<String, String>,
}

pub fn parse(config_reader: RefCell<impl io::Read>) -> Result<Configuration> {
    let mut contents: String = String::new();
    match config_reader.borrow_mut().read_to_string(&mut contents) {
        Ok(size) => {
            if size == 0 {
                return Err(Error::from("read size is 0 from configuration"));
            }
        }
        Err(e) => return Err(e.into()),
    }
    match toml::from_str(&contents) {
        Ok(c) => Ok(c),
        Err(e) => Err(e.into()),
    }
}

#[test]
fn test_deserialize() {
    let config: Configuration = toml::from_str(
        r"
    default_browser = 'chrome'
    [browsers]
    chrome = 'C:\Program Files (x86)\Google\Chrome\Application\chrome.exe'
    [urls]
    'google.com' = 'chrome'
",
    )
    .unwrap();
    assert_eq!(String::from("chrome"), config.default_browser);
    assert_eq!(
        String::from(r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe"),
        config.browsers["chrome"]
    );
    assert_eq!(String::from("chrome"), config.urls["google.com"]);
}

#[test]
fn test_parse() {
    let buf_reader = io::Cursor::new(String::from(
        r"
    default_browser = 'chrome'
    [browsers]
    chrome = 'C:\Program Files (x86)\Google\Chrome\Application\chrome.exe'
    [urls]
    'google.com' = 'chrome'
",
    ));
    let r = parse(RefCell::new(buf_reader));
    let config = r.unwrap();
    assert_eq!(String::from("chrome"), config.default_browser);
    assert_eq!(
        String::from(r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe"),
        config.browsers["chrome"]
    );
    assert_eq!(String::from("chrome"), config.urls["google.com"]);
}
