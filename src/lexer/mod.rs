use crate::debug;
use crate::types::token::Token;
use crate::types::EString;
use crate::types::token::Keyword;
use std::mem::discriminant;
use crate::errors::Error;
use crate::errors::error;
use crate::types::token::Types;

pub fn lex(string: EString) -> Vec<Vec<Token>> {
    let mut lex_inst = Lexer {
        string: String::from(""),
        charlist: Vec::new(),
        cursor: 0,
        finlist: Vec::new(),
        line: 1,
        oldlist: Vec::new()
    };

    let mut oldlist: Vec<Vec<Token>> = Vec::new();
    let split = string.as_str().split("\n").collect::<Vec<&str>>();
    println!("{:?}", &split);
    for s in split {
        lex_inst.advance_line();
        lex_inst.string = s.to_string();
        oldlist.push(lex_inst.lex());
    }
    return oldlist;
}

pub struct Lexer {
    string: EString,
    cursor: usize,
    charlist: Vec<char>,
    finlist: Vec<Token>,
    line: usize,
    oldlist: Vec<Vec<Token>>
}

impl Lexer {
    pub fn lex(&mut self) -> Vec<Token> {
        let mut toklist: Vec<Token> = Vec::new();

        for i in self.string.chars() {
            self.charlist.push(i);
        }

        for i in 0..self.charlist.len() {
            let matcher = self.matcher();
            if matches!(matcher, Token::EMBreak) {
                break;
            } else {
                toklist.push(matcher);
            }
        }
        self.finlist = toklist;
        self.rules();
        let lfinlist = self.finlist.clone();
        self.flush();
        lfinlist
    }

    pub fn matcher(&mut self) -> Token {
        if self.cursor == self.charlist.len() {
            return Token::EMBreak;
        }
        match self.charlist[self.cursor] {
			'\r' => {self.advance(); return Token::NewLine},
            '\t' => {let token = Token::Whitespace; self.advance(); return token;},
            ';' => {let token = Token::Semicolon(self.charlist[self.cursor]); self.advance(); return token;}
            ':' => {self.advance(); return Token::Colon;}
			'{' => {self.advance(); self.make_args()},
            '}' => {self.advance(); return Token::RBraces},
            '-' => {let token = Token::Minus(self.charlist[self.cursor]); self.advance(); return token;},
            '>' => {let token = Token::Arer(self.charlist[self.cursor].to_string()); self.advance(); return token;},
            '<' => {let token = Token::Arer(self.charlist[self.cursor].to_string()); self.advance(); return token;},
            '+' => {let token = Token::Plus(self.charlist[self.cursor]); self.advance(); return token;},
            '=' => {let token = Token::Equals(self.charlist[self.cursor]); self.advance(); return token;},
            '(' => {self.advance(); self.make_function()},
            ')' => {let token = Token::Parenth(self.charlist[self.cursor]); self.advance(); return token;},
            '|' => {let token = Token::Pipe(self.charlist[self.cursor]); self.advance(); return token;},
            '"' => {self.advance(); self.make_string()},
            '0'..'9' => self.make_number(),
            '9' => self.make_number(),
            'a'..'z' => self.make_keyword(),
            'z' => self.make_keyword(),
            ' ' => {self.advance(); Token::Whitespace},
            _ => {error(Error::UnknownChar, self.line, self.charlist[self.cursor]); return Token::Fault(String::from(""), String::from(""))},
        }
    }

    pub fn flush(&mut self) {
        self.cursor = 0;
        self.charlist = Vec::new();
        self.finlist =  Vec::new();
    }

    pub fn advance_line(&mut self) {
        self.line += 1;
    }

    pub fn make_args(&mut self) -> Token {
        let mut raw_args: Vec<Token> = Vec::new(); 
        let mut args_stack: Vec<Token> = Vec::new();
        'matcher: loop {
            if self.cursor == self.charlist.len() {
                break 'matcher;
            }
            match self.charlist[self.cursor] {
                '}' => {
                    self.advance();
                    break 'matcher;
                },
                ',' => {
                    raw_args.push(Token::Comma);
                    self.advance();
                }
                ' ' => {self.advance()}
                _ => {
                    raw_args.push(self.matcher());
                    // self.advance();
                },
            };
        }

