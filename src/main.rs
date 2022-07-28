use env_logger::Env;
#[allow(unused_imports)]
use log::{debug, error, info, warn};

mod option;
mod setting;

fn main() {
    let mut custom_cc: String = String::new();
    let mut custom_ofile: String = String::new();
    let mut source: Vec<String> = Vec::new();
    let mut flags: option::Flags = option::Flags::INIT;

    setting::init();
    setting::read_setting(&mut custom_cc, &mut custom_ofile);

    option::parse(&mut custom_cc, &mut custom_ofile, &mut source, &mut flags);

    if flags.check(option::Flags::QUIET) {
        env_logger::Builder::from_env(Env::default().default_filter_or("error")).init();
    } else if flags.check(option::Flags::VERBOSE) {
        env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    } else {
        env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();
    }
}
