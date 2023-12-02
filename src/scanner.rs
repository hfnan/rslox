
macro_rules! match_token {
    ($self: ident, $expected: expr, $then: expr, $else: expr) => {{
        let ty = if $self.is_at_end() || $self.source.chars().nth($self.current).unwrap() != $expected {
            $then
        } else {
            $self.current += 1;
            $else
        };
        $self.make_token(ty)
    }};
}

macro_rules! digit {
    () => {
        '0'..='9'
    };
}

macro_rules! alpha {
    () => {
        'a'..='z' | 'A'..='Z' | '_'
    };
}

pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
}

pub struct Token<'scanner> {
    pub ty: TOKEN,
    pub source: &'scanner str,
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
    AND = 233, CLASS, ELSE, FALSE, FOR, FUN, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
}

#[derive(Debug)]
pub struct TokenError {
    message: String,
    line: usize,
}

type TokenResult<'scanner> = Result<Token<'scanner>, TokenError>;

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {source: source, start: 0, current: 0, line: 1}
    }

    pub fn scan_token(&mut self) -> TokenResult {
        self.skip_whitespace();
        self.start = self.current;
        if self.is_at_end() {
            return Ok(self.make_token(TOKEN::EOF));
        }  

        let c = self.advance();

        match c {
            '(' => Ok(self.make_token(TOKEN::LEFT_PAREN)),
            ')' => Ok(self.make_token(TOKEN::RIGHT_PAREN)),
            '{' => Ok(self.make_token(TOKEN::LEFT_BRACE)),
            '}' => Ok(self.make_token(TOKEN::RIGHT_BRACE)),
            ';' => Ok(self.make_token(TOKEN::SEMICOLON)),
            ',' => Ok(self.make_token(TOKEN::COMMA)),
            '.' => Ok(self.make_token(TOKEN::DOT)),
            '-' => Ok(self.make_token(TOKEN::MINUS)),
            '+' => Ok(self.make_token(TOKEN::PLUS)),
            '/' => Ok(self.make_token(TOKEN::SLASH)),
            '*' => Ok(self.make_token(TOKEN::STAR)),
            '!' => Ok(match_token!(self, '=', TOKEN::BANG_EQUAL, TOKEN::BANG)),
            '=' => Ok(match_token!(self, '=', TOKEN::EQUAL_EQUAL, TOKEN::EQUAL)),
            '<' => Ok(match_token!(self, '=', TOKEN::LESS_EQUAL, TOKEN::LESS)),
            '>' => Ok(match_token!(self, '=', TOKEN::GREATER_EQUAL, TOKEN::GREATER)),
            '"' => Ok(self.string()?),
            digit!() => Ok(self.number()),
            alpha!() => Ok(self.identifier()),
            _ => Err(self.error_token("Unexpected character."))
        }

    }

    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }

    fn make_token(&self, ty: TOKEN) -> Token {
        Token {ty: ty, source: &self.source,  start: self.start, length: self.current - self.start, line: self.line}
    }

    fn error_token(&self, message: &str) -> TokenError {
        TokenError { message: message.to_owned(), line: self.line }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                Some(' ' | '\r' | '\t') => {self.advance();},
                Some('\n') => {
                    self.line += 1;
                    self.advance();
                },
                Some('/') => if let Some('/') = self.peek_next() {
                    while Some('\n') != self.peek() && None != self.peek() {self.advance();}
                } else {
                    return;
                },
                _ => return,
            }
        }
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }

    fn string(&mut self) -> TokenResult {
        while let Some(c) = self.peek() {
            match c {
                '"' => break,
                '\n' => self.line += 1,
                _ => (),
            };
            self.advance();
        }

        if let None = self.peek() {
            Err(self.error_token("Unterminated string."))
        } else {
            self.advance();
            Ok(self.make_token(TOKEN::STRING))
        }
    }

    fn number(&mut self) -> Token {
        while let Some(digit!()) = self.peek() {
            self.advance();
        }
        if let (Some('.'), Some(digit!())) = (self.peek(), self.peek_next()) {
            self.advance();
            while let Some(digit!()) = self.peek() {
                self.advance();
            }
        }

        self.make_token(TOKEN::NUMBER)
    }

    fn identifier(&mut self) -> Token {
        while let Some(alpha!() | digit!()) = self.peek() {
            self.advance();
        }
        let ty = match &self.source[self.start..self.current] {
            "and" => TOKEN::AND,
            "class" => TOKEN::CLASS,
            "else" => TOKEN::ELSE,
            "false" => TOKEN::FALSE,
            "for" => TOKEN::FOR,
            "fun" => TOKEN::FUN,
            "if" => TOKEN::IF,
            "nil" => TOKEN::NIL,
            "or" => TOKEN::OR,
            "print" => TOKEN::PRINT,
            "return" => TOKEN::RETURN,
            "super" => TOKEN::SUPER,
            "this" => TOKEN::THIS,
            "true" => TOKEN::TRUE,
            "var" => TOKEN::VAR,
            "while" => TOKEN::WHILE,
            _ => TOKEN::IDENTIFIER,
        };

        self.make_token(ty)
    }
}

