use crate::enums::Operator;
use crate::enums::Token;

#[derive(Debug)]
pub enum ASTNode {
    Command {
        name: String,
        args: Vec<String>,
        redirections: Vec<Redirection>,
    },
    Pipeline(Vec<ASTNode>),
}

#[derive(Debug)]
pub struct Redirection {
    pub fd: u8,
    pub direction: RedirectionType,
    pub target: String,
}

#[derive(Debug)]
pub enum RedirectionType {
    Input,
    Output,
    Append,
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn consume_token(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }
    pub fn parse(&mut self) -> ASTNode {
        let mut pipeline = Vec::new();
        loop {
            pipeline.push(self.parse_command());

            match self.peek_token() {
                Some(Token::Operator(Operator::Pipe)) => {
                    // | is not part of either command
                    self.consume_token();
                }
                _ => break,
            }
        }

        if pipeline.len() == 1 {
            // return the command directly
            pipeline.pop().unwrap()
        } else {
            ASTNode::Pipeline(pipeline)
        }
    }

    fn parse_command(&mut self) -> ASTNode {
        let mut name = None;
        let mut args = Vec::new();
        let mut redirs = Vec::new();

        while let Some(token) = self.peek_token() {
            match token {
                Token::Operator(Operator::Pipe) => break,
                Token::Operator(op) => {
                    let op = op.clone();
                    self.consume_token();
                    let target = match self.consume_token() {
                        Some(Token::Word(w)) => w.clone(),
                        _ => panic!("Expected target after redirection"),
                    };
                    let (fd, rtype) = match op {
                        Operator::Output(None) => (1, RedirectionType::Output),
                        Operator::Output(Some(x)) => (x, RedirectionType::Output),
                        Operator::Append(Some(x)) => (x, RedirectionType::Append),
                        Operator::Append(None) => (1, RedirectionType::Append),
                        Operator::Input => (0, RedirectionType::Input),
                        _ => unreachable!(),
                    };
                    redirs.push(Redirection {
                        fd,
                        direction: rtype,
                        target,
                    });
                }
                Token::Word(w) => {
                    let w = w.clone();
                    self.consume_token();
                    if name.is_none() {
                        name = Some(w);
                    } else {
                        args.push(w);
                    }
                }
            }
        }

        ASTNode::Command {
            name: name.expect("Expected command name"),
            args,
            redirections: redirs,
        }
    }
}
