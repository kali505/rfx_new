use std::io::{BufRead, BufReader};
use std::str::CharIndices;
use std::{fs::File, io::Seek, io::SeekFrom};

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy, strum_macros::Display)]
pub enum TokenType {
    Null,
    Broken,
    String,
    Number,

    LogicEqual,
    LogicNotEqual,
    LogicAND,
    LogicOR,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBrack,
    RightBrack,
    LeftABrack,
    RightABrack,

    Bang,

    Plus,
    Minus,
    Multify,
    Divide,
    Mod,

    LeftBShift,
    RightBShift,
    BitAND,
    BitOR,
    BitXOR,

    PlusEqu,
    MinusEqu,
    MultifyEqu,
    DivideEqu,
    ModEqu,

    LeftBShiftEqu,
    RightBShiftEqu,
    BAndEqu,
    BOrEqu,
    BXorEqu,

    Quote,
    DblQuote,
    Dot,
    Semicolon,
    DblColon,

    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    Char,
    Bool,

    Const,
    Impl,
    Trait,
    Pub,
    Private,

    If,
    While,
    For,

    Use,

    LineComment,
    LongComment,
    BrokenLongComment,
}

#[derive(Clone)]
pub struct Token {
    pub s: String,
    pub tp: TokenType,
}

impl Token {
    pub fn new() -> Self {
        Token {
            s: String::new(),
            tp: TokenType::Null,
        }
    }
}

#[derive(PartialEq, Clone)]
struct TokenDict<'a> {
    s: &'a str,
    tp: TokenType,
}

static LONG_SP_OPERATOR: [TokenDict; 18] = [
    TokenDict {
        s: "==",
        tp: TokenType::LogicEqual,
    },
    TokenDict {
        s: "!=",
        tp: TokenType::LogicNotEqual,
    },
    TokenDict {
        s: "&&",
        tp: TokenType::LogicAND,
    },
    TokenDict {
        s: "||",
        tp: TokenType::LogicOR,
    },
    TokenDict {
        s: "<<",
        tp: TokenType::LeftBShift,
    },
    TokenDict {
        s: ">>",
        tp: TokenType::RightBShift,
    },
    TokenDict {
        s: "+=",
        tp: TokenType::PlusEqu,
    },
    TokenDict {
        s: "-=",
        tp: TokenType::MinusEqu,
    },
    TokenDict {
        s: "*=",
        tp: TokenType::MultifyEqu,
    },
    TokenDict {
        s: "/=",
        tp: TokenType::DivideEqu,
    },
    TokenDict {
        s: "%=",
        tp: TokenType::ModEqu,
    },
    TokenDict {
        s: "/=",
        tp: TokenType::DivideEqu,
    },
    TokenDict {
        s: "<<=",
        tp: TokenType::LeftBShiftEqu,
    },
    TokenDict {
        s: ">>=",
        tp: TokenType::RightBShiftEqu,
    },
    TokenDict {
        s: "&=",
        tp: TokenType::BAndEqu,
    },
    TokenDict {
        s: "|=",
        tp: TokenType::BOrEqu,
    },
    TokenDict {
        s: "^=",
        tp: TokenType::BXorEqu,
    },
    TokenDict {
        s: "::",
        tp: TokenType::DblColon,
    },
];
static SHORT_SP_OPERATOR: [TokenDict; 21] = [
    TokenDict {
        s: "(",
        tp: TokenType::LeftParen,
    },
    TokenDict {
        s: ")",
        tp: TokenType::RightParen,
    },
    TokenDict {
        s: "{",
        tp: TokenType::LeftBrace,
    },
    TokenDict {
        s: "}",
        tp: TokenType::RightBrace,
    },
    TokenDict {
        s: "[",
        tp: TokenType::LeftBrack,
    },
    TokenDict {
        s: "]",
        tp: TokenType::RightBrack,
    },
    TokenDict {
        s: "<",
        tp: TokenType::LeftABrack,
    },
    TokenDict {
        s: ">",
        tp: TokenType::RightABrack,
    },
    TokenDict {
        s: "!",
        tp: TokenType::Bang,
    },
    TokenDict {
        s: "+",
        tp: TokenType::Plus,
    },
    TokenDict {
        s: "-",
        tp: TokenType::Minus,
    },
    TokenDict {
        s: "*",
        tp: TokenType::Multify,
    },
    TokenDict {
        s: "/",
        tp: TokenType::Divide,
    },
    TokenDict {
        s: "%",
        tp: TokenType::Mod,
    },
    TokenDict {
        s: "&",
        tp: TokenType::BitAND,
    },
    TokenDict {
        s: "|",
        tp: TokenType::BitOR,
    },
    TokenDict {
        s: "^",
        tp: TokenType::BitXOR,
    },
    TokenDict {
        s: "\'",
        tp: TokenType::Quote,
    },
    TokenDict {
        s: "\"",
        tp: TokenType::DblQuote,
    },
    TokenDict {
        s: ".",
        tp: TokenType::Dot,
    },
    TokenDict {
        s: ";",
        tp: TokenType::Semicolon,
    },
];
static KEYWORDS: [TokenDict; 9] = [
    TokenDict {
        s: "const",
        tp: (TokenType::Const),
    },
    TokenDict {
        s: "impl",
        tp: (TokenType::Impl),
    },
    TokenDict {
        s: "trait",
        tp: (TokenType::Trait),
    },
    TokenDict {
        s: "pub",
        tp: (TokenType::Pub),
    },
    TokenDict {
        s: "private",
        tp: (TokenType::Private),
    },
    TokenDict {
        s: "if",
        tp: (TokenType::If),
    },
    TokenDict {
        s: "while",
        tp: (TokenType::While),
    },
    TokenDict {
        s: "for",
        tp: (TokenType::For),
    },
    TokenDict {
        s: "use",
        tp: (TokenType::Use),
    },
];

