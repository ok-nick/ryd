use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    plugged: HashMap<String, DisplayOptions>,
    battery: HashMap<String, DisplayOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayOptions {
    refresh_rate: u32,
    width: u32,
    height: u32,
    x: i32,
    y: i32,
    // TODO: https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-changedisplaysettingsw
    // I could also set dmBitsPerPel and flags like grayscale and stuff
}
