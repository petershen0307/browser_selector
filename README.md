# browser selector

this project inspired by the [blog](https://blog.dantup.com/2015/09/simple-windows-browser-selector/)

[original project](https://github.com/DanTup/BrowserSelector) is written by C#

## Configuration

```toml
[browsers]
chrome = 'C:\Program Files (x86)\Google\Chrome\Application\chrome.exe'
edge = 'C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe'

# Url preferences.
# Only * is treated as a special character (wildcard).
# Matches are domain-only. Protocols and paths are ignored.
# Use "*.blah.com" for subdomains, not "*blah.com" as that would also match "abcblah.com".
[urls]
microsoft.com = 'edge'
*.microsoft.com = 'edge'
```