fn consume_until(c: &mut CharIndices<'_>, end: usize) {
    while let Some((i, _)) = c.next() {
        if i == end {
            break;
        }
    }
}

pub trait Lex {
    fn lex(&mut self) -> Option<Token>;
}

impl Lex for BufReader<File> {
    fn lex(&mut self) -> Option<Token> {
        let self_start = self.stream_position().unwrap();
        let mut s: String = String::new();
        let mut s_ci;
        let s_start;
        let token;

        if self.read_line(&mut s).unwrap() == 0 {
            // End of File
            return None;
        }

        s_ci = s.char_indices();
        s_start = s_ci.clone().peekable().peek().unwrap().0;

        token = s_ci.lex();
        if token.is_none() {
            let s_end = s.char_indices().count();
            self.seek(SeekFrom::Start(self_start + (s_end - s_start) as u64))
                .unwrap();
            return self.lex();
        }

        if s_ci.clone().peekable().peek().is_none() {
            // End of Line & End of File
            self.seek(SeekFrom::End(0)).unwrap();
        } else {
            let s_end = s_ci.clone().peekable().peek().unwrap().0;
            self.seek(SeekFrom::Start(self_start + (s_end - s_start) as u64))
                .unwrap();
        }

        let unwrap_ret = token.unwrap();
        return Some(unwrap_ret);
    }
}

