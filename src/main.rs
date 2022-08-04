use std::path::Path;

use env_logger::Env;
#[allow(unused_imports)]
use log::{debug, error, info, warn};

mod compile;
mod option;
mod port;
mod setting;

use port::TMP_DIR;

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

    for src in source {
        let src_path: &Path = Path::new(&src);
        let src_dir = match src_path.parent() {
            None => {
                warn!("invalid file name: {}", src);
                continue;
            }
            Some(x) => x.to_str().unwrap().to_string(),
        };
        let src_stem = match src_path.file_stem() {
            None => {
                warn!("invalid file name: {}", src);
                continue;
            }
            Some(x) => x.to_str().unwrap().to_string(),
        };
        let src_name = match src_path.file_name() {
            None => {
                warn!("invalid file name: {}", src);
                continue;
            }
            Some(x) => x.to_str().unwrap().to_string(),
        };

        if flags.check(option::Flags::GENERATE_C) {
            let mut cfile = src_stem.clone();
            cfile.push_str(".c");
            compile::build_c(src_name, src_dir, cfile);
        } else {
            compile::build_c(
                src_name,
                TMP_DIR.to_string(),
                compile::random_cfile(TMP_DIR.to_string() + "/c").expect("rfx tmp dir full"),
            );
        }
    }
}
