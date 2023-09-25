use crate::fs::feature::xattr;
use crate::options::parser::Opts;
use crate::options::{NumberSource, OptionsError, Vars};
use crate::output::file_name::Options as FileStyle;
use crate::output::grid_details::{self, RowThreshold};
use crate::output::table::{Columns, Options as TableOptions, SizeFormat, TimeTypes, UserFormat};
use crate::output::time::TimeFormat;
use crate::output::{details, grid, Mode, TerminalWidth, View};

impl View {
    pub fn deduce<V: Vars>(
        matches: &Opts,
        vars: &V,
        strictness: bool,
    ) -> Result<Self, OptionsError> {
        let mode = Mode::deduce(matches, vars, strictness)?;
        let width = TerminalWidth::deduce(matches, vars)?;
        let file_style = FileStyle::deduce(matches, vars)?;
        let deref_links = matches.dereference > 0;
        Ok(Self {
            mode,
            width,
            file_style,
            deref_links,
        })
    }
}

impl Mode {
    /// Determine which viewing mode to use based on the user’s options.
    ///
    /// As with the other options, arguments are scanned right-to-left and the
    /// first flag found is matched, so `exa --oneline --long` will pick a
    /// details view, and `exa --long --oneline` will pick the lines view.
    ///
    /// This is complicated a little by the fact that `--grid` and `--tree`
    /// can also combine with `--long`, so care has to be taken to use the
    pub fn deduce<V: Vars>(
        matches: &Opts,
        vars: &V,
        strictness: bool,
    ) -> Result<Self, OptionsError> {
        let flag = matches.long > 0 || matches.oneline > 0 || matches.grid > 0 || matches.tree > 0;

        if !flag {
            Self::strict_check_long_flags(matches, strictness)?;
            let grid = grid::Options::deduce(matches);
            return Ok(Self::Grid(grid));
        };

        if matches.long > 0 {
            let details = details::Options::deduce_long(matches, vars, strictness)?;

            let flags = matches.grid > 0 || (matches.tree > 0);

            if flags && matches.grid > 0 {
                let grid = grid::Options::deduce(matches);
                let row_threshold = RowThreshold::deduce(vars)?;
                let grid_details = grid_details::Options {
                    grid,
                    details,
                    row_threshold,
                };
                return Ok(Self::GridDetails(grid_details));
            }

            // the --tree case is handled by the DirAction parser later
            return Ok(Self::Details(details));
        }

        Self::strict_check_long_flags(matches, strictness)?;

        if matches.tree > 0 {
            let details = details::Options::deduce_tree(matches);
            return Ok(Self::Details(details));
        }

        if matches.oneline > 0 {
            return Ok(Self::Lines);
        }

        let grid = grid::Options::deduce(matches);
        Ok(Self::Grid(grid))
    }

    fn strict_check_long_flags(matches: &Opts, strictness: bool) -> Result<(), OptionsError> {
        // If --long hasn’t been passed, then check if we need to warn the
        // user about flags that won’t have any effect.
        // TODO strict handling
        if strictness && matches.long == 0 {
            if matches.tree > 0 {
                return Err(OptionsError::Useless(
                    "--tree".to_string(),
                    false,
                    "--long".to_string(),
                ));
            } else if matches.binary > 0 {
                return Err(OptionsError::Useless(
                    "--binary".to_string(),
                    false,
                    "--long".to_string(),
                ));
            } else if matches.bytes > 0 {
                return Err(OptionsError::Useless(
                    "--bytes".to_string(),
                    false,
                    "--long".to_string(),
                ));
            } else if matches.inode > 0 {
                return Err(OptionsError::Useless(
                    "--inode".to_string(),
                    false,
                    "--long".to_string(),
                ));
            } else if matches.links > 0 {
                return Err(OptionsError::Useless(
                    "--links".to_string(),
                    false,
                    "--long".to_string(),
                ));
            } else if matches.header > 0 {
                return Err(OptionsError::Useless(
                    "--header".to_string(),
                    false,
                    "--long".to_string(),
                ));
            } else if matches.blocksize > 0 {
                return Err(OptionsError::Useless(
                    "--blocksize".to_string(),
                    false,
                    "--long".to_string(),
                ));
            } else if matches.time.is_some() {
                return Err(OptionsError::Useless(
                    "--time".to_string(),
                    false,
                    "--long".to_string(),
                ));
            } else if matches.group > 0 {
                return Err(OptionsError::Useless(
                    "--group".to_string(),
                    false,
                    "--long".to_string(),
                ));
            } else if matches.numeric > 0 {
                return Err(OptionsError::Useless(
                    "--numeric".to_string(),
                    false,
                    "--long".to_string(),
                ));
            } else if matches.mount > 0 {
                return Err(OptionsError::Useless(
                    "--mount".to_string(),
                    false,
                    "--long".to_string(),
                ));
            }
        }
        Ok(())
    }
}

