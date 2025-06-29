use crate::enums::Command;
use crate::util;
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

        // TODO: refactor to get executables at the start of the line rather than every completion
        let mut commands: Vec<String> = Command::get_builtins();
        let executables = util::get_path_executables();
        let mut path_exe_strings = util::get_path_exe_strings(executables);
        commands.append(&mut path_exe_strings);

        commands.sort();
        commands.dedup();

        let matching: Vec<_> = commands
            .into_iter()
            .filter(|cmd| cmd.starts_with(word))
            .collect();

        // println!("{:?}", matching);

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
