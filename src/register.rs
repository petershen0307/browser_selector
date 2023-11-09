use winreg::enums;
use winreg::reg_key::RegKey;

const APP_ID: &str = "ps_browser_selector";
const APP_NAME: &str = "ps_browser_selector";
const APP_DESCRIPTION: &str = "browser selector";

/// expected register location
/// - Computer\HKEY_LOCAL_MACHINE\SOFTWARE\ps_browser_selector
/// - Computer\HKEY_LOCAL_MACHINE\SOFTWARE\RegisteredApplications
///     - ps_browser_selector
/// - Computer\HKEY_LOCAL_MACHINE\SOFTWARE\Classes\ps_browser_selectorURL
///     - Computer\HKEY_LOCAL_MACHINE\SOFTWARE\Classes\ps_browser_selectorURL\shell\open\command
pub fn register() {
    // Register application.
    let hklm = RegKey::predef(enums::HKEY_LOCAL_MACHINE);
    let app_reg: RegKey = hklm
        .create_subkey(format!("SOFTWARE\\{}", APP_ID))
        .unwrap()
        .0;

    // Register capabilities.
    let capability_reg = app_reg.create_subkey("Capabilities").unwrap().0;
    let _ = capability_reg.set_value("ApplicationName", &APP_NAME);
    let _ = capability_reg.set_value("ApplicationDescription", &APP_DESCRIPTION);

    // Set up protocols we want to handle.
    let url_associate_reg = capability_reg.create_subkey("URLAssociations").unwrap().0;
    let _ = url_associate_reg.set_value("http", &format!("{}URL", APP_ID));
    let _ = url_associate_reg.set_value("https", &format!("{}URL", APP_ID));
    // let _ = url_associate_reg.set_value("ftp", &format!("{}URL", APP_ID));

    // Register as application.
    let _ = hklm
        .open_subkey_with_flags(
            "SOFTWARE\\RegisteredApplications",
            enums::KEY_READ | enums::KEY_WRITE,
        )
        .unwrap()
        .set_value(APP_ID, &format!("SOFTWARE\\{0}\\Capabilities", APP_ID));

    // Set URL Handler.
    let handler_reg = hklm
        .create_subkey(format!("SOFTWARE\\Classes\\{}URL", APP_ID))
        .unwrap()
        .0;
    let _ = handler_reg.set_value("", &APP_NAME);
    let _ = handler_reg.set_value("FriendlyTypeName", &APP_NAME);
    let app_opne_url_command = format!("{} %1", std::env::current_exe().unwrap().to_str().unwrap());
    let _ = handler_reg
        .create_subkey("shell\\open\\command")
        .unwrap()
        .0
        .set_value("", &app_opne_url_command);
}

pub fn unregister() {
    let hklm = RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
    hklm.open_subkey_with_flags("SOFTWARE\\RegisteredApplications", enums::KEY_ALL_ACCESS)
        .unwrap()
        .delete_value(APP_ID)
        .unwrap();
    hklm.delete_subkey_all(format!("SOFTWARE\\{}", APP_ID))
        .unwrap();
    hklm.delete_subkey_all(format!("SOFTWARE\\Classes\\{}URL", APP_ID))
        .unwrap();
}
