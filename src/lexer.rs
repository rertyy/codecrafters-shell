use crate::enums::Token;

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
            self.position += 1;
        }

        if !self.current_token.is_empty() {
            self.emit_token();
        }

        std::mem::take(&mut self.tokens)
    }

    fn handle_char(&mut self, c: char) {
        match self.current_state {
            LexerState::Normal => {
                if c.is_whitespace() {
                    if !self.current_token.is_empty() {
                        self.emit_token();
                    }
                } else if c == '\'' {
                    if !self.current_token.is_empty() {
                        self.emit_token();
                    }
                    self.current_state = LexerState::InSingleQuote;
                } else if c == '"' {
                    if !self.current_token.is_empty() {
                        self.emit_token();
                    }
                    self.current_state = LexerState::InDoubleQuote;
                } else if c == '\\' {
                    self.current_state = LexerState::Escape;
                } else {
                    self.current_token.push(c);
                }
            }
            LexerState::InSingleQuote => {
                // Single quotes don't have escape sequences
                if c == '\'' {
                    self.emit_token();
                    self.current_state = LexerState::Normal;
                } else {
                    self.current_token.push(c);
                }
            }
            LexerState::InDoubleQuote => {
                if c == '"' {
                    self.emit_token();
                    self.current_state = LexerState::Normal;
                } else if c == '\\' {
                    self.current_state = LexerState::Escape;
                } else {
                    self.current_token.push(c);
                }
            }
            LexerState::Escape => {
                self.current_token.push(c);
                self.current_state = LexerState::Normal;
            }
        }
    }

    fn emit_token(&mut self) {
        match self.current_state {
            LexerState::Normal => match self.current_token.parse::<Token>() {
                Ok(token) => self.tokens.push(token),
                Err(_) => self.tokens.push(Token::Word(self.current_token.clone())),
            },
            LexerState::InSingleQuote | LexerState::InDoubleQuote => {
                self.tokens
                    .push(Token::StringLiteral(self.current_token.clone()));
            }
            _ => panic!("Unexpected state when emitting token"),
        }
        self.current_token.clear();
    }
}
