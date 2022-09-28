use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::Chars;

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

    Equal,
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
    Comma,
    Semicolon,
    DblColon,
    Arrow,

    Const,
    Impl,
    Trait,
    Pub,
    Private,

    If,
    While,
    For,

    Use,
    Fn,

    LineComment,
    LongComment,
    BrokenLeftLongComment,
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

static LONG_SP_OPERATOR: &'static [TokenDict] = &[
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
    TokenDict {
        s: "->",
        tp: TokenType::Arrow,
    },
];
static SHORT_SP_OPERATOR: &'static [TokenDict] = &[
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
        s: "=",
        tp: TokenType::Equal,
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
        s: ",",
        tp: TokenType::Comma,
    },
    TokenDict {
        s: ";",
        tp: TokenType::Semicolon,
    },
];
static KEYWORDS: &'static [TokenDict] = &[
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
    TokenDict {
        s: "fn",
        tp: (TokenType::Fn),
    },
];

struct LexStream {
    line: u64,
    col: u64,
    eof: bool,
    eol: bool,
    reader: BufReader<File>,
    linebuf: String,
}

impl LexStream {
    pub fn new(f: File) -> Self {
        Self {
            line: 0,
            col: 0,
            eof: false,
            eol: false,
            reader: BufReader::new(f),
            linebuf: String::new(),
        }
    }

    pub fn line(&self) -> u64 {
        self.line
    }

    pub fn col(&self) -> u64 {
        self.col
    }

    pub fn is_eof(&self) -> bool {
        self.eof
    }

    fn is_eol(&self) -> bool {
        self.eol
    }

    fn new_line(&mut self) {
        if self.reader.read_line(&mut self.linebuf).unwrap() == 0 {
            self.eof = true;
        }

        self.eol = false;
        self.line += 1;
        self.col = 0;
    }
}

pub trait Lex {
    fn peek(&mut self) -> Option<Token>;
    fn lex(&mut self) -> Option<Token>;
}

impl Lex for LexStream {
    fn lex(&mut self) -> Option<Token> {
        let token: Option<Token>;
        let mut siter;

        if self.is_eof() {
            return None;
        } else if self.is_eol() {
            self.new_line();
        }

        siter = self.linebuf.chars();
        token = siter.lex();

        if token.is_none() {
            // End of Line
            self.new_line();
            return self.lex();
        }

        self.col += self.linebuf.chars().count() as u64 - siter.clone().count() as u64;
        self.linebuf = siter.collect();
        let unwrap_ret = token.unwrap();
        return Some(unwrap_ret);
    }

    fn peek(&mut self) -> Option<Token> {
        return self.linebuf.chars().peek();
    }
}

impl Lex for Chars<'_> {
    fn peek(&mut self) -> Option<Token> {
        return self.clone().lex();
    }

    fn lex(&mut self) -> Option<Token> {
        let mut ret = Token::new();
        let mut lex_iter;

        //skip whitespace
        loop {
            let c = self.clone().next();
            if c.is_none() {
                //<whitespace><end of string>
                return None;
            }
            if !c.unwrap().is_whitespace() {
                break;
            }

            self.next();
        }

        //remove comment
        if self.clone().count() >= 2 {
            // checked length
            let s: String = self.clone().take(2).collect();
            if s == "//" {
                //skip '//'
                self.next();
                self.next();

                //skip one line
                loop {
                    let e = self.next();
                    if e.is_none() {
                        return None;
                    }

                    let c = e.unwrap();
                    if c == '\n' {
                        break;
                    }
                }
            } else if s == "/*" {
                //long comment
                let self_save = self.clone();
                let mut iter_long_comment = self.clone().zip(self.clone().skip(1));
                loop {
                    let e = iter_long_comment.next();
                    if e.is_none() {
                        ret.tp = TokenType::BrokenLeftLongComment;
                        ret.s = self
                            .take(self_save.count() - iter_long_comment.count())
                            .collect();
                        return Some(ret);
                    }

                    let (c1, c2) = e.unwrap();
                    if c1 == '*' && c2 == '/' {
                        break;
                    }
                }
            }
        }
        //skip whitespace
        loop {
            let c = self.clone().next();
            if c.is_none() {
                //<whitespace><end of string>
                return None;
            }
            if !c.unwrap().is_whitespace() {
                break;
            }

            self.next();
        }

        //this is start
        lex_iter = self.clone();

        //start lex
        'end_lex: while let Some(curr_char) = lex_iter.clone().next() {
            if curr_char.is_whitespace() {
                break;
            }

            //treat _ as alphabet
            if !curr_char.is_ascii_alphanumeric() && curr_char != '_' {
                //special char
                if !lex_iter.clone().eq(self.clone()) {
                    //<alphabets><special char(= curr_char)>
                    break;
                }

                let max_len = lex_iter.clone().count();
                //check for long operator
                for dict in LONG_SP_OPERATOR.iter() {
                    if max_len >= dict.s.len()
                        && lex_iter
                            .clone()
                            .take(dict.s.chars().count())
                            .collect::<String>()
                            == *dict.s
                    {
                        let found = dict;
                        ret.tp = found.tp;
                        ret.s = dict.s.to_string();
                        for _i in 0..dict.s.chars().count() {
                            lex_iter.next();
                        }
                        break 'end_lex;
                    }
                }

                //check for short operator
                for dict in SHORT_SP_OPERATOR.iter() {
                    if curr_char == dict.s.chars().nth(0).unwrap() {
                        let found = dict;
                        ret.tp = found.tp;
                        ret.s = dict.s.to_string();
                        lex_iter.next();
                        break 'end_lex;
                    }
                }

                //cannot find matching operator
                while let Some(spc) = lex_iter.clone().next() {
                    if spc.is_alphanumeric() || spc.is_whitespace() || spc == '_' {
                        break;
                    }
                    lex_iter.next();
                }
                ret.tp = TokenType::Broken;
                break;
            }

            lex_iter.next();
        }

        let result: String = self
            .take(self.clone().count() - lex_iter.clone().count())
            .collect();
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

        ret.s = result;
        *self = lex_iter;
        return Some(ret);
    }
}

#[cfg(test)]
mod tests {
    use crate::compile::lex::Lex;
    use std::fs::File;
    use std::io::stdout;
    use std::io::Write;

    use super::LexStream;

    #[test]
    fn test_lex_char() {
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
        let mut str_chars = test_str.chars();
        let mut i = 0;
        loop {
            i += 1;
            let token = str_chars.lex();
            if token.is_none() {
                break;
            }
            let unwrap_token = token.unwrap();
            println!("{}, {}: \'{}\'", i, unwrap_token.tp, unwrap_token.s);
            stdout().flush().unwrap();
        }
    }

    #[test]
    fn test_lex_file() {
        let test_file = File::open("test/lex.test").unwrap();
        let mut lexs = LexStream::new(test_file);

        loop {
            let token = lexs.lex();
            if token.is_none() || lexs.is_eof() {
                break;
            }
            let unwrap_token = token.unwrap();
            println!("{}, {}: \'{}\'", lexs.line(), lexs.col(), unwrap_token.s);
            stdout().flush().unwrap();
        }
    }
}
/*

}*/
