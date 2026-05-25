// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
use std::io::{self, Write};

pub fn print_completions(shell: &str, dest: &mut dyn Write) -> io::Result<()> {
    let script = match shell {
        "bash" => include_str!("../../completions/bash/eza"),
        "fish" => include_str!("../../completions/fish/eza.fish"),
        "zsh" => include_str!("../../completions/zsh/_eza"),
        "powershell" | "pwsh" => include_str!("../../completions/pwsh/_eza.ps1"),
        "nushell" | "nu" => include_str!("../../completions/nush/eza.nu"),
        other => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("unknown shell for completions: {other}"),
            ));
        }
    };
    dest.write_all(script.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn fish_completions_contain_complete_directive() {
        let mut buf = Cursor::new(Vec::new());
        print_completions("fish", &mut buf).unwrap();
        let out = String::from_utf8(buf.into_inner()).unwrap();
        assert!(out.contains("complete -c eza"));
    }

    #[test]
    fn unknown_shell_is_rejected() {
        let mut buf = Cursor::new(Vec::new());
        assert!(print_completions("nope", &mut buf).is_err());
    }
}
