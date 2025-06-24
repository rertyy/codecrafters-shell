use crate::enums::{Operator, Token};

#[derive(Debug, Clone)]
enum LexerState {
    Normal,
    InSingleQuote,
    InDoubleQuote,
    Escape,
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
        match self.current_state {
            LexerState::Normal => match c {
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
                    self.current_state = LexerState::Escape;
                }
                _ => self.current_token.push(c),
            },
            LexerState::InSingleQuote => {
                // Single quotes don't have escape sequences
                match c {
                    '\'' => {
                        self.current_state = LexerState::Normal;
                    }
                    _ => self.current_token.push(c),
                }
            }
            LexerState::InDoubleQuote => match c {
                '"' => {
                    self.current_state = LexerState::Normal;
                }
                '\\' => {
                    self.current_state = LexerState::Escape;
                }
                _ => self.current_token.push(c),
            },
            LexerState::Escape => {
                self.current_token.push(c);
                self.current_state = LexerState::Normal;
            }
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
            _ => unreachable!("Emit token"),
        }
        self.current_token.clear();
    }
}
