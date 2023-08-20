use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
pub enum KeyWordType {
    KVoid,
    KChar,
    KInt,
    KFloat,
    Kdouble,
}

#[derive(Debug)]
pub enum OperatorType {
    OpEq,
    OpAssign,
}

#[derive(Debug)]
pub enum TokenType {
    Note,
    NewLine,
    Space,
    KeyWord(KeyWordType),
    Number,
    FlotNumber,
    Str,
    Char,
    Identifier,
    Operator(OperatorType),
}

#[derive(Debug)]
pub struct Location {
    file: String,
    line: usize,
    column: usize,
}

impl Location {
    pub fn show(&self) -> String {
        format!("{}:{}:{}", &self.file, self.line, self.column)
    }
}

#[derive(Debug)]
pub struct Token {
    loc: Location,
    token_type: TokenType,
    source: String,
}

impl Token {
    pub fn show(&self) -> String {
        format!("'{}' [{:?}] Loc:({})", &self.source, self.token_type, self.loc.show())
    }
}

#[derive(Debug)]
pub struct Lex {
    file: String,
    tokens: Vec<Token>,

    index: usize,
    line: usize,
    column: usize,
}

impl Lex {
    pub fn new(file: &str) -> Self {
        let lex = Lex {
            file: String::from(file),
            tokens: Vec::<Token>::new(),
            index: 0,
            line: 1,
            column: 1,
        };
        lex
    }

    pub fn add_token(&mut self, loc: Location, token_type: TokenType, source: &str) {
        let token = Token {
            loc: loc,
            token_type: token_type,
            source: String::from(source),
        };

        self.tokens.push(token);
    }

    pub fn show(&self) -> String {
        let mut str = String::new();

        for token in &self.tokens {
            match token.token_type {
                TokenType::Note | TokenType::NewLine | TokenType::Space => continue,
                _ => str += &format!("{}\n", token.show()).as_str(),
            }
        }
        str.pop();
        str
    }

    pub fn parse(&mut self) {
        let path = Path::new(&self.file);

        let mut file = match File::open(&path) {
            Err(why) => {
                eprintln!("couldn't open {}: {:?}", &self.file, why);
                std::process::exit(-1);
            },
            Ok(file) => file,
        };

        let mut src = String::new();
        match file.read_to_string(&mut src) {
            Err(why) => {
                eprintln!("couldn't read {}: {:?}", &self.file, why);
                std::process::exit(-1);
            },
            Ok(_) => (),
        };

        let bytes = src.as_bytes();
        while self.index < bytes.len() {
            if self.parse_note(bytes) {
                continue;
            }
            if self.parse_new_line(bytes) {
                continue;
            }
            if self.parse_space(bytes) {
                continue;
            }
            if self.parse_string(bytes) {
                continue;
            }
            if self.parse_char(bytes) {
                continue;
            }
            if self.parse_identifier(bytes) {
                continue;
            }
            if self.parse_operator(bytes) {
                continue;
            }
            if self.parse_number(bytes) {
                continue;
            }

            self.index += 1;
            self.column += 1;
        }
    }

    fn parse_note(&mut self, bytes: &[u8]) -> bool {
        if bytes.len() - self.index < 2 {
            return false;
        }

        if (bytes[self.index] as char == '/') && (bytes[self.index + 1] as char == '*') {
            let start = self.index;
            self.index += 2;
            let line = self.line;
            let column = self.column;

            loop {
                if bytes.len() - self.index < 2 {
                    eprintln!("'/*' Missing ending");
                    std::process::exit(-1);
                }

                let chr = bytes[self.index] as char;
                if chr == '\n' {
                    self.line += 1;
                    self.column = 1;
                    self.index += 1;
                    continue;
                }

                if chr == '*' && bytes[self.index + 1] as char == '/' {
                    self.index += 2;
                    let token = Token {
                        loc: Location {
                            file: String::from(&self.file),
                            line: line,
                            column: column,
                        },
                        token_type: TokenType::Note,
                        source: String::from_utf8_lossy(&bytes[start..self.index].to_vec()).to_string(),
                    };
                    self.tokens.push(token);

                    self.column += 2;
                    break;
                }
                self.column += 1;
                self.index += 1;
            }
            return true;
        } else if (bytes[self.index] as char == '/') && (bytes[self.index + 1] as char == '/') {
            let start = self.index;
            self.index += 2;
            let column = self.column;

            loop {
                if bytes.len() >= self.index {
                    eprintln!("'//' Missing ending");
                    std::process::exit(-1);
                }

                let chr = bytes[self.index] as char;
                if chr == '\n' {
                    self.index += 1;
                    let token = Token {
                        loc: Location {
                            file: String::from(&self.file),
                            line: self.line,
                            column: column,
                        },
                        token_type: TokenType::Note,
                        source: String::from_utf8_lossy(&bytes[start..self.index].to_vec()).to_string(),
                    };
                    self.tokens.push(token);

                    self.line += 1;
                    self.column = 1;
                    break;
                }
                self.column += 1;
                self.index += 1;
            }
            return true;
        }

        false
    }

