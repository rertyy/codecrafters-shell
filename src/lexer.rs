use crate::enums::{Operator, Token};

#[derive(Debug, Clone)]
enum LexerState {
    Normal,
    InSingleQuote,
    InDoubleQuote,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_state: LexerState,
    current_token: String,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            current_state: LexerState::Normal,
            current_token: String::new(),
            tokens: Vec::new(),
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        while self.position < self.input.len() {
            let c = self.input[self.position];
            self.handle_char(c);
            self.advance();
        }

        if !self.current_token.is_empty() {
            self.emit_token();
        }

        std::mem::take(&mut self.tokens)
    }

    fn handle_char(&mut self, c: char) {
        // println!("{}, {}, {:?}", c, self.current_token, self.current_state);
        match self.current_state {
            LexerState::Normal => self.handle_normal_char(c),
            LexerState::InSingleQuote => self.handle_single_quote_char(c),
            LexerState::InDoubleQuote => self.handle_double_quote_char(c),
        }
    }

    fn handle_normal_char(&mut self, c: char) {
        match c {
            c if c.is_whitespace() => {
                self.emit_token();
            }
            '&' => {
                self.emit_token();
                self.current_token.push(c);
                if let Some('&') = self.peek() {
                    self.advance();
                    self.current_token.push(c);
                }
                self.emit_token();
            }

            '|' => {
                self.emit_token();
                self.current_token.push(c);
                if let Some('|') = self.peek() {
                    self.advance();
                    self.current_token.push(c);
                }
                self.emit_token();
            }

            '>' => {
                if !self.current_token.is_empty()
                    && self
                        .current_token
                        .chars()
                        .all(|ch| char::is_ascii_digit(&ch))
                {
                    // 1> or 2>>
                    self.current_token.push(c);
                    if let Some('>') = self.peek() {
                        self.advance();
                        self.current_token.push(c);
                    }
                    self.emit_token();
                } else {
                    // word boundary -> start of new operator
                    self.emit_token();
                    self.current_token.push(c);
                    if let Some('>') = self.peek() {
                        self.advance();
                        self.current_token.push(c);
                    }
                    self.emit_token();
                }
            }

            '\'' => {
                self.current_state = LexerState::InSingleQuote;
            }
            '"' => {
                self.current_state = LexerState::InDoubleQuote;
            }
            '\\' => {
                if let Some(&next) = self.peek() {
                    // in normal, backslash escapes everything
                    self.current_token.push(next);
                    self.advance();
                    return;
                }
                self.current_token.push(c);
            }
            _ => self.current_token.push(c),
        }
    }
    fn handle_single_quote_char(&mut self, c: char) {
        // Single quotes don't have escape sequences
        match c {
            '\'' => {
                self.current_state = LexerState::Normal;
            }
            _ => self.current_token.push(c),
        }
    }

    fn handle_double_quote_char(&mut self, c: char) {
        match c {
            '"' => {
                self.current_state = LexerState::Normal;
            }
            '\\' => {
                if let Some(next) = self.peek() {
                    // if the next char is ' or \", escape those
                    // else, add \
                    let esc_chars = vec!['\"', '\\'];
                    if esc_chars.contains(next) {
                        self.advance();
                        self.current_token.push(self.input[self.position]);
                        return;
                    }
                }
                self.current_token.push(c);
            }
            _ => self.current_token.push(c),
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.get(self.position + 1)
    }

    fn emit_token(&mut self) {
        if self.current_token.is_empty() {
            return;
        }
        match self.current_state {
            LexerState::Normal => {
                if let Ok(operator) = self.current_token.parse::<Operator>() {
                    self.tokens.push(Token::Operator(operator))
                } else {
                    self.tokens.push(Token::Word(self.current_token.clone()))
                }
            }
            LexerState::InSingleQuote | LexerState::InDoubleQuote => {
                self.tokens.push(Token::Word(self.current_token.clone()));
            }
        }
        self.current_token.clear();
    }
}
