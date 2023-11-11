use crate::options::parser::Opts;
use crate::options::vars::EZA_STDIN_SEPARATOR;
use crate::options::Vars;
use std::ffi::OsString;
use std::io;
use std::io::IsTerminal;

#[derive(Debug, PartialEq)]
pub enum FilesInput {
    Stdin(OsString),
    Args,
}

impl FilesInput {
    pub fn deduce<V: Vars>(matches: &Opts, vars: &V) -> Self {
        if io::stdin().is_terminal() || matches.stdin == 0 {
            FilesInput::Args
        } else if matches.stdin > 0 && !io::stdin().is_terminal() {
            let separator = vars
                .get(EZA_STDIN_SEPARATOR)
                .unwrap_or(OsString::from("\n"));
            FilesInput::Stdin(separator)
        } else {
            FilesInput::Args
        }
    }
}
