use super::port::FILENAME_CHARS;
use std::path::Path;

mod ast;
mod lex;
mod parser;
mod syntax;

pub fn build_c(rfxfile: String, outdir: String, cfile: String) {}

pub fn random_cfile(dir: String) -> Option<String> {
    let mut ret: Option<String> = None;
    let dir_path = Path::new(&dir);

    for seed in 0..FILENAME_CHARS.len().pow(10) {
        let check = dir_path.clone();
        let mut filename: String = String::from("0000000000"); //ten chars

        for ci in 1..10 {
            let x = seed % FILENAME_CHARS.len().pow(ci) / FILENAME_CHARS.len().pow(ci - 1);

            let index = filename.len() - (ci as usize) + 1;
            let replace = FILENAME_CHARS[x];
            filename = filename
                .chars()
                .enumerate()
                .map(|(i, c)| if i == index { replace } else { c })
                .collect();
        }

        let full = check.join(filename.clone() + ".c");
        if !full.exists() {
            ret = Some(filename + ".c");
            break;
        }
    }

    return ret;
}