        // workaround for a advance bug that goes too far in the variables depth
        /*
        if self.charlist[self.cursor - 1] == '}' && self.charlist[self.cursor - 2] == '}' {
            self.unadvance();
        }
        */

        for i in raw_args {
            match i {
                Token::Comma => {},
                _ => {args_stack.push(i)}
            }
        }
        
        // check if item exists in array, check if it goes like "}|{"
        return Token::Braces(args_stack);
    }

    pub fn make_function(&mut self) -> Token {
        let mut func_name = String::from("");
        'matcher: loop {
            if self.cursor == self.charlist.len() {
                break 'matcher;
            }
            match self.charlist[self.cursor] {
                ')' => {
                    self.advance();
                    break 'matcher;
                }
                _ => {
                    func_name.push(self.charlist[self.cursor]);
                    self.advance();
                },
            };
        }
        // check if item exists in array, check if it goes like "}|{"
        return Token::SomethingFunction(func_name);
    }

    pub fn rules(&mut self) {
        self.cursor = 0;
        let mut patched_list: Vec<Token> = Vec::new();
        for i in &self.finlist {
            if self.cursor == self.finlist.len() {
                debug!("Returned because index was out of bounds, skipping to next procedure");
                break;
            }
            if self.cursor != 0 {
                if discriminant(&self.finlist[self.cursor - 1]) == discriminant(&Token::Minus('-')) && discriminant(i) == discriminant(&Token::Arer(String::from(">"))) {
                    debug!("Found Arer that would be fatal, skipping, the thing {:#?}", i);
                    continue;
                }
            } else { 
                debug!("Found unsubtractable, continuing");
            }
            let arer_arrow = String::from(">");
            match i {
                _i if self.cursor == self.finlist.len() => break,
                i if matches!(i, &Token::Minus('-')) && matches!(&self.finlist[self.cursor + 1], Token::Arer(arer_arrow)) => {
                    debug!("Found ARER forward... Adding to list, some debug: {0:?}, some more debug: {1:?}", self.finlist[self.cursor + 1], i);
                    patched_list.push(Token::Arer("->".to_string()));
                },
                _ => {
                    debug!("Continuing rules iteration, matched underscore _");
                    patched_list.push(i.clone());
                }
            }
            self.cursor += 1;
        }
        self.finlist = patched_list;
        self.match_keyword();
    }
    pub fn match_keyword(&mut self) {
        let mut patched_list: Vec<Token> = Vec::new();
        for i in &self.finlist {
            if let Token::SomethingKeyword(keyword) = i {
                match keyword.as_str() {
                    "maybe" => patched_list.push(Token::Keyword(Keyword::Maybe)),
                    "exports" => patched_list.push(Token::Keyword(Keyword::Exports)),
                    "this" => patched_list.push(Token::Keyword(Keyword::This)),
					"int" => patched_list.push(Token::Type(Types::Int)),
                    _ => {
                        patched_list.push(Token::SomeName(String::from(keyword)))
                    }
                }
            } else {
                patched_list.push(i.clone());
            }
        }
        self.finlist = patched_list;
    }

    pub fn make_keyword(&mut self) -> Token {
        let mut string = String::from("");
        'matcher: loop {
            if self.cursor == self.charlist.len() {
                break 'matcher;
            }
            match self.charlist[self.cursor] {
                ' ' => {
                    self.advance();
                    break 'matcher;
                },
                '}' => {
                    break 'matcher;
                },
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
                break 'matcher;
            }
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
                break 'matcher;
            }
            match self.charlist[self.cursor] {
                '0'..'9' => {
                    num_str.push(self.charlist[self.cursor]);
                    if self.cursor == self.charlist.len() {
                        break 'matcher;
                    }
                    self.advance();
                }
                // quick fix for 9 because its not picked up?
                '9' => {
                    num_str.push(self.charlist[self.cursor]);
                    if self.cursor == self.charlist.len() {
                        break 'matcher;
                    }
                    self.advance();
                }
                _ => {
                    break 'matcher
                },
            };
        }
        return Token::Int(num_str);
    }

    pub fn advance(&mut self) {
        self.cursor += 1;
    }

    pub fn unadvance(&mut self) {
        self.cursor -= 1;
    }
}
