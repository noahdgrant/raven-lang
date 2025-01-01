use std::convert::TryFrom;
pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn advance(&mut self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            let c = self.source[self.current..].chars().next().unwrap();
            self.current += c.len_utf8();
            Some(c)
        }
    }

    pub fn get_lexeme(&self) -> &str {
        &self.source[self.start..self.current]
    }

    fn string_token(&mut self) -> Result<Token, Token> {
        while let Some(c) = self.peek() {
            if c == '"' {
                break;
            }
            if c == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(Token::new(
                "Unterminated string",
                TokenType::Errorr,
                self.line,
            ));
        };

        self.advance();
        Ok(Token::new(self.get_lexeme(), TokenType::String, self.line))
    }

    fn number_token(&mut self) -> Token {
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        if let Some(c) = self.peek() {
            if let Some(n) = self.peek_next() {
                // Check for decimal number
                if c == '.' && n.is_ascii_digit() {
                    self.advance();
                }

                while let Some(d) = self.peek() {
                    if d.is_ascii_digit() {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
        }

        Token::new(self.get_lexeme(), TokenType::Number, self.line)
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn check_keyword(
        &self,
        start: usize,
        length: usize,
        rest: &str,
        token_type: TokenType,
    ) -> TokenType {
        if self.current - self.start == start + length
            && self.get_lexeme()[start..start + length] == *rest
        {
            return token_type;
        }
        TokenType::Identifier
    }

    fn identifier_type(&self) -> TokenType {
        match self.get_lexeme().chars().nth(0).unwrap() {
            // TODO: figure out how to make "else if" work
            'a' => self.check_keyword(1, 2, "nd", TokenType::And),
            'e' => self.check_keyword(1, 3, "lse", TokenType::Else),
            'f' => match self.get_lexeme().chars().nth(1) {
                Some('a') => self.check_keyword(2, 3, "lse", TokenType::False),
                Some('o') => self.check_keyword(2, 1, "r", TokenType::For),
                Some('u') => self.check_keyword(2, 6, "nction", TokenType::Function),
                _ => TokenType::Identifier,
            },
            'i' => match self.get_lexeme().chars().nth(1) {
                Some('f') => self.check_keyword(2, 0, "", TokenType::If),
                Some('n') => self.check_keyword(2, 0, "", TokenType::In),
                Some('m') => self.check_keyword(2, 4, "port", TokenType::Import),
                _ => TokenType::Identifier,
            },
            'l' => self.check_keyword(1, 2, "et", TokenType::Let),
            'm' => match self.get_lexeme().chars().nth(1) {
                Some('a') => self.check_keyword(2, 3, "tch", TokenType::Match),
                Some('u') => self.check_keyword(2, 5, "table", TokenType::Mutable),
                _ => TokenType::Identifier,
            },
            'n' => self.check_keyword(1, 2, "ot", TokenType::Not),
            'o' => self.check_keyword(1, 1, "r", TokenType::Or),
            'p' => self.check_keyword(1, 5, "ublic", TokenType::Public),
            't' => self.check_keyword(1, 3, "rue", TokenType::True),
            's' => match self.get_lexeme().chars().nth(1) {
                Some('e') => self.check_keyword(2, 2, "lf", TokenType::Selff),
                Some('t') => self.check_keyword(2, 4, "ruct", TokenType::Struct),
                _ => TokenType::Identifier,
            },
            'r' => self.check_keyword(1, 5, "eturn", TokenType::Return),
            'w' => self.check_keyword(1, 4, "hile", TokenType::While),
            _ => TokenType::Identifier,
        }
    }

    fn identifier_token(&mut self) -> Token {
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || self.is_alpha(c) {
                self.advance();
            } else {
                break;
            }
        }
        Token::new(self.get_lexeme(), self.identifier_type(), self.line)
    }

    pub fn get_token(&mut self) -> Result<Token, Token> {
        self.skip_whitespace();
        self.reset_start();

        match self.advance() {
            Some(c) => {
                if self.is_alpha(c) {
                    return Ok(self.identifier_token());
                }
                if c.is_ascii_digit() {
                    return Ok(self.number_token());
                }

                match TokenType::try_from(c) {
                    Ok(TokenType::LeftParen) => Ok(Token::new(
                        self.get_lexeme(),
                        TokenType::LeftParen,
                        self.line,
                    )),
                    Ok(TokenType::RightParen) => Ok(Token::new(
                        self.get_lexeme(),
                        TokenType::RightParen,
                        self.line,
                    )),
                    Ok(TokenType::LeftBrace) => Ok(Token::new(
                        self.get_lexeme(),
                        TokenType::LeftBrace,
                        self.line,
                    )),
                    Ok(TokenType::RightBrace) => Ok(Token::new(
                        self.get_lexeme(),
                        TokenType::RightBrace,
                        self.line,
                    )),
                    Ok(TokenType::Comma) => {
                        Ok(Token::new(self.get_lexeme(), TokenType::Comma, self.line))
                    }
                    Ok(TokenType::Dot) => {
                        Ok(Token::new(self.get_lexeme(), TokenType::Dot, self.line))
                    }
                    Ok(TokenType::Minus) => {
                        Ok(self.compound_token(TokenType::Minus, '=', TokenType::MinusEqual))
                    }
                    Ok(TokenType::Plus) => {
                        Ok(self.compound_token(TokenType::Plus, '=', TokenType::PlusEqual))
                    }
                    Ok(TokenType::Slash) => {
                        Ok(self.compound_token(TokenType::Slash, '=', TokenType::SlashEqual))
                    }
                    Ok(TokenType::Star) => {
                        Ok(self.compound_token(TokenType::Star, '=', TokenType::StarEqual))
                    }
                    Ok(TokenType::Bang) => {
                        Ok(self.compound_token(TokenType::Bang, '=', TokenType::BangEqual))
                    }
                    Ok(TokenType::Equal) => {
                        Ok(self.compound_token(TokenType::Equal, '=', TokenType::EqualEqual))
                    }
                    Ok(TokenType::Less) => {
                        Ok(self.compound_token(TokenType::Less, '=', TokenType::LessEqual))
                    }
                    Ok(TokenType::Greater) => {
                        Ok(self.compound_token(TokenType::Greater, '=', TokenType::GreaterEqual))
                    }
                    Ok(TokenType::DoubleQuote) => self.string_token(),
                    Ok(_) => panic!("Unexpected token - should be unreachable"),
                    // TODO: make this a real error type to clean this up
                    Err(_) => Err(Token::new(
                        "Unexpected character",
                        TokenType::Errorr,
                        self.line,
                    )),
                }
            }
            None => Ok(Token::new(self.get_lexeme(), TokenType::EOF, self.line)),
        }
    }

    fn compound_token(
        &mut self,
        token_type: TokenType,
        expected: char,
        matched_type: TokenType,
    ) -> Token {
        if self.peek() == Some(expected) {
            self.advance().unwrap();
            Token::new(self.get_lexeme(), matched_type, self.line)
        } else {
            Token::new(self.get_lexeme(), token_type, self.line)
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            self.source[self.current..].chars().next()
        }
    }

    pub fn peek_next(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            self.source[self.current..].chars().nth(1)
        }
    }

    pub fn reset_start(&mut self) {
        self.start = self.current;
    }

    fn skip_whitespace(&mut self) {
        loop {
            if let Some(c) = self.peek() {
                match c {
                    ' ' | '\r' | '\t' => {
                        self.advance();
                        break;
                    }
                    '\n' => {
                        self.line += 1;
                        self.advance();
                        break;
                    }
                    '#' => {
                        // A comment goes until the end of the line
                        while let Some(c) = self.peek() {
                            if c == '\n' {
                                break;
                            }
                            self.advance();
                        }
                    }
                    _ => break,
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    DoubleQuote,
    // One of two character tokens
    Minus,
    MinusEqual,
    Plus,
    PlusEqual,
    Slash,
    SlashEqual,
    Star,
    StarEqual,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals
    Identifier,
    String,
    Number,
    // Keywords
    And,
    Else,
    ElseIf,
    False,
    For,
    Function,
    If,
    Import,
    In,
    Let,
    Match,
    Mutable,
    Not,
    Or,
    Public,
    Return,
    Selff,
    Struct,
    True,
    While,
    Volatile,

    // Other
    EOF,
    Errorr,
}

impl TryFrom<char> for TokenType {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '(' => Ok(TokenType::LeftParen),
            ')' => Ok(TokenType::RightParen),
            '{' => Ok(TokenType::LeftBrace),
            '}' => Ok(TokenType::RightBrace),
            ',' => Ok(TokenType::Comma),
            '.' => Ok(TokenType::Dot),
            '-' => Ok(TokenType::Minus),
            '+' => Ok(TokenType::Plus),
            '/' => Ok(TokenType::Slash),
            '*' => Ok(TokenType::Star),
            '!' => Ok(TokenType::Bang),
            '=' => Ok(TokenType::Equal),
            '<' => Ok(TokenType::Less),
            '>' => Ok(TokenType::Greater),
            '"' => Ok(TokenType::DoubleQuote),
            _ => Err(()),
        }
    }
}

pub struct Token<'a> {
    pub lexeme: &'a str,
    pub typee: TokenType,
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(lexeme: &'a str, typee: TokenType, line: usize) -> Token<'a> {
        Token {
            lexeme,
            typee,
            line,
        }
    }
}