impl grid::Options {
    fn deduce(matches: &Opts) -> Self {
        grid::Options {
            across: matches.across > 0,
        }
    }
}

impl details::Options {
    fn deduce_tree(matches: &Opts) -> Self {
        details::Options {
            table: None,
            header: false,
            xattr: xattr::ENABLED && matches.extended > 0,
            secattr: xattr::ENABLED && matches.security_context > 0,
            mounts: matches.mount > 0,
        }
    }

    fn deduce_long<V: Vars>(
        matches: &Opts,
        vars: &V,
        strictness: bool,
    ) -> Result<Self, OptionsError> {
        if strictness {
            if matches.across > 0 && !matches.grid > 0 {
                return Err(OptionsError::Useless(
                    "--accros".to_string(),
                    true,
                    "--long".to_string(),
                ));
            } else if matches.oneline > 0 {
                return Err(OptionsError::Useless(
                    "--oneline".to_string(),
                    true,
                    "--long".to_string(),
                ));
            }
        }

        Ok(details::Options {
            table: Some(TableOptions::deduce(matches, vars)?),
            header: matches.header > 0,
            xattr: xattr::ENABLED && matches.extended > 0,
            secattr: xattr::ENABLED && matches.security_context > 0,
            mounts: matches.mount > 0,
        })
    }
}

impl TerminalWidth {
    fn deduce<V: Vars>(matches: &Opts, vars: &V) -> Result<Self, OptionsError> {
        use crate::options::vars;

        if let Some(width) = matches.width {
            if width == 0 {
                return Ok(Self::Automatic);
            }
            Ok(Self::Set(width))
        } else if let Some(columns) = vars.get(vars::COLUMNS).and_then(|s| s.into_string().ok()) {
            match columns.parse() {
                Ok(width) => Ok(Self::Set(width)),
                Err(e) => {
                    let source = NumberSource::Var(vars::COLUMNS.to_string());
                    Err(OptionsError::FailedParse(columns, source, e))
                }
            }
        } else {
            Ok(Self::Automatic)
        }
    }
}

impl RowThreshold {
    fn deduce<V: Vars>(vars: &V) -> Result<Self, OptionsError> {
        use crate::options::vars;

        if let Some(columns) = vars
            .get_with_fallback(vars::EZA_GRID_ROWS, vars::EXA_GRID_ROWS)
            .and_then(|s| s.into_string().ok())
        {
            match columns.parse() {
                Ok(rows) => Ok(Self::MinimumRows(rows)),
                Err(e) => {
                    let source = NumberSource::Var(vars::EXA_GRID_ROWS.to_string());
                    Err(OptionsError::FailedParse(columns, source, e))
                }
            }
        } else {
            Ok(Self::AlwaysGrid)
        }
    }
}

impl TableOptions {
    fn deduce<V: Vars>(matches: &Opts, vars: &V) -> Result<Self, OptionsError> {
        let time_format = TimeFormat::deduce(matches, vars)?;
        let size_format = SizeFormat::deduce(matches);
        let user_format = UserFormat::deduce(matches);
        let columns = Columns::deduce(matches)?;
        Ok(Self {
            size_format,
            time_format,
            user_format,
            columns,
        })
    }
}

impl Columns {
    fn deduce(matches: &Opts) -> Result<Self, OptionsError> {
        let time_types = TimeTypes::deduce(matches)?;

        let git = matches.git > 0 && matches.no_git == 0;
        let subdir_git_repos = matches.git_repos > 0 && matches.no_git == 0;
        let subdir_git_repos_no_stat =
            !subdir_git_repos && matches.git_repos_no_status > 0 && matches.no_git == 0;

        let blocksize = matches.blocksize > 0;
        let group = matches.group > 0;
        let inode = matches.inode > 0;
        let links = matches.links > 0;
        let octal = matches.octal > 0;
        let security_context = xattr::ENABLED && matches.security_context > 0;

        let permissions = matches.no_permissions == 0;
        let filesize = matches.no_filesize == 0;
        let user = matches.no_user == 0;

        Ok(Self {
            time_types,
            inode,
            links,
            blocksize,
            group,
            git,
            subdir_git_repos,
            subdir_git_repos_no_stat,
            octal,
            security_context,
            permissions,
            filesize,
            user,
        })
    }
}

