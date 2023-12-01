
pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
}

pub struct Token {
    pub ty: TOKEN,
    pub start: usize,
    pub length: usize,
    pub line: usize,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum TOKEN {
    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,
    // One or two character tokens.
    BANG, BANG_EQUAL, EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL, LESS, LESS_EQUAL,
    // Literals.
    IDENTIFIER, STRING, NUMBER,
    // Keywords.
    AND, CLASS, ELSE, FALSE, FOR, FUN, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
}

#[derive(Debug)]
pub struct TokenError {
    message: String,
    line: usize,
}

type TokenResult = Result<Token, TokenError>;

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {source: source, start: 0, current: 0, line: 1}
    }

    pub fn scan_token(&mut self) -> TokenResult {
        self.start = self.current;
        if self.is_at_end() {
            return Ok(self.make_token(TOKEN::EOF));
        }  

        Err(self.error_token("Unexpected character."))
    }

    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }

    fn make_token(&self, ty: TOKEN) -> Token {
        Token {ty: ty, start: self.start, length: self.current - self.start, line: self.line}
    }

    fn error_token(&self, message: &str) -> TokenError {
        TokenError { message: message.to_owned(), line: self.line }
    }
}

