use fs::File;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use std::fs;
use std::io::Read;
use std::path::Path;

use super::port::{RFX_HOME_DIR, SETTING_FILE};

fn create_rfx_dir() -> std::io::Result<()> {
    fs::create_dir(RFX_HOME_DIR)?;
    return Ok(());
}

fn create_setting_file() -> std::io::Result<()> {
    File::create(SETTING_FILE)?;
    return Ok(());
}

pub fn init() {
    if !Path::new(RFX_HOME_DIR).exists() {
        create_rfx_dir().unwrap();
    }

    if !Path::new(SETTING_FILE).exists() {
        create_setting_file().unwrap();
    }
}

pub fn read_setting(custom_cc: &mut String, custom_ofile: &mut String) {
    let mut sfile = File::open(SETTING_FILE).unwrap();
    let mut setting_raw: String = String::new();
    let setting_parsed: json::JsonValue;

    sfile.read_to_string(&mut setting_raw).unwrap();
    setting_parsed = match json::parse(setting_raw.as_str()) {
        Err(_e) => {
            return;
        }

        Ok(x) => x,
    };

    if !setting_parsed["DEFAULT_CC"].is_null() {
        *custom_cc = setting_parsed["DEFAULT_CC"].as_str().unwrap().to_string();
    }
    if !setting_parsed["DEFAULT_OUT_BINARY"].is_null() {
        *custom_ofile = setting_parsed["DEFAULT_OUT_BINARY"]
            .as_str()
            .unwrap()
            .to_string();
    }
}
