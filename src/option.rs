use bitflags::bitflags;

bitflags! {
    struct Flags: u32 {
        const VERBOSE = 0x1;
        const QUIET = 0x2;

        const GENERATE_C = 0x4;
        const GENERATE_ASM = 0x8;
        const GENERATE_OBJ = 0x10;
        const GENERATE_BIN = 0x20;

        const CUSTOM_CC = 0x40;
    }
}

static mut custom_cc: String = String::new();
static mut source: Vec<String> = Vec::new();

fn parse(){
    //arg withoout binary name
    let args: Vec<String> = env::args().skip(1).collect();

    for a in args{
        if let Some('-') = a.chars().nth(1){
            
        }
    }
}