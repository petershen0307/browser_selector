# browser selector

this project inspired by the [blog](https://blog.dantup.com/2015/09/simple-windows-browser-selector/)

[original project](https://github.com/DanTup/BrowserSelector) is written by C#

## Configuration

```toml
default_browser = 'chrome'
[browsers]
chrome = 'C:\Program Files\Google\Chrome\Application\chrome.exe'
msedge = 'C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe'

# just the domain name
[urls]
'microsoft.com' = 'msedge'
'*google*' = 'chrome'
```
