fn mainold() {
    loop {
        {
            // using crossterm is too much work...
            loop {
                if event::poll(Duration::from_millis(500)).unwrap() {
                    match event::read().unwrap() {
                        event::Event::Key(KeyEvent { code, .. }) => match code {
                            KeyCode::Tab => {}
                            KeyCode::Up => match history_index {
                                None => {
                                    if !history.is_empty() {
                                        history_index = Some(history.len() - 1);
                                        cached_edits.insert(history.len(), buffer.clone());
                                        buffer = cached_edits
                                            .get(&(history.len() - 1))
                                            .unwrap_or(&history[history.len() - 1])
                                            .clone();
                                    }
                                }
                                Some(0) => {
                                    util::render_prompt(&buffer);
                                }
                                Some(i) => {
                                    cached_edits.insert(i, buffer.clone());
                                    history_index = Some(i - 1);
                                    buffer = cached_edits
                                        .get(&(i - 1))
                                        .unwrap_or(&history[i - 1])
                                        .clone();
                                }
                            },
                            KeyCode::Down => match history_index {
                                None => (),
                                Some(i) => {
                                    if i < history.len() {
                                        cached_edits.insert(i, buffer.clone());
                                        history_index = Some(i + 1);
                                        buffer = cached_edits.get(&(i + 1)).unwrap().clone();
                                    }
                                    util::render_prompt(&buffer);
                                }
                            },
                            KeyCode::Char(c) => {
                                buffer.push(c);
                                util::render_prompt(&buffer);
                            }
                            KeyCode::Backspace => {
                                buffer.pop();
                                util::render_prompt(&buffer);
                            }
                            KeyCode::Enter => {
                                println!();
                                terminal::disable_raw_mode().unwrap();
                                buffer = buffer.trim().to_string();
                                if buffer.is_empty() {
                                    util::render_prompt("");
                                    break;
                                }

                                history.push(buffer.clone());

                                let mut lexer = Lexer::new(&*buffer);
                                let tokens = lexer.lex();
                                // println!("{:?}", tokens);

                                let mut parser = Parser::new(tokens);
                                let node = parser.parse();
                                // println!("{:?}", node);

                                match node {
                                    ASTNode::Command {
                                        name,
                                        args,
                                        redirections,
                                    } => run_command(name, &args, redirections),
                                    ASTNode::Pipeline(pipeline) => run_pipeline(pipeline),
                                }
                                buffer.clear();
                                util::render_prompt("");
                                terminal::enable_raw_mode().unwrap();
                                break;
                            }
                            _ => (),
                        },
                        _ => (),
                    }
                }
            }
        }
    }
}

pub(crate) fn render_prompt(buffer: &str) {
    use crossterm::{
        cursor,
        terminal::{Clear, ClearType},
        ExecutableCommand,
    };

    io::stdout()
        .execute(cursor::MoveToColumn(0))
        .unwrap()
        .execute(Clear(ClearType::CurrentLine))
        .unwrap();
    print!("$ {}", buffer);
    io::stdout().flush().unwrap();
}