impl SizeFormat {
    /// Determine which file size to use in the file size column based on
    /// the user’s options.
    ///
    /// The default mode is to use the decimal prefixes, as they are the
    /// most commonly-understood, and don’t involve trying to parse large
    /// strings of digits in your head. Changing the format to anything else
    /// involves the `--binary` or `--bytes` flags, and these conflict with
    /// each other.
    fn deduce(matches: &Opts) -> Self {
        let flag = matches.binary > 0 || matches.bytes > 0;

        if flag {
            if matches.binary > 0 {
                Self::BinaryBytes
            } else if matches.bytes > 0 {
                Self::JustBytes
            } else {
                Self::DecimalBytes
            }
        } else {
            Self::DecimalBytes
        }
    }
}

impl TimeFormat {
    /// Determine how time should be formatted in timestamp columns.
    fn deduce<V: Vars>(matches: &Opts, vars: &V) -> Result<Self, OptionsError> {
        let word = if let Some(ref w) = matches.time_style {
            w.clone()
        } else {
            use crate::options::vars;
            match vars.get(vars::TIME_STYLE) {
                Some(ref t) if !t.is_empty() => t.clone(),
                _ => return Ok(Self::DefaultFormat),
            }
        };

        match word.to_string_lossy().as_ref() {
            "default" => Ok(Self::DefaultFormat),
            "relative" => Ok(Self::Relative),
            "iso" => Ok(Self::ISOFormat),
            "long-iso" => Ok(Self::LongISO),
            "full-iso" => Ok(Self::FullISO),
            _ => Err(OptionsError::BadArgument(
                "--time-style".to_string(),
                word.to_string_lossy().to_string(),
            )),
        }
    }
}

impl UserFormat {
    fn deduce(matches: &Opts) -> Self {
        let flag = matches.numeric > 0;
        if flag {
            Self::Numeric
        } else {
            Self::Name
        }
    }
}