impl Lex for CharIndices<'_> {
    fn lex(&mut self) -> Option<Token> {
        let token_end_index;
        let mut ret = Token::new();
        let start;

        //skip whitespace
        loop {
            if self.clone().peekable().peek().is_none() {
                //<whitespace><end of string>
                return None;
            }
            if !self.clone().peekable().peek().unwrap().1.is_whitespace() {
                start = self.clone().peekable().peek().unwrap().0;
                break;
            }
            self.next();
        }

        //remove comment
        if self.clone().as_str()[0..].len() >= 2 {
            // checked length
            let s = &self.clone().as_str()[0..2];
            if s.eq("//") {
                //one line comment
                let mut iter_short_comment = self.clone();
                let mut i = 0;
                loop {
                    let e = iter_short_comment.next();
                    if e.is_none() {
                        token_end_index = self.as_str().len() - 1;
                        break;
                    }

                    let c = e.unwrap().1;
                    if c == '\n' {
                        token_end_index = i - 1;
                        break;
                    }
                    i += 1;
                }
                ret.s = self.as_str()[0..token_end_index + 1].to_string();
                consume_until(self, start + token_end_index);

                return Some(ret);
            } else if s.eq("/*") {
                //long comment
                let mut iter_long_comment = self.clone().zip(self.clone().skip(1));
                let mut i = 0;
                loop {
                    let e = iter_long_comment.next();
                    if e.is_none() {
                        token_end_index = self.as_str().len() - 1;
                        ret.tp = TokenType::BrokenLongComment;
                        break;
                    }

                    let (c1, c2) = e.unwrap();
                    if c1.1 == '*' && c2.1 == '/' {
                        token_end_index = i + 1;
                        ret.tp = TokenType::LongComment;
                        break;
                    }
                    i += 1;
                }
                ret.s = self.as_str()[0..token_end_index + 1].to_string();
                consume_until(self, start + token_end_index);

                return Some(ret);
            }
        }

        //start lex
        let mut i = 0;
        loop {
            let e = self.clone().nth(i);
            if e.is_none() {
                token_end_index = self.as_str().len() - 1;
                break;
            }

            let curr_char = e.unwrap().1;

            if curr_char.is_whitespace() {
                token_end_index = i - 1;
                break;
            }

            //treat _ as alphabet
            if !curr_char.is_ascii_alphanumeric() && curr_char != '_' {
                //special char
                if i != 0 {
                    //<alphabets><special char(= curr_char)>
                    token_end_index = i - 1;
                    break;
                }

                let max_len = self.clone().as_str()[i..].len();
                //check for long operator
                let mut d: Option<TokenDict> = None;
                for dict in LONG_SP_OPERATOR.iter() {
                    if max_len >= dict.s.len()
                        && self.clone().as_str()[i..i + dict.s.len() - 1] == *dict.s
                    {
                        d = Some(dict.clone());
                        break;
                    }
                }

                if d != None {
                    let found = d.unwrap();
                    ret.tp = found.tp;
                    token_end_index = i + found.s.len() - 1;
                    break;
                }

                //check for short operator
                for dict in SHORT_SP_OPERATOR.iter() {
                    if curr_char == dict.s.chars().nth(0).unwrap() {
                        d = Some(dict.clone());
                        break;
                    }
                }

                if d != None {
                    let found = d.unwrap();
                    ret.tp = found.tp;
                    token_end_index = i;
                    break;
                } else {
                    //cannot find matching operator
                    let mut clen = 0;
                    for spc in self.clone().skip(i) {
                        if spc.1.is_alphanumeric() || spc.1.is_whitespace() {
                            break;
                        }
                        clen += 1;
                    }
                    ret.tp = TokenType::Broken;
                    token_end_index = i + clen - 1;
                    break;
                }
            }
            i += 1;
        }

        let result: &str = &self.as_str()[0..token_end_index + 1];
        if ret.tp == TokenType::Null {
            let mut number = true;
            let mut string = true;
            if result.chars().nth(0).unwrap().is_ascii_digit() {
                string = false;
            }
            for c in result.chars() {
                if !c.is_ascii_digit() {
                    number = false;
                    if !c.is_ascii_alphabetic() && c != '_' {
                        string = false;
                    }
                }
                if !number && !string {
                    break;
                }
            }
            if number {
                ret.tp = TokenType::Number;
            } else if string {
                ret.tp = TokenType::String;
            }
        }
        //check for keyword
        for dict in KEYWORDS.iter() {
            if result == dict.s {
                ret.tp = dict.tp;
                break;
            }
        }

        ret.s = result.to_string();
        consume_until(self, start + token_end_index);
        return Some(ret);
    }
}

#[cfg(test)]
mod tests {
    use crate::compile::lex::Lex;
    use std::fs::File;
    use std::io::stdout;
    use std::io::BufReader;
    use std::io::Seek;
    use std::io::Write;

    #[test]
    fn test_lex_char_indices() {
        let test_str = r#"
        #[cfg(test)]
        mod tests {
            use crate::compile::lex::Lex;

            /* This function is for testing lex() in Trait Lex*/
            #[test]
            fn test_lex() {
                let test_str = "";
                let mut str_ci = test_str.char_indices();
                let mut i = 0;
                loop {
                    i += 1;
                    let token = str_ci.lex();
                    if token.is_none() {
                        break;
                    }
                    let unwrap_token = token.unwrap();
                    // print
                    println!("i: \'{}\'", unwrap_token.s);
                }
            }
        }"#;
        let mut str_ci = test_str.char_indices();
        let mut i = 0;
        loop {
            i += 1;
            let token = str_ci.lex();
            if token.is_none() {
                break;
            }
            let unwrap_token = token.unwrap();
            println!("{}: \'{}\'", i, unwrap_token.s);
            stdout().flush().unwrap();
        }
    }

    #[test]
    fn test_lex_file() {
        let test_file = File::open("test/lex.test").unwrap();
        let mut file_reader = BufReader::new(test_file);
        let mut i = 0;
        loop {
            i += 1;
            let token = file_reader.lex();
            if token.is_none() {
                break;
            }
            let unwrap_token = token.unwrap();
            println!(
                "{}, {}: \'{}\'",
                i,
                file_reader.stream_position().unwrap(),
                unwrap_token.s
            );
            stdout().flush().unwrap();
        }
    }
}
/*

}*/