    fn parse_new_line(&mut self, bytes: &[u8]) -> bool {
        if !(bytes[self.index] as char == '\n') {
            return false;
        }

        let token = Token {
            loc: Location {
                file: String::from(&self.file),
                line: self.line,
                column: self.column,
            },
            token_type: TokenType::NewLine,
            source: String::from("\n"),
        };
        self.tokens.push(token);
        self.index += 1;
        self.line += 1;
        self.column = 1;
        true
    }

    fn parse_space(&mut self, bytes: &[u8]) -> bool {
        if !(bytes[self.index] as char == ' ') {
            return false;
        }

        let token = Token {
            loc: Location {
                file: String::from(&self.file),
                line: self.line,
                column: self.column,
            },
            token_type: TokenType::Space,
            source: String::from(" "),
        };
        self.tokens.push(token);
        self.index += 1;
        self.column += 1;
        true
    }

    fn parse_string(&mut self, bytes: &[u8]) -> bool {
        if !(bytes[self.index] as char == '\"') {
            return false;
        }

        let mut skip = false;
        let start = self.index;
        let mut column = self.column;
        let mut line = self.line;
        self.index += 1;

        loop {
            if bytes.len() <= self.index {
                eprintln!("Error: \"Missing '\"' at the end\" at ({}:{}:{})",
                    self.file, self.line, self.column);
                std::process::exit(-1);
            }

            let chr = bytes[self.index] as char;
            match chr {
                '\"' => {
                    if !skip {
                        self.index += 1;
                        let token = Token {
                            loc: Location {
                                file: String::from(&self.file),
                                line: self.line,
                                column: self.column,
                            },
                            token_type: TokenType::Str,
                            source: String::from_utf8_lossy(&bytes[start..self.index].to_vec()).to_string(),
                        };
                        self.tokens.push(token);

                        self.line = line;
                        self.column = column + 1;
                        return true;
                    } else {
                        skip = false;
                    }
                },
                '\\' => {
                    skip = true;
                },
                '\n' => {
                    skip = false;
                    line += 1;
                    column = 0;
                },
                _ => skip = false,
            }
            self.index += 1;
            column += 1;
        }
    }

    fn parse_char(&mut self, bytes: &[u8]) -> bool {
        if !(bytes[self.index] as char == '\'') {
            return false;
        }

        let mut skip = false;
        let start = self.index;
        let mut column = self.column;
        self.index += 1;
        let mut max: usize = 2;

        loop {
            if bytes.len() <= self.index {
                eprintln!("Error: \"Missing '\'' at the end\" at ({}:{}:{})",
                    self.file, self.line, self.column);
                std::process::exit(-1);
            }
            if self.index > max {
                eprintln!("Error: \"There can only be one character between \"''\"\" at ({}:{}:{})",
                    self.file, self.line, self.column);
                std::process::exit(-1);
            }

            let chr = bytes[self.index] as char;
            match chr {
                '\'' => {
                    if !skip {
                        self.index += 1;
                        let token = Token {
                            loc: Location {
                                file: String::from(&self.file),
                                line: self.line,
                                column: self.column,
                            },
                            token_type: TokenType::Char,
                            source: String::from_utf8_lossy(&bytes[start..self.index].to_vec()).to_string(),
                        };
                        self.tokens.push(token);

                        self.column = column + 1;
                        return true;
                    } else {
                        skip = false;
                    }
                },
                '\\' => {
                    skip = true;
                    max = 3;
                },
                _ => {
                    skip = false;
                    if !chr.is_ascii() {
                        eprintln!("Error: \"[{}] is not an ascii character\" at ({}:{}:{})",
                         chr as u8, self.file, self.line, column);
                        std::process::exit(-1);
                    }
                },
            }
            self.index += 1;
            column += 1;
        }
    }

