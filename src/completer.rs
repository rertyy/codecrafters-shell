use crate::enums::Command;
use rustyline::completion::{Completer, Pair};
use rustyline::{completion, Context};
use rustyline_derive::{Helper, Highlighter, Hinter, Validator};

#[derive(Helper, Highlighter, Hinter, Validator)]
pub struct MyHelper {
    // #[rustyline(Completer)]
    // pub completer: FilenameCompleter,
    // #[rustyline(Highlighter)]
    // pub highlighter: MatchingBracketHighlighter,
    // #[rustyline(Validator)]
    // pub validator: MatchingBracketValidator,
    // #[rustyline(Hinter)]
    // pub hinter: HistoryHinter,
}

impl Completer for MyHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let is_break = |c: char| c.is_ascii_whitespace();
        let (start, word) = completion::extract_word(line, pos, Some('\\'), is_break);

        let mut commands: Vec<String> = Vec::new();
        commands.append(&mut Command::get_builtins());

        let matching: Vec<_> = commands
            .into_iter()
            .filter(|cmd| cmd.starts_with(word))
            .collect();

        if matching.len() == 1 {
            let disp = &matching[0];
            let repl = format!("{} ", disp);
            let pair = Pair {
                display: repl.to_string(),
                replacement: repl,
            };
            Ok((start, vec![pair]))
        } else {
            let matches = matching
                .into_iter()
                .map(|cmd| Pair {
                    display: cmd.clone(),
                    replacement: cmd.clone(),
                })
                .collect();

            Ok((start, matches))
        }
    }
}
