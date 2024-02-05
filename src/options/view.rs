use clap::ValueEnum;

use crate::output::TerminalWidth::Automatic;

use crate::fs::feature::xattr;
use crate::options::parser::{ColorScaleModeArgs, Opts};
use crate::options::{vars, NumberSource, OptionsError, Vars};
use crate::output::color_scale::{ColorScaleMode, ColorScaleOptions};
use crate::output::file_name::Options as FileStyle;
use crate::output::grid_details::{self, RowThreshold};
use crate::output::table::{
    Columns, FlagsFormat, GroupFormat, Options as TableOptions, SizeFormat, TimeTypes, UserFormat,
};
use crate::output::time::TimeFormat;
use crate::output::TerminalWidth::Set;
use crate::output::{details, grid, Mode, TerminalWidth, View};

use super::parser::ColorScaleArgs;

impl View {
    pub fn deduce<V: Vars>(matches: &Opts, vars: &V, strict: bool) -> Result<Self, OptionsError> {
        let mode = Mode::deduce(matches, vars, strict)?;
        let deref_links = matches.dereference;
        let total_size = matches.total_size;
        let width = TerminalWidth::deduce(matches, vars)?;
        let file_style = FileStyle::deduce(matches, vars, width.actual_terminal_width().is_some())?;
        Ok(Self {
            mode,
            width,
            file_style,
            deref_links,
            total_size,
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
    pub fn deduce<V: Vars>(matches: &Opts, vars: &V, strict: bool) -> Result<Self, OptionsError> {
        if !(matches.long || matches.oneline || matches.grid || matches.tree) {
            if strict {
                Self::strict_check_long_flags(matches)?;
            }
            let grid = grid::Options::deduce(matches);
            return Ok(Self::Grid(grid));
        };

        if matches.long {
            let details = details::Options::deduce_long(matches, vars, strict)?;

            if matches.grid {
                let grid = grid::Options::deduce(matches);
                let row_threshold = RowThreshold::deduce(vars)?;
                let grid_details = grid_details::Options {
                    details,
                    row_threshold,
                };
                return Ok(Self::GridDetails(grid_details));
            }

            // the --tree case is handled by the DirAction parser later
            return Ok(Self::Details(details));
        }

        if strict {
            Self::strict_check_long_flags(matches)?;
        }

        if matches.tree {
            let details = details::Options::deduce_tree(matches, vars)?;
            return Ok(Self::Details(details));
        }

        if matches.oneline {
            return Ok(Self::Lines);
        }

        let grid = grid::Options::deduce(matches);
        Ok(Self::Grid(grid))
    }

    fn strict_check_long_flags(matches: &Opts) -> Result<(), OptionsError> {
        // If --long hasn’t been passed, then check if we need to warn the
        // user about flags that won’t have any effect.
        for option in &[
            (matches.binary, "binary"),
            (matches.bytes, "bytes"),
            (matches.inode, "inode"),
            (matches.links, "links"),
            (matches.header, "header"),
            (matches.blocksize, "blocksize"),
            (matches.time.is_some(), "time"),
            (matches.group, "group"),
            (matches.numeric, "numeric"),
            (matches.mounts, "mounts"),
        ] {
            let (opt, name) = option;
            if *opt {
                return Err(OptionsError::Useless(name, false, "long"));
            }
        }

        if matches.git && !matches.no_git {
            return Err(OptionsError::Useless("git", false, "long"));
        } else if matches.level.is_some() && !matches.recurse && !matches.tree {
            return Err(OptionsError::Useless2("level", "recurse", "tree"));
        }

        Ok(())
    }
}

impl grid::Options {
    fn deduce(matches: &Opts) -> Self {
        grid::Options {
            across: matches.across,
        }
    }
}

impl details::Options {
    fn deduce_tree<V: Vars>(matches: &Opts, vars: &V) -> Result<Self, OptionsError> {
        let details = details::Options {
            table: None,
            header: false,
            xattr: xattr::ENABLED && matches.extended,
            secattr: xattr::ENABLED && matches.security_context,
            mounts: matches.mounts,
            color_scale: ColorScaleOptions::deduce(matches, vars)?,
        };

        Ok(details)
    }

    fn deduce_long<V: Vars>(matches: &Opts, vars: &V, strict: bool) -> Result<Self, OptionsError> {
        if strict {
            if matches.across && !matches.grid {
                return Err(OptionsError::Useless("across", true, "long"));
            } else if matches.oneline {
                return Err(OptionsError::Useless("one-line", true, "long"));
            }
        }

        Ok(details::Options {
            table: Some(TableOptions::deduce(matches, vars)?),
            header: matches.header,
            xattr: xattr::ENABLED && matches.extended,
            secattr: xattr::ENABLED && matches.security_context,
            mounts: matches.mounts,
            color_scale: ColorScaleOptions::deduce(matches, vars)?,
        })
    }
}

impl TerminalWidth {
    fn deduce<V: Vars>(matches: &Opts, vars: &V) -> Result<Self, OptionsError> {
        if let Some(width) = matches.width {
            if width >= 1 {
                Ok(Set(width))
            } else {
                Ok(Automatic)
            }
        } else if let Some(columns) = vars.get(vars::COLUMNS).and_then(|s| s.into_string().ok()) {
            match columns.parse() {
                Ok(width) => Ok(Set(width)),
                Err(e) => {
                    let source = NumberSource::Env(vars::COLUMNS);
                    Err(OptionsError::FailedParse(columns, source, e))
                }
            }
        } else {
            Ok(Automatic)
        }
    }
}

impl RowThreshold {
    fn deduce<V: Vars>(vars: &V) -> Result<Self, OptionsError> {
        if let Some(columns) = vars
            .get_with_fallback(vars::EZA_GRID_ROWS, vars::EXA_GRID_ROWS)
            .and_then(|s| s.into_string().ok())
        {
            match columns.parse() {
                Ok(rows) => Ok(Self::MinimumRows(rows)),
                Err(e) => {
                    let source = NumberSource::Env(
                        vars.source(vars::EZA_GRID_ROWS, vars::EXA_GRID_ROWS)
                            .unwrap(),
                    );
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
        let time_format = TimeFormat::deduce(matches, vars);
        let flags_format = FlagsFormat::deduce(vars);
        let size_format = SizeFormat::deduce(matches);
        let user_format = UserFormat::deduce(matches);
        let group_format = GroupFormat::deduce(matches);
        let columns = Columns::deduce(matches, vars)?;
        Ok(Self {
            size_format,
            time_format,
            user_format,
            group_format,
            flags_format,
            columns,
        })
    }
}

impl Columns {
    fn deduce<V: Vars>(matches: &Opts, vars: &V) -> Result<Self, OptionsError> {
        let time_types = TimeTypes::deduce(matches)?;

        let no_git_env = vars
            .get_with_fallback(vars::EXA_OVERRIDE_GIT, vars::EZA_OVERRIDE_GIT)
            .is_some();

        let git = matches.git && !matches.no_git && !no_git_env;
        let subdir_git_repos = matches.git_repos && !matches.no_git && !no_git_env;
        let subdir_git_repos_no_stat =
            !subdir_git_repos && matches.git_repos_no_status && !matches.no_git && !no_git_env;

        let file_flags = matches.file_flags;
        let blocksize = matches.blocksize;
        let group = matches.group;
        let inode = matches.inode;
        let links = matches.links;
        let octal = matches.octal;
        let security_context = xattr::ENABLED && matches.security_context;

        let permissions = !matches.no_permissions;
        let filesize = !matches.no_filesize;
        let user = !matches.no_user;

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
            file_flags,
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
        use SizeFormat::*;
        if matches.binary {
            BinaryBytes
        } else if matches.bytes {
            JustBytes
        } else {
            DecimalBytes
        }
    }
}

impl TimeFormat {
    /// Determine how time should be formatted in timestamp columns.
    fn deduce<V: Vars>(matches: &Opts, vars: &V) -> Self {
        if let Some(arg) = &matches.time_style {
            arg.clone()
        } else {
            match vars.get(vars::TIME_STYLE) {
                Some(t) if !t.is_empty() => TimeFormat::from_str(t.to_str().unwrap_or(""), false)
                    .unwrap_or(TimeFormat::DefaultFormat),
                _ => Self::DefaultFormat,
            }
        }
    }
}

impl UserFormat {
    fn deduce(matches: &Opts) -> Self {
        if matches.numeric {
            Self::Numeric
        } else {
            Self::Name
        }
    }
}

impl GroupFormat {
    fn deduce(matches: &Opts) -> Self {
        if matches.smart_group {
            Self::Smart
        } else {
            Self::Regular
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
        let modified = matches.modified;
        let changed = matches.changed;
        let accessed = matches.accessed;
        let created = matches.created;

        let no_time = matches.no_time;

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
                return Err(OptionsError::Useless("modified", true, "time"));
            } else if changed {
                return Err(OptionsError::Useless("changed", true, "time"));
            } else if accessed {
                return Err(OptionsError::Useless("accessed", true, "time"));
            } else if created {
                return Err(OptionsError::Useless("created", true, "time"));
            } else if word == "mod" || word == "modified" {
                Self { modified: true,  changed: false, accessed: false, created: false }
            } else if word == "ch" || word == "changed" {
                Self { modified: false, changed: true,  accessed: false, created: false }
            } else if word == "acc" || word == "accessed" {
                Self { modified: false, changed: false, accessed: true,  created: false }
            } else if word == "cr" || word == "created" {
                Self { modified: false, changed: false, accessed: false, created: true  }
            } else {
                return Err(OptionsError::BadArgument("time", word.into()));
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

impl ColorScaleOptions {
    pub fn deduce<V: Vars>(matches: &Opts, vars: &V) -> Result<Self, OptionsError> {
        let min_luminance =
            match vars.get_with_fallback(vars::EZA_MIN_LUMINANCE, vars::EXA_MIN_LUMINANCE) {
                Some(var) => match var.to_string_lossy().parse() {
                    Ok(luminance) if (-100..=100).contains(&luminance) => luminance,
                    _ => 40,
                },
                None => 40,
            };

        let mode = match matches.color_scale_mode {
            ColorScaleModeArgs::Fixed => ColorScaleMode::Fixed,
            ColorScaleModeArgs::Gradient => ColorScaleMode::Gradient,
        };

        let mut options = ColorScaleOptions {
            mode,
            min_luminance,
            size: false,
            age: false,
        };

        let Some(words) = &matches.color_scale else {
            return Ok(options);
        };

        match words {
            ColorScaleArgs::All => {
                options.size = true;
                options.age = true;
            },
            ColorScaleArgs::Age => {
                options.age = true;
            },
            ColorScaleArgs::Size => {
                options.size = true;
            },
        }

        Ok(options)
    }
}

#[cfg(test)]
mod tests {
    use crate::options::{parser::ColorScaleArgs, vars::MockVars};
    use std::num::ParseIntError;
    use std::ffi::OsString;

    use super::*;

    #[test]
    fn deduce_time_types_no_time() {
        let options = Opts {
            no_time: true,
            ..Opts::default()
        };

        assert_eq!(
            TimeTypes::deduce(&options),
            Ok(TimeTypes {
                modified: false,
                ..TimeTypes::default()
            })
        );
    }

    #[test]
    fn deduce_time_types_default() {
        assert_eq!(
            TimeTypes::deduce(&Opts::default()),
            Ok(TimeTypes::default())
        );
    }

    #[test]
    fn deduce_time_types_modified_word() {
        let options = Opts {
            time: Some(OsString::from("modified")),
            ..Opts::default()
        };

        assert_eq!(
            TimeTypes::deduce(&options),
            Ok(TimeTypes {
                modified: true,
                ..TimeTypes::default()
            })
        );
    }

    #[test]
    fn deduce_time_types_accessed_word() {
        let options = Opts {
            time: Some(OsString::from("accessed")),
            ..Opts::default()
        };

        assert_eq!(
            TimeTypes::deduce(&options),
            Ok(TimeTypes {
                accessed: true,
                modified: false,
                ..TimeTypes::default()
            })
        );
    }

    #[test]
    fn deduce_time_types_changed_word() {
        let options = Opts {
            time: Some(OsString::from("changed")),
            ..Opts::default()
        };

        assert_eq!(
            TimeTypes::deduce(&options),
            Ok(TimeTypes {
                modified: false,
                changed: true,
                ..TimeTypes::default()
            })
        );
    }

    #[test]
    fn deduce_time_types_created_word() {
        let options = Opts {
            time: Some(OsString::from("created")),
            ..Opts::default()
        };

        assert_eq!(
            TimeTypes::deduce(&options),
            Ok(TimeTypes {
                modified: false,
                created: true,
                ..TimeTypes::default()
            })
        );
    }

    #[test]
    fn deduce_time_types_modified() {
        let options = Opts {
            modified: true,
            ..Opts::default()
        };

        assert_eq!(
            TimeTypes::deduce(&options),
            Ok(TimeTypes {
                modified: true,
                ..TimeTypes::default()
            })
        );
    }

    #[test]
    fn deduce_time_types_accessed() {
        let options = Opts {
            accessed: true,
            ..Opts::default()
        };

        assert_eq!(
            TimeTypes::deduce(&options),
            Ok(TimeTypes {
                accessed: true,
                modified: false,
                ..TimeTypes::default()
            })
        );
    }

    #[test]
    fn deduce_time_types_changed() {
        let options = Opts {
            changed: true,
            ..Opts::default()
        };

        assert_eq!(
            TimeTypes::deduce(&options),
            Ok(TimeTypes {
                modified: false,
                changed: true,
                ..TimeTypes::default()
            })
        );
    }

    #[test]
    fn deduce_time_types_created() {
        let options = Opts {
            created: true,
            ..Opts::default()
        };

        assert_eq!(
            TimeTypes::deduce(&options),
            Ok(TimeTypes {
                modified: false,
                created: true,
                ..TimeTypes::default()
            })
        );
    }

    #[test]
    fn deduce_group_format_on() {
        let options = Opts {
            smart_group: true,
            ..Opts::default()
        };

        assert_eq!(GroupFormat::deduce(&options), GroupFormat::Smart);
    }

    #[test]
    fn deduce_group_format_off() {
        let options = Opts { ..Opts::default() };

        assert_eq!(GroupFormat::deduce(&options), GroupFormat::Regular);
    }

    #[test]
    fn deduce_user_format_on() {
        let options = Opts {
            numeric: true,
            ..Opts::default()
        };

        assert_eq!(UserFormat::deduce(&options), UserFormat::Numeric);
    }

    #[test]
    fn deduce_user_format_off() {
        let options = Opts { ..Opts::default() };

        assert_eq!(UserFormat::deduce(&options), UserFormat::Name);
    }

    #[test]
    fn deduce_size_format_off() {
        let options = Opts { ..Opts::default() };

        assert_eq!(SizeFormat::deduce(&options), SizeFormat::DecimalBytes);
    }

    #[test]
    fn deduce_user_format_bytes() {
        let options = Opts {
            bytes: true,
            ..Opts::default()
        };

        assert_eq!(SizeFormat::deduce(&options), SizeFormat::JustBytes);
    }

    #[test]
    fn deduce_user_format_binary() {
        let options = Opts {
            binary: true,
            ..Opts::default()
        };

        assert_eq!(SizeFormat::deduce(&options), SizeFormat::BinaryBytes);
    }

    #[test]
    fn deduce_grid_options() {
        let options = Opts {
            across: true,
            ..Opts::default()
        };

        assert_eq!(
            grid::Options::deduce(&options),
            grid::Options { across: true }
        );
    }

    #[test]
    fn deduce_time_style_iso_env() {
        let mut vars = MockVars {
            ..MockVars::default()
        };

        let options = Opts { ..Opts::default() };

        vars.set(vars::TIME_STYLE, &OsString::from("iso"));
        assert_eq!(TimeFormat::deduce(&options, &vars), TimeFormat::ISOFormat);
    }

    #[test]
    fn deduce_time_style_iso_arg() {
        let vars = MockVars {
            ..MockVars::default()
        };

        let options = Opts {
            time_style: Some(TimeFormat::ISOFormat),
            ..Opts::default()
        };

        assert_eq!(TimeFormat::deduce(&options, &vars), TimeFormat::ISOFormat);
    }

    #[test]
    fn deduce_time_style_long_iso_env() {
        let mut vars = MockVars {
            ..MockVars::default()
        };

        let options = Opts { ..Opts::default() };

        vars.set(vars::TIME_STYLE, &OsString::from("long-iso"));
        assert_eq!(TimeFormat::deduce(&options, &vars), TimeFormat::LongISO);
    }

    #[test]
    fn deduce_time_style_long_iso_arg() {
        let vars = MockVars {
            ..MockVars::default()
        };

        let options = Opts {
            time_style: Some(TimeFormat::LongISO),
            ..Opts::default()
        };

        assert_eq!(TimeFormat::deduce(&options, &vars), TimeFormat::LongISO);
    }

    #[test]
    fn deduce_time_style_full_iso_env() {
        let mut vars = MockVars {
            ..MockVars::default()
        };

        let options = Opts { ..Opts::default() };

        vars.set(vars::TIME_STYLE, &OsString::from("full-iso"));
        assert_eq!(TimeFormat::deduce(&options, &vars), TimeFormat::FullISO);
    }

    #[test]
    fn deduce_time_style_full_iso_arg() {
        let vars = MockVars {
            ..MockVars::default()
        };

        let options = Opts {
            time_style: Some(TimeFormat::FullISO),
            ..Opts::default()
        };

        assert_eq!(TimeFormat::deduce(&options, &vars), TimeFormat::FullISO);
    }

    #[test]
    fn deduce_time_style_relative_env() {
        let mut vars = MockVars {
            ..MockVars::default()
        };

        let options = Opts { ..Opts::default() };

        vars.set(vars::TIME_STYLE, &OsString::from("relative"));
        assert_eq!(TimeFormat::deduce(&options, &vars), TimeFormat::Relative);
    }

    #[test]
    fn deduce_time_style_relative_arg() {
        let vars = MockVars {
            ..MockVars::default()
        };

        let options = Opts {
            time_style: Some(TimeFormat::Relative),
            ..Opts::default()
        };

        assert_eq!(TimeFormat::deduce(&options, &vars), TimeFormat::Relative);
    }

    #[test]
    fn deduce_time_style_custom_env() {
        let mut vars = MockVars {
            ..MockVars::default()
        };

        let options = Opts { ..Opts::default() };

        vars.set(vars::TIME_STYLE, &OsString::from("+%Y-%b-%d"));
        assert_eq!(
            TimeFormat::deduce(&options, &vars),
            TimeFormat::Custom {
                non_recent: Some(String::from("%Y-%b-%d")),
                recent: None
            }
        );
    }

    #[test]
    fn deduce_time_style_custom_arg() {
        let vars = MockVars {
            ..MockVars::default()
        };

        let options = Opts {
            time_style: Some(TimeFormat::from_str("+%Y-%b-%d", true).unwrap()),
            ..Opts::default()
        };

        assert_eq!(
            TimeFormat::deduce(&options, &vars),
            TimeFormat::Custom {
                non_recent: Some(String::from("%Y-%b-%d")),
                recent: None
            }
        );
    }

    #[test]
    fn deduce_time_style_non_recent_and_recent() {
        let vars = MockVars {
            ..MockVars::default()
        };

        let options = Opts {
            time_style: Some(
                TimeFormat::from_str(
                    "+%Y-%m-%d %H
--%m-%d %H:%M",
                    false,
                )
                .unwrap(),
            ),
            ..Opts::default()
        };

        assert_eq!(
            TimeFormat::deduce(&options, &vars),
            TimeFormat::Custom {
                non_recent: Some(String::from("%Y-%m-%d %H")),
                recent: Some(String::from("--%m-%d %H:%M"))
            }
        );
    }

    #[test]
    fn deduce_color_scale_size_age_luminance_40_gradient() {
        let vars = MockVars {
            ..MockVars::default()
        };

        let options = Opts {
            color_scale: Some(ColorScaleArgs::from_str("size,age", false).unwrap()),
            ..Opts::default()
        };

        assert_eq!(
            ColorScaleOptions::deduce(&options, &vars),
            Ok(ColorScaleOptions {
                mode: ColorScaleMode::Gradient,
                min_luminance: 40,
                size: true,
                age: true,
            })
        );
    }

    #[test]
    fn deduce_color_scale_size_luminance_60_gradient() {
        let mut vars = MockVars {
            ..MockVars::default()
        };

        let options = Opts {
            color_scale: Some(ColorScaleArgs::from_str("size", true).unwrap()),
            ..Opts::default()
        };

        vars.set(vars::EZA_MIN_LUMINANCE, &OsString::from("60"));

        assert_eq!(
            ColorScaleOptions::deduce(&options, &vars),
            Ok(ColorScaleOptions {
                mode: ColorScaleMode::Gradient,
                min_luminance: 60,
                size: true,
                age: false,
            })
        );
    }

    #[test]
    fn deduce_color_scale_age_luminance_60_fixed() {
        let mut vars = MockVars {
            ..MockVars::default()
        };

        let options = Opts {
            color_scale_mode: ColorScaleModeArgs::Fixed,
            color_scale: Some(ColorScaleArgs::from_str("Age", true).unwrap()),
            ..Opts::default()
        };

        vars.set(vars::EZA_MIN_LUMINANCE, &OsString::from("60"));

        assert_eq!(
            ColorScaleOptions::deduce(&options, &vars),
            Ok(ColorScaleOptions {
                mode: ColorScaleMode::Fixed,
                min_luminance: 60,
                size: false,
                age: true,
            })
        );
    }

    #[test]
    fn deduce_color_scale_size_age_luminance_99_fixed() {
        let mut vars = MockVars {
            ..MockVars::default()
        };

        let options = Opts {
            color_scale: Some(ColorScaleArgs::from_str("size,age", false).unwrap()),
            color_scale_mode: ColorScaleModeArgs::Fixed,
            ..Opts::default()
        };

        vars.set(vars::EZA_MIN_LUMINANCE, &OsString::from("99"));

        assert_eq!(
            ColorScaleOptions::deduce(&options, &vars),
            Ok(ColorScaleOptions {
                mode: ColorScaleMode::Fixed,
                min_luminance: 99,
                size: true,
                age: true,
            })
        );
    }

    #[test]
    fn deduce_mode_grid() {
        let vars = MockVars {
            ..MockVars::default()
        };

        let options = Opts {
            grid: true,
            ..Opts::default()
        };

        assert_eq!(
            Mode::deduce(&options, &vars, false),
            Ok(Mode::Grid(grid::Options { across: false }))
        );
    }

    #[test]
    fn deduce_mode_grid_across() {
        let vars = MockVars {
            ..MockVars::default()
        };

        let options = Opts {
            grid: true,
            across: true,
            ..Opts::default()
        };

        assert_eq!(
            Mode::deduce(&options, &vars, false),
            Ok(Mode::Grid(grid::Options { across: true }))
        );
    }
    #[test]
    fn deduce_details_options_tree() {
        let options = Opts {
            tree: true,
            ..Opts::default()
        };

        let vars = MockVars {
            ..MockVars::default()
        };

        assert_eq!(
            details::Options::deduce_tree(&options, &vars),
            Ok(details::Options {
                table: None,
                header: false,
                xattr: xattr::ENABLED && options.extended,
                secattr: xattr::ENABLED && options.security_context,
                mounts: options.mounts,
                color_scale: ColorScaleOptions::deduce(&options, &vars).unwrap(),
            })
        );
    }

    #[test]
    fn deduce_details_options_tree_mounts() {
        let options = Opts {
            tree: true,
            mounts: true,
            ..Opts::default()
        };

        let vars = MockVars {
            ..MockVars::default()
        };

        assert_eq!(
            details::Options::deduce_tree(&options, &vars),
            Ok(details::Options {
                table: None,
                header: false,
                xattr: false,
                secattr: false,
                mounts: true,
                color_scale: ColorScaleOptions::deduce(&options, &vars).unwrap(),
            })
        );
    }

    #[test]
    fn deduce_details_options_tree_xattr() {
        let options = Opts {
            tree: true,
            extended: true,
            ..Opts::default()
        };

        let vars = MockVars {
            ..MockVars::default()
        };

        assert_eq!(
            details::Options::deduce_tree(&options, &vars),
            Ok(details::Options {
                table: None,
                header: false,
                xattr: xattr::ENABLED && options.extended,
                secattr: xattr::ENABLED && options.security_context,
                mounts: false,
                color_scale: ColorScaleOptions::deduce(&options, &vars).unwrap(),
            })
        );
    }

    #[test]
    fn deduce_details_options_tree_secattr() {
        let options = Opts {
            tree: true,
            security_context: true,
            ..Opts::default()
        };

        let vars = MockVars {
            ..MockVars::default()
        };

        assert_eq!(
            details::Options::deduce_tree(&options, &vars),
            Ok(details::Options {
                table: None,
                header: false,
                xattr: xattr::ENABLED && options.extended,
                secattr: xattr::ENABLED && options.security_context,
                mounts: false,
                color_scale: ColorScaleOptions::deduce(&options, &vars).unwrap(),
            })
        );
    }

    #[test]
    fn deduce_details_long_strict_across() {
        let options = Opts {
            long: true,
            across: true,
            ..Opts::default()
        };

        let vars = MockVars {
            ..MockVars::default()
        };

        assert_eq!(
            details::Options::deduce_long(&options, &vars, true),
            Err(OptionsError::Useless("across", true, "long"))
        );
    }

    #[test]
    fn deduce_details_long_strict_one_line() {
        let options = Opts {
            long: true,
            oneline: true,
            ..Opts::default()
        };

        let vars = MockVars {
            ..MockVars::default()
        };

        assert_eq!(
            details::Options::deduce_long(&options, &vars, true),
            Err(OptionsError::Useless("one-line", true, "long"))
        );
    }

    #[test]
    fn deduce_terminal_width_automatic() {
        let options = Opts { ..Opts::default() };

        let vars = MockVars {
            ..MockVars::default()
        };

        assert_eq!(TerminalWidth::deduce(&options, &vars), Ok(Automatic));
    }

    #[test]
    fn deduce_terminal_width_set_arg() {
        let options = Opts {
            width: Some(80),
            ..Opts::default()
        };

        let vars = MockVars {
            ..MockVars::default()
        };

        assert_eq!(TerminalWidth::deduce(&options, &vars), Ok(Set(80)));
    }

    #[test]
    fn deduce_terminal_width_set_env() {
        let mut vars = MockVars {
            ..MockVars::default()
        };

        vars.set(vars::COLUMNS, &OsString::from("80"));

        let options = Opts { ..Opts::default() };

        assert_eq!(TerminalWidth::deduce(&options, &vars), Ok(Set(80)));
    }

    #[test]
    fn deduce_terminal_width_set_env_bad() {
        let mut vars = MockVars {
            ..MockVars::default()
        };

        vars.set(vars::COLUMNS, &OsString::from("bad"));

        let options = Opts { ..Opts::default() };

        let e: Result<i64, ParseIntError> =
            vars.get(vars::COLUMNS).unwrap().to_string_lossy().parse();

        assert_eq!(
            TerminalWidth::deduce(&options, &vars),
            Err(OptionsError::FailedParse(
                String::from("bad"),
                NumberSource::Env(vars::COLUMNS),
                e.unwrap_err()
            ))
        );
    }
}