    fn parse_keyword(&mut self, bytes: &[u8]) -> bool {
        false
    }

    fn parse_identifier(&mut self, bytes: &[u8]) -> bool {
        match bytes[self.index] as char {
            'a'..='z' | 'A'..='Z' | '_' => (),
            _ => return false,
        }

        let start = self.index;
        let mut index = self.index;
        let mut column = self.column;

        loop {
            if bytes.len() <= index {
                let token = Token {
                    loc: Location {
                        file: String::from(&self.file),
                        line: self.line,
                        column: self.column,
                    },
                    token_type: TokenType::Identifier,
                    source: String::from_utf8_lossy(&bytes[start..index].to_vec()).to_string(),
                };
                self.tokens.push(token);

                self.index = index;
                self.column = column;
                return true;
            }

            let chr = bytes[index] as char;
            match chr {
                ' ' | ';' | ',' | '\t' | '\n' | '\x0C' | '\r' | '!'..='/' | ':'..='@' | '['..='`' | '{'..='~' => {
                    if chr != '_' {
                        let token = Token {
                            loc: Location {
                                file: String::from(&self.file),
                                line: self.line,
                                column: self.column,
                            },
                            token_type: TokenType::Identifier,
                            source: String::from_utf8_lossy(&bytes[start..index].to_vec()).to_string(),
                        };
                        self.tokens.push(token);

                        self.index = index;
                        self.column = column;
                        return true;
                    }
                },
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => (),
                _ => {
                    eprintln!("Error: \"'{}' cannot be an identifier\" at ({}:{}:{})",
                        chr, self.file, self.line, self.column);
                    std::process::exit(-1);
                },
            }
            index += 1;
            column += 1;
        }
    }

    fn parse_operator(&mut self, bytes: &[u8]) -> bool {
        if !(bytes[self.index] as char).is_ascii_punctuation() {
            return false;
        }

        let token = Token {
            loc: Location {
                file: String::from(&self.file),
                line: self.line,
                column: self.column,
            },
            token_type: TokenType::Operator(OperatorType::OpEq),
            source: String::from_utf8_lossy(&bytes[self.index..(self.index + 1)].to_vec()).to_string(),
        };
        self.tokens.push(token);

        self.index += 1;
        self.column += 1;
        return true;
    }

