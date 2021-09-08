use crate::debug;
use crate::token::Token;
use crate::types::EString;

pub fn lex(string: EString) -> Vec<Token> {
    let mut lex_inst = Lexer {
        string: string,
        charlist: Vec::new(),
        cursor: 0,
    };
    lex_inst.lex()
}

struct Lexer {
    string: EString,
    cursor: usize,
    charlist: Vec<char>,
}

impl Lexer {
    pub fn lex(&mut self) -> Vec<Token> {
        let mut toklist: Vec<Token> = Vec::new();
        let mut x = 0;

        for i in self.string.chars() {
            self.charlist.push(i);
        }

        for i in 0..self.charlist.len() {
            if self.cursor == self.charlist.len() {
                break;
            }
            toklist.push(self.matcher());
        }
        toklist
    }

    pub fn matcher(&mut self) -> Token {
        debug!("something {}", self.charlist[self.cursor]); 
        match self.charlist[self.cursor] {
            // currently fixing the thing
            '-' => {let token = Token::Minus(self.charlist[self.cursor]); self.advance(); return token;},
            '>' => {let token = Token::Arer(self.charlist[self.cursor].to_string()); self.advance(); return token;},
            '<' => {let token = Token::Arer(self.charlist[self.cursor].to_string()); self.advance(); return token;},
            '+' => {let token = Token::Plus(self.charlist[self.cursor]); self.advance(); return token;},
            '=' => {let token = Token::Equals(self.charlist[self.cursor]); self.advance(); return token;},
            '(' => {let token = Token::Parenth(self.charlist[self.cursor]);self.advance(); return token;},
            ')' => {let token = Token::Parenth(self.charlist[self.cursor]); self.advance(); return token;},
            '"' => {self.advance(); self.make_string()},
            '0'..'9' => self.make_number(),
            '9' => self.make_number(),
            'a'..'z' => self.make_keyword(),
            'z' => self.make_keyword(),
            ' ' => {self.advance(); Token::Whitespace},
            _ => {let token = Token::Fault(self.charlist[self.cursor]); self.advance(); return token;},
        }
    }

    pub fn make_keyword(&mut self) -> Token {
        let mut string = String::from("");
        'matcher: loop {
            if self.cursor == self.charlist.len() {
                println!("reached this, shoudl be toiesyegon");
                break 'matcher;
            }
            println!("reached this, shoudl be not break");
            match self.charlist[self.cursor] {
                ' ' => {
                    self.advance();
                    break 'matcher;
                }
                _ => {
                    string.push(self.charlist[self.cursor]);
                    self.advance();
                },
            };
        }
        return Token::SomethingKeyword(string);
    }

    pub fn make_string(&mut self) -> Token {
        let mut string = String::from("");
        'matcher: loop {
            if self.cursor == self.charlist.len() {
                println!("reached this, shoudl be toiesyegon");
                break 'matcher;
            }
            println!("reached this, shoudl be not break");
            match self.charlist[self.cursor] {
                '"' => {
                    self.advance();
                    break 'matcher;
                }
                _ => {
                    string.push(self.charlist[self.cursor]);
                    self.advance();
                },
            };
        }
        return Token::String(string);
    }

    pub fn make_number(&mut self) -> Token {
        let mut num_str: String = String::from("");
        'matcher: loop {
            if self.cursor == self.charlist.len() {
                println!("reached this, shoudl be toiesyegon");
                break 'matcher;
            }
            println!("reached this, shoudl be not break");
            match self.charlist[self.cursor] {
                '0'..'9' => {
                    num_str.push(self.charlist[self.cursor]);
                    if self.cursor == self.charlist.len() {
                        break 'matcher;
                    }
                    println!("htht {0} and {1}", self.cursor, self.charlist[self.cursor]);
                    self.advance();
                }
                // quick fix for 9 because its not picked up?
                '9' => {
                    num_str.push(self.charlist[self.cursor]);
                    if self.cursor == self.charlist.len() {
                        break 'matcher;
                    }
                    println!("htht {0} and {1}", self.cursor, self.charlist[self.cursor]);
                    self.advance();
                }
                _ => {
                    debug!("somethingaaaa {}", self.charlist[self.cursor]); 
                    break 'matcher
                },
            };
        }
        return Token::Int(num_str);
    }

    pub fn advance(&mut self) {
        self.cursor += 1;
    }
}
