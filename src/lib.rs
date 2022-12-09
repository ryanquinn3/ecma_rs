use std::process;
use std::fs;
use std::str::CharIndices;


#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Function,
    Let,
    Const
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword { value: Keyword},
    Identifer {value: String},
    LeftParens,
    RightParens,
    LeftCurly,
    RightCurly,
    LeftBracket,
    RightBracket,
    Dot,
    SemiColon,
    Colon,
    StringLiteral { value: String },
    NumberLiteral{ value: String }, 
    Equals,
    Plus,
    Minus,
    Multiply,
    Division
}



pub struct Lexer<'a>{
    input: &'a str,
    iter: CharIndices<'a>,
    last_char: char,

    last_char_index: usize,
}

impl<'a> Lexer<'a>{

    pub fn new(input: &'a str) -> Lexer<'a>{
        
    
        let mut lexer = Lexer { input,
            iter: input.char_indices(),
             last_char: '\x00',
              last_char_index: 0};

        lexer.scan_char();
        lexer
               
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_nontokens();
        if self.is_done() {
            return None;
        }

        if let Some(token) = self.match_op() {
            self.scan_char();
            return Some(token);
        }
        if self.is_quote() {
            return Some(Token::StringLiteral { value: self.scan_string().to_string() })
        }
        if self.is_identifier_start() {
            return Some(Token::Identifer { value: self.scan_identifier().to_string() })
        }
        if self.last_char.is_alphanumeric() {
            return Some(Token::NumberLiteral { value: self.scan_numeric().to_string() })
        }
        self.scan_char();
        return Some(Token::Dot)
    }

    fn is_quote(&mut self) -> bool {
        self.last_char == '\'' || self.last_char == '\"'
    }

    fn scan_string(&mut self) -> &str {
        let start = self.last_char_index;

        // Consume opening quote
        self.scan_char();
        while !self.is_done() && !self.is_quote() {
            self.scan_char();
        }
        // Consome trailing quote
        self.scan_char();
        // Chop off quotes from returned slice
        &self.input[(start + 1)..(self.last_char_index - 1)]
    }

    fn scan_identifier(&mut self) -> &str {
        let start = self.last_char_index;
        while self.is_identifier_start() {
            self.scan_char();
        }
        &self.input[start..self.last_char_index]
    }

    fn scan_numeric(&mut self) -> &str {
        let start = self.last_char_index;
        while self.last_char.is_alphanumeric() {
            self.scan_char();
        }
        &self.input[start..self.last_char_index]
    }

    fn is_identifier_start(&self) -> bool {
        self.last_char.is_alphabetic() || self.last_char == '_'
    }

    fn match_op(&self) -> Option<Token> {
        match self.last_char {
            '{' =>  Some(Token::LeftCurly),
            '}' =>  Some(Token::RightCurly),
            '[' =>  Some(Token::LeftBracket),
            ']' =>  Some(Token::RightBracket),
            '(' =>  Some(Token::LeftParens),
            ')' =>  Some(Token::RightParens),
            '=' => Some(Token::Equals),
            '.' => Some(Token::Dot),
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Multiply),
            '/' => Some(Token::Division),
            ':' => Some(Token::Colon),
            ';' => Some(Token::SemiColon),
            _ => None
        }
    }


    fn is_done(&self) -> bool {
        self.last_char_index >= self.input.len()
    }

    fn scan_char(&mut self) -> Option<char> {
        if let Some((idx, char)) = self.iter.next() {
            self.last_char = char;
            self.last_char_index = idx;
            return Some(char);
        } 
        self.last_char = '\x00';
        self.last_char_index = self.input.len();
        None
    }

    fn skip_nontokens(&mut self) {
        while self.last_char == ' ' || self.last_char == '\t' || self.last_char == '\r' || self.last_char == '\n' {
            self.scan_char();
        }
    }

}


pub fn from_filepath(filepath: &str) -> String {
    let contents = fs::read_to_string(filepath).unwrap_or_else(|err| {
        eprintln!("failed to read file: {}", err);
        process::exit(1);
    });
    contents
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        return self.next_token();
    }
}