    fn parse_number(&mut self, bytes: &[u8]) -> bool {
        let mut f_flag = false;
        let mut bin_num = false;
        let mut oct_num = false;
        let mut hex_num = false;
        let mut dec_num = false;
        let mut err_token = false;

        match bytes[self.index] as char {
            '0'..='9' => (),
            _ => return false,
        }

        let start = self.index;
        let mut index = self.index;
        let mut column = self.column;

        loop {
            if bytes.len() <= index {
                if err_token {
                    eprintln!("Error: \"Identifiers cannot start with a number\" at ({}:{}:{})",
                            self.file, self.line, self.column);
                    std::process::exit(-1);
                } else {
                    let token = Token {
                        loc: Location {
                            file: String::from(&self.file),
                            line: self.line,
                            column: self.column,
                        },
                        token_type: TokenType::Number,
                        source: String::from_utf8_lossy(&bytes[start..index].to_vec()).to_string(),
                    };
                    self.tokens.push(token);

                    self.index = index;
                    self.column = column;
                    return true;
                }
            }

            let chr = bytes[index] as char;
            match chr {
                '0' => {
                    if !err_token {
                        if f_flag {
                            err_token = true;
                        } else if start == index {
                            oct_num = true;
                        }
                    }
                },
                '1' => {
                    if !err_token {
                        if f_flag {
                            err_token = true;
                        }else if !bin_num && !oct_num && !hex_num {
                            dec_num = true;
                        }
                    }
                },
                '2'..='7' => {
                    if !err_token {
                        if f_flag {
                            err_token = true;
                        } else {
                            if !bin_num && !oct_num && !hex_num {
                                dec_num = true;
                            }
                            if bin_num {
                                eprintln!("Error: \"The number of binary values exceeds 1\" at ({}:{}:{})",
                                    self.file, self.line, column);
                                std::process::exit(-1);
                            }
                        }
                    }
                },
                '8'..='9' => {
                    if !err_token {
                        if f_flag {
                            err_token = true;
                        } else {
                            if !bin_num && !oct_num && !hex_num {
                                dec_num = true;
                            }
                            if bin_num {
                                eprintln!("Error: \"The number of binary values exceeds 1\" at ({}:{}:{})",
                                    self.file, self.line, column);
                                std::process::exit(-1);
                            } else if oct_num {
                                eprintln!("Error: \"The number of octal values exceeds 7\" at ({}:{}:{})",
                                    self.file, self.line, column);
                                std::process::exit(-1);
                            }
                        }
                    }
                },
                'a' | 'A' | 'c'..='e' | 'C'..='E'  => {
                    if !err_token {
                        if f_flag {
                            err_token = true;
                        } else {
                            if bin_num {
                                eprintln!("Error: \"The number of binary values exceeds 1\" at ({}:{}:{})",
                                    self.file, self.line, column);
                                std::process::exit(-1);
                            } else if oct_num {
                                eprintln!("Error: \"The number of octal values exceeds 7\" at ({}:{}:{})",
                                    self.file, self.line, column);
                                std::process::exit(-1);
                            } else if dec_num {
                                eprintln!("Error: \"The number of decimal values exceeds 9\" at ({}:{}:{})",
                                        self.file, self.line, column);
                                std::process::exit(-1);
                            } else if !hex_num {
                                err_token = true;
                            }
                        }
                    }
                },
                'x' | 'X' => {
                    if !err_token {
                        if !(oct_num && (index - start == 1)) {
                            err_token = true;
                        } else {
                            oct_num = false;
                            hex_num = true;
                        }
                    }
                },
                'b' | 'B' => {
                    if !err_token {
                        if oct_num {
                            if index - start == 1 {
                                oct_num = false;
                                bin_num = true;
                            } else {
                                eprintln!("Error: \"The number of octal values exceeds 7\" at ({}:{}:{})",
                                    self.file, self.line, column);
                                std::process::exit(-1);
                            }
                        } else if bin_num {
                            eprintln!("Error: \"The number of binary values exceeds 1\" at ({}:{}:{})",
                                    self.file, self.line, column);
                            std::process::exit(-1);
                        } else if dec_num {
                            eprintln!("Error: \"The number of decimal values exceeds 9\" at ({}:{}:{})",
                                    self.file, self.line, column);
                            std::process::exit(-1);
                        } else if !hex_num {
                            err_token = true;
                        }
                    }
                },
                ' ' | ';' | ',' | '\t' | '\n' | '\x0C' | '\r' | '!'..='/' | ':'..='@' | '['..='`' | '{'..='~' => {
                    if err_token {
                        eprintln!("Error: \"Identifiers cannot start with a number\" at ({}:{}:{})",
                                self.file, self.line, self.column);
                        std::process::exit(-1);
                    } else {
                        let token = Token {
                            loc: Location {
                                file: String::from(&self.file),
                                line: self.line,
                                column: self.column,
                            },
                            token_type: TokenType::Number,
                            source: String::from_utf8_lossy(&bytes[start..index].to_vec()).to_string(),
                        };
                        self.tokens.push(token);

                        self.index = index;
                        self.column = column;
                        return true;
                    }
                },
                'f' | 'F' => {
                    if !err_token {
                        if !hex_num {
                            if index != start && (bytes[index - 1] as char).is_ascii_digit() {
                                if f_flag {
                                    err_token = true;
                                } else {
                                    f_flag = true;
                                }
                            } else {
                                if bin_num {
                                    eprintln!("Error: \"The number of binary values exceeds 1\" at ({}:{}:{})",
                                        self.file, self.line, column);
                                    std::process::exit(-1);
                                } else if oct_num {
                                    eprintln!("Error: \"The number of octal values exceeds 7\" at ({}:{}:{})",
                                        self.file, self.line, column);
                                    std::process::exit(-1);
                                } else if dec_num {
                                    eprintln!("Error: \"The number of decimal values exceeds 9\" at ({}:{}:{})",
                                            self.file, self.line, column);
                                    std::process::exit(-1);
                                }
                            }
                        }
                    }
                }
                _ => {
                    err_token = true;
                },
            }

            index += 1;
            column += 1;
        }
    }
}
