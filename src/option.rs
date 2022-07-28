use std::env;

use bitflags::bitflags;

bitflags! {
    pub struct Flags: u32 {
        const INIT = 0x0;
        const VERBOSE = 0x1;
        const QUIET = 0x2;

        const GENERATE_C = 0x4;
        const GENERATE_ASM = 0x8;
        const GENERATE_OBJ = 0x10;
        const GENERATE_BIN = 0x20;

        const CUSTOM_CC = 0x40;
        const CUSTOM_OUTFILE = 0x80;

        const BIT16 = 0x100;
        const BIT32 = 0x200;
        const BIT64 = 0x400;
    }
}

impl Flags {
    pub fn check(self, f: Self) -> bool {
        return self.bits & f.bits != 0;
    }
}

pub fn parse(
    custom_cc: &mut String,
    custom_ofile: &mut String,
    source: &mut Vec<String>,
    flag: &mut Flags,
) {
    //arg withoout binary name
    let args: Vec<String> = env::args().skip(1).collect();

    *flag = Flags::INIT;

    for (i, a) in args.iter().enumerate() {
        match a.chars().nth(1) {
            None => {
                continue;
            }

            Some('-') => {
                //option
                let c = a.chars().nth(1);
                if c == Some('-') {
                    //long option
                    match a[2..].to_lowercase().as_str() {
                        "verbose" => {
                            flag.set(Flags::VERBOSE, true);
                            flag.set(Flags::QUIET, false);
                        }
                        "cc" => {
                            flag.set(Flags::CUSTOM_CC, true);
                            let cc = args.iter().nth(i + 1);
                            if cc != None {
                                *custom_cc = cc.unwrap().clone();
                            } else {
                                println!("No CC supplyed with --CC option, ignoring.");
                            }
                        }
                        _ => {
                            println!("invalid option, ignoring: {}", a);
                        }
                    }
                } else if c == None {
                    println!("invalid option, ignoring: {}", a);
                } else {
                    //short option
                    match a[1..].to_lowercase().as_str() {
                        "v" => {
                            flag.set(Flags::VERBOSE, true);
                        }
                        "q" => {
                            flag.set(Flags::QUIET, true);
                            flag.set(Flags::VERBOSE, false);
                        }

                        "c" => {
                            flag.set(Flags::GENERATE_C, true);
                        }
                        "a" => {
                            flag.set(Flags::GENERATE_ASM, true);
                        }
                        "d" => {
                            flag.set(Flags::GENERATE_OBJ, true);
                        }
                        "b" => {
                            flag.set(Flags::GENERATE_BIN, true);
                        }

                        "m16" => {
                            flag.set(Flags::BIT16, true);
                        }
                        "m32" => {
                            flag.set(Flags::BIT32, true);
                        }
                        "m64" => {
                            flag.set(Flags::BIT64, true);
                        }

                        "o" => {
                            flag.set(Flags::CUSTOM_OUTFILE, true);
                            let ofile = args.iter().nth(i + 1);
                            if ofile != None {
                                *custom_ofile = ofile.unwrap().clone();
                            } else {
                                println!("No output file supplyed with -o option, ignoring.");
                            }
                        }
                        _ => {
                            println!("invalid option, ignoring: {}", a);
                        }
                    }
                }
            }

            _ => {
                //source file path
                source.push(a.to_string());
            }
        }
    }
}