impl TimeTypes {
    /// Determine which of a file’s time fields should be displayed for it
    /// based on the user’s options.
    ///
    /// There are two separate ways to pick which fields to show: with a
    /// flag (such as `--modified`) or with a parameter (such as
    /// `--time=modified`). An error is signaled if both ways are used.
    ///
    /// It’s valid to show more than one column by passing in more than one
    /// option, but passing *no* options means that the user just wants to
    /// see the default set.
    fn deduce(matches: &Opts) -> Result<Self, OptionsError> {
        let possible_word = &matches.time;
        let modified = matches.modified > 0;
        let changed = matches.changed > 0;
        let accessed = matches.accessed > 0;
        let created = matches.created > 0;

        let no_time = matches.no_time > 0;

        #[rustfmt::skip]
        let time_types = if no_time {
            Self {
                modified: false,
                changed: false,
                accessed: false,
                created: false,
            }
        } else if let Some(word) = possible_word {
            if modified {
                return Err(OptionsError::Useless("--modified".to_string(), true, "--time".to_string()));
            }
            else if changed {
                return Err(OptionsError::Useless("--changed".to_string(), true, "--time".to_string()));
            }
            else if accessed {
                return Err(OptionsError::Useless("--accessed".to_string(), true, "--time".to_string()));
            }
            else if created {
                return Err(OptionsError::Useless("--created".to_string(), true, "--time".to_string()));
            }
            else if word == "mod" || word == "modified" {
                Self { modified: true,  changed: false, accessed: false, created: false }
            } else if word == "ch" || word == "changed" {
                Self { modified: false, changed: true,  accessed: false, created: false }
            } else if word == "acc" || word == "accessed" {
                Self { modified: false, changed: false, accessed: true,  created: false }
            } else if word == "cr" || word == "created" {
                Self { modified: false, changed: false, accessed: false, created: true  }
            }
            else {
                return Err(OptionsError::BadArgument("--time".to_string(), word.to_string_lossy().to_string()));
            }
        } else if modified || changed || accessed || created {
            Self {
                modified,
                changed,
                accessed,
                created,
            }
        } else {
            Self::default()
        };

        Ok(time_types)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::ffi::OsString;

    #[test]
    fn deduce_time_type() {
        let matches = Opts { ..Opts::default() };

        assert_eq!(TimeTypes::deduce(&matches).unwrap(), TimeTypes::default());
    }

    #[test]
    fn deduce_time_type_modified() {
        let matches = Opts {
            modified: 1,
            ..Opts::default()
        };

        assert_eq!(
            TimeTypes::deduce(&matches).unwrap(),
            TimeTypes {
                modified: true,
                ..TimeTypes::default()
            }
        );
    }

    #[test]
    fn deduce_time_type_changed() {
        let matches = Opts {
            changed: 1,
            ..Opts::default()
        };

        assert_eq!(
            TimeTypes::deduce(&matches).unwrap(),
            TimeTypes {
                changed: true,
                modified: false,
                ..TimeTypes::default()
            }
        );
    }

    #[test]
    fn deduce_time_type_accessed() {
        let matches = Opts {
            accessed: 1,
            ..Opts::default()
        };

        assert_eq!(
            TimeTypes::deduce(&matches).unwrap(),
            TimeTypes {
                accessed: true,
                modified: false,
                ..TimeTypes::default()
            }
        );
    }

    #[test]
    fn deduce_time_type_created() {
        let matches = Opts {
            created: 1,
            ..Opts::default()
        };

        assert_eq!(
            TimeTypes::deduce(&matches).unwrap(),
            TimeTypes {
                created: true,
                modified: false,
                ..TimeTypes::default()
            }
        );
    }

    #[test]
    fn deduce_time_type_mod_string() {
        let matches = Opts {
            time: Some(OsString::from("mod")),
            ..Opts::default()
        };

        assert_eq!(
            TimeTypes::deduce(&matches).unwrap(),
            TimeTypes {
                modified: true,
                ..TimeTypes::default()
            }
        );
    }

    #[test]
    fn deduce_time_type_ch_string() {
        let matches = Opts {
            time: Some(OsString::from("ch")),
            ..Opts::default()
        };

        assert_eq!(
            TimeTypes::deduce(&matches).unwrap(),
            TimeTypes {
                changed: true,
                modified: false,
                ..TimeTypes::default()
            }
        );
    }

    #[test]
    fn deduce_time_type_acc_string() {
        let matches = Opts {
            time: Some(OsString::from("acc")),
            ..Opts::default()
        };

        assert_eq!(
            TimeTypes::deduce(&matches).unwrap(),
            TimeTypes {
                accessed: true,
                modified: false,
                ..TimeTypes::default()
            }
        );
    }

    #[test]
    fn deduce_time_type_cr_string() {
        let matches = Opts {
            time: Some(OsString::from("cr")),
            ..Opts::default()
        };

        assert_eq!(
            TimeTypes::deduce(&matches).unwrap(),
            TimeTypes {
                created: true,
                modified: false,
                ..TimeTypes::default()
            }
        );
    }

    #[test]
    fn deduce_time_type_useless_mod() {
        let matches = Opts {
            time: Some(OsString::from("mod")),
            modified: 1,
            ..Opts::default()
        };

        assert!(TimeTypes::deduce(&matches).is_err());
    }

    #[test]
    fn deduce_time_type_useless_ch() {
        let matches = Opts {
            time: Some(OsString::from("ch")),
            changed: 1,
            ..Opts::default()
        };

        assert!(TimeTypes::deduce(&matches).is_err());
    }

    #[test]
    fn deduce_time_type_useless_acc() {
        let matches = Opts {
            time: Some(OsString::from("acc")),
            accessed: 1,
            ..Opts::default()
        };

        assert!(TimeTypes::deduce(&matches).is_err());
    }

    #[test]
    fn deduce_time_type_useless_cr() {
        let matches = Opts {
            time: Some(OsString::from("cr")),
            created: 1,
            ..Opts::default()
        };

        assert!(TimeTypes::deduce(&matches).is_err());
    }

    #[test]
    fn deduce_user_format() {
        let matches = Opts { ..Opts::default() };

        assert_eq!(UserFormat::deduce(&matches), UserFormat::Name);
    }

    #[test]
    fn deduce_user_format_numeric() {
        let matches = Opts {
            numeric: 1,
            ..Opts::default()
        };

        assert_eq!(UserFormat::deduce(&matches), UserFormat::Numeric);
    }

    #[test]
    fn deduce_size_format() {
        let matches = Opts { ..Opts::default() };

        assert_eq!(SizeFormat::deduce(&matches), SizeFormat::DecimalBytes);
    }

    #[test]
    fn deduce_size_format_binary() {
        let matches = Opts {
            binary: 1,
            ..Opts::default()
        };

        assert_eq!(SizeFormat::deduce(&matches), SizeFormat::BinaryBytes);
    }

    #[test]
    fn deduce_size_format_bytes() {
        let matches = Opts {
            bytes: 1,
            ..Opts::default()
        };

        assert_eq!(SizeFormat::deduce(&matches), SizeFormat::JustBytes);
    }

    #[test]
    fn deduce_dtails_tree() {
        let matches = Opts {
            tree: 1,
            ..Opts::default()
        };

        assert_eq!(
            details::Options::deduce_tree(&matches),
            details::Options {
                table: None,
                header: false,
                xattr: xattr::ENABLED && matches.extended > 0,
                secattr: xattr::ENABLED && matches.security_context > 0,
                mounts: matches.mount > 0,
            }
        );
    }
}
