// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use clap::{ArgMatches, ValueEnum};

use crate::output::TerminalWidth::Automatic;

use crate::fs::feature::xattr;
use crate::options::parser::ColorScaleModeArgs;
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

use super::parser::{ColorScaleArgs, TimeArgs};

impl View {
    pub fn deduce<V: Vars>(
        matches: &ArgMatches,
        vars: &V,
        strict: bool,
    ) -> Result<Self, OptionsError> {
        let mode = Mode::deduce(matches, vars, strict)?;
        let deref_links = matches.get_flag("dereference");
        let follow_links = matches.get_flag("follow-symlinks");
        let total_size = matches.get_flag("total-size");
        let width = TerminalWidth::deduce(matches, vars)?;
        let file_style = FileStyle::deduce(matches, vars, width.actual_terminal_width().is_some())?;
        Ok(Self {
            mode,
            width,
            file_style,
            deref_links,
            follow_links,
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
    pub fn deduce<V: Vars>(
        matches: &ArgMatches,
        vars: &V,
        strict: bool,
    ) -> Result<Self, OptionsError> {
        let long = matches.get_flag("long");
        let oneline = matches.get_flag("oneline");
        let grid = matches.get_flag("grid");
        let tree = matches.get_flag("tree");

        if !(long || oneline || grid || tree) {
            if strict {
                Self::strict_check_long_flags(matches)?;
            }
            let grid = grid::Options::deduce(matches);
            return Ok(Self::Grid(grid));
        };

        if long {
            let details = details::Options::deduce_long(matches, vars, strict)?;

            if grid {
                let _grid = grid::Options::deduce(matches);
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

        if tree {
            let details = details::Options::deduce_tree(matches, vars);
            return Ok(Self::Details(details));
        }

        if oneline {
            return Ok(Self::Lines);
        }

        let grid = grid::Options::deduce(matches);
        Ok(Self::Grid(grid))
    }

    fn strict_check_long_flags(matches: &ArgMatches) -> Result<(), OptionsError> {
        // If --long hasn’t been passed, then check if we need to warn the
        // user about flags that won’t have any effect.
        for flag in &[
            "binary",
            "bytes",
            "inode",
            "links",
            "header",
            "blocksize",
            "time",
            "group",
            "numeric",
            "mounts",
        ] {
            if matches.contains_id(flag) {
                return Err(OptionsError::Useless(flag, false, "long"));
            }
        }

        if matches.get_flag("git") && !matches.get_flag("no-git") {
            return Err(OptionsError::Useless("git", false, "long"));
        } else if matches.contains_id("level")
            && !matches.get_flag("recurse")
            && !matches.get_flag("tree")
        {
            return Err(OptionsError::Useless2("level", "recurse", "tree"));
        }

        Ok(())
    }
}

impl grid::Options {
    fn deduce(matches: &ArgMatches) -> Self {
        grid::Options {
            across: matches.get_flag("across"),
        }
    }
}

impl details::Options {
    fn deduce_tree<V: Vars>(matches: &ArgMatches, vars: &V) -> Self {
        details::Options {
            table: None,
            header: false,
            xattr: xattr::ENABLED && matches.get_flag("extended"),
            secattr: xattr::ENABLED && matches.get_flag("security-context"),
            mounts: matches.get_flag("mounts"),
            color_scale: ColorScaleOptions::deduce(matches, vars),
            follow_links: matches.get_flag("follow-symlinks"),
        }
    }

    fn deduce_long<V: Vars>(
        matches: &ArgMatches,
        vars: &V,
        strict: bool,
    ) -> Result<Self, OptionsError> {
        if strict {
            if matches.get_flag("across") && !matches.get_flag("grid") {
                return Err(OptionsError::Useless("across", true, "long"));
            } else if matches.get_flag("oneline") {
                return Err(OptionsError::Useless("one-line", true, "long"));
            }
        }

        Ok(details::Options {
            table: Some(TableOptions::deduce(matches, vars)?),
            header: matches.get_flag("header"),
            xattr: xattr::ENABLED && matches.get_flag("extended"),
            secattr: xattr::ENABLED && matches.get_flag("security-context"),
            mounts: matches.get_flag("mounts"),
            color_scale: ColorScaleOptions::deduce(matches, vars),
            follow_links: matches.get_flag("follow-symlinks"),
        })
    }
}

impl TerminalWidth {
    fn deduce<V: Vars>(matches: &ArgMatches, vars: &V) -> Result<Self, OptionsError> {
        if let Some(&width) = matches.get_one("width") {
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
    fn deduce<V: Vars>(matches: &ArgMatches, vars: &V) -> Result<Self, OptionsError> {
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
    fn deduce<V: Vars>(matches: &ArgMatches, vars: &V) -> Result<Self, OptionsError> {
        let time_types = TimeTypes::deduce(matches)?;

        let no_git_env = vars
            .get_with_fallback(vars::EXA_OVERRIDE_GIT, vars::EZA_OVERRIDE_GIT)
            .is_some();

        let git = matches.get_flag("git") && !matches.get_flag("no-git") && !no_git_env;
        let subdir_git_repos =
            matches.get_flag("git-repos") && !matches.get_flag("no-git") && !no_git_env;
        let subdir_git_repos_no_stat = !subdir_git_repos
            && matches.get_flag("git-repos-no-status")
            && !matches.get_flag("no-git")
            && !no_git_env;

        let file_flags = matches.get_flag("file-flags");
        let blocksize = matches.get_flag("blocksize");
        let group = matches.get_flag("group");
        let inode = matches.get_flag("inode");
        let links = matches.get_flag("links");
        let octal = matches.get_flag("octal-permissions");
        let security_context = xattr::ENABLED && matches.get_flag("security-context");

        let permissions = !matches.get_flag("no-permissions");
        let filesize = !matches.get_flag("no-filesize");
        let user = !matches.get_flag("no-user");

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
    fn deduce(matches: &ArgMatches) -> Self {
        use SizeFormat::*;
        if matches.get_flag("binary") {
            BinaryBytes
        } else if matches.get_flag("bytes") {
            JustBytes
        } else {
            DecimalBytes
        }
    }
}

const FORMAT_STYLE_FIELDS: [&str; 6] = [
    "default",
    "iso",
    "long-iso",
    "full-iso",
    "relative",
    "+<CUSTOM_FORMAT>",
];

impl TimeFormat {
    /// Determine how time should be formatted in timestamp columns.
    pub fn try_from_str(value: &str) -> Result<Self, String> {
        use nu_ansi_term::Color::*;

        let error_header = format!(
            "invalid value '{}' for '{}'\n  [possible values: {}]\n\n",
            Yellow.paint(value),
            White.paint("--time-style <STYLE>"),
            FORMAT_STYLE_FIELDS
                .map(|s| Green.paint(s).to_string())
                .join(", ")
        );
        let error_footer = format!("For more information, try '{}'.\n", White.paint("--help"),);

        let fmt = match value {
            "default" => return Ok(TimeFormat::DefaultFormat),
            "iso" => return Ok(TimeFormat::ISOFormat),
            "long-iso" => return Ok(TimeFormat::LongISO),
            "full-iso" => return Ok(TimeFormat::FullISO),
            "relative" => return Ok(TimeFormat::Relative),
            s if !s.starts_with('+') => {
                let error_middle =
                    format!("{}{}\n\n",
                    "Please start the format with a plus sign (+) to indicate a custom format.\n",
                    "For example: \"+%Y-%m-%d %H:%M:%S\"",
                );
                return Err(format!("{error_header}{error_middle}{error_footer}"));
            }
            s => s,
        };

        let mut lines = fmt.strip_prefix('+').unwrap().lines();

        // line 1 is None when there is nothing after `+`
        // line 1 is empty when `+` is followed immediately by `\n`
        let non_recent = match lines.next() {
            None | Some("") => {
                let error_middle = format!(
                    "{}{}",
                    "Custom timestamp format is empty,",
                    "please supply a chrono format string after the +."
                );
                return Err(format!("{error_header}{error_middle}{error_footer}"));
            }
            Some(non_recent) => non_recent,
        };

        // line 2 is None when there is not a single `\n`, or nothing after the first `\n`
        // line 2 is empty when there are at least 2 `\n`, and nothing between the 1st and 2nd `\n`
        let recent = match lines.next() {
            Some("") => {
                let error_middle = format!(
                    "{}{}",
                    "Custom timestamp format for recent files is empty,",
                    "please supply a chrono format string at the second line."
                );
                return Err(format!("{error_header}{error_middle}{error_footer}"));
            }
            recent => recent.map(std::string::ToString::to_string),
        };

        Ok(TimeFormat::Custom {
            non_recent: String::from(non_recent),
            recent,
        })
    }
}

impl TimeFormat {
    /// Determine how time should be formatted in timestamp columns.
    fn deduce<V: Vars>(matches: &ArgMatches, vars: &V) -> Self {
        if let Some(arg) = matches.get_one::<TimeFormat>("time-style") {
            arg.clone()
        } else {
            match vars.get(vars::TIME_STYLE) {
                Some(t) if !t.is_empty() => TimeFormat::try_from_str(t.to_str().unwrap_or(""))
                    .unwrap_or(TimeFormat::DefaultFormat),
                _ => Self::DefaultFormat,
            }
        }
    }
}

impl UserFormat {
    fn deduce(matches: &ArgMatches) -> Self {
        if matches.get_flag("numeric") {
            Self::Numeric
        } else {
            Self::Name
        }
    }
}

impl GroupFormat {
    fn deduce(matches: &ArgMatches) -> Self {
        if matches.get_flag("smart-group") {
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
    fn deduce(matches: &ArgMatches) -> Result<Self, OptionsError> {
        let possible_word = matches.get_one::<TimeArgs>("time");
        let modified = matches.get_flag("modified");
        let changed = matches.get_flag("changed");
        let accessed = matches.get_flag("accessed");
        let created = matches.get_flag("created");

        let no_time = matches.get_flag("no-time");

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
            } else if *word == TimeArgs::Modified  {
                Self { modified: true,  changed: false, accessed: false, created: false }
            } else if *word == TimeArgs::Changed {
                Self { modified: false, changed: true,  accessed: false, created: false }
            } else if *word == TimeArgs::Accessed {
                Self { modified: false, changed: false, accessed: true,  created: false }
            } else if *word == TimeArgs::Created {
                Self { modified: false, changed: false, accessed: false, created: true  }
            } else {
                return Err(OptionsError::BadArgument("time", word.to_possible_value().unwrap().get_name().into()));
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
    pub fn deduce<V: Vars>(matches: &ArgMatches, vars: &V) -> Self {
        let min_luminance =
            match vars.get_with_fallback(vars::EZA_MIN_LUMINANCE, vars::EXA_MIN_LUMINANCE) {
                Some(var) => match var.to_string_lossy().parse() {
                    Ok(luminance) if (-100..=100).contains(&luminance) => luminance,
                    _ => 40,
                },
                None => 40,
            };

        let mode = match matches.get_one("color-scale-mode").unwrap() {
            ColorScaleModeArgs::Fixed => ColorScaleMode::Fixed,
            ColorScaleModeArgs::Gradient => ColorScaleMode::Gradient,
        };

        let mut options = ColorScaleOptions {
            mode,
            min_luminance,
            size: false,
            age: false,
        };

        let Some(words) = matches.get_many("color-scale") else {
            return options;
        };

        for word in words {
            match word {
                ColorScaleArgs::All => {
                    options.size = true;
                    options.age = true;
                }
                ColorScaleArgs::Age => {
                    options.age = true;
                }
                ColorScaleArgs::Size => {
                    options.size = true;
                }
            }
        }

        options
    }
}

#[cfg(test)]
mod tests {
    use crate::options::parser::test::mock_cli;
    use crate::options::vars::test::MockVars;
    use std::ffi::OsString;
    use std::num::ParseIntError;

    use super::*;

    #[test]
    fn deduce_time_types_no_time() {
        assert_eq!(
            TimeTypes::deduce(&mock_cli(vec!["--no-time"])),
            Ok(TimeTypes {
                modified: false,
                ..TimeTypes::default()
            })
        );
    }

    #[test]
    fn deduce_time_types_default() {
        assert_eq!(
            TimeTypes::deduce(&mock_cli(vec![""])),
            Ok(TimeTypes::default())
        );
    }

    #[test]
    fn deduce_time_types_modified_word() {
        assert_eq!(
            TimeTypes::deduce(&mock_cli(vec!["--time", "modified"])),
            Ok(TimeTypes {
                modified: true,
                ..TimeTypes::default()
            })
        );
    }

    #[test]
    fn deduce_time_types_accessed_word() {
        assert_eq!(
            TimeTypes::deduce(&mock_cli(vec!["--time", "accessed"])),
            Ok(TimeTypes {
                accessed: true,
                modified: false,
                ..TimeTypes::default()
            })
        );
    }

    #[test]
    fn deduce_time_types_changed_word() {
        assert_eq!(
            TimeTypes::deduce(&&mock_cli(vec!["--time", "changed"])),
            Ok(TimeTypes {
                modified: false,
                changed: true,
                ..TimeTypes::default()
            })
        );
    }

    #[test]
    fn deduce_time_types_created_word() {
        assert_eq!(
            TimeTypes::deduce(&mock_cli(vec!["--time", "created"])),
            Ok(TimeTypes {
                modified: false,
                created: true,
                ..TimeTypes::default()
            })
        );
    }

    #[test]
    fn deduce_time_types_modified() {
        assert_eq!(
            TimeTypes::deduce(&mock_cli(vec!["--modified"])),
            Ok(TimeTypes {
                modified: true,
                ..TimeTypes::default()
            })
        );
    }

    #[test]
    fn deduce_time_types_accessed() {
        assert_eq!(
            TimeTypes::deduce(&mock_cli(vec!["--accessed"])),
            Ok(TimeTypes {
                accessed: true,
                modified: false,
                ..TimeTypes::default()
            })
        );
    }

    #[test]
    fn deduce_time_types_changed() {
        assert_eq!(
            TimeTypes::deduce(&mock_cli(vec!["--changed"])),
            Ok(TimeTypes {
                modified: false,
                changed: true,
                ..TimeTypes::default()
            })
        );
    }

    #[test]
    fn deduce_time_types_created() {
        assert_eq!(
            TimeTypes::deduce(&mock_cli(vec!["--created"])),
            Ok(TimeTypes {
                modified: false,
                created: true,
                ..TimeTypes::default()
            })
        );
    }

    #[test]
    fn deduce_group_format_on() {
        assert_eq!(
            GroupFormat::deduce(&mock_cli(vec!["--smart-group"])),
            GroupFormat::Smart
        );
    }

    #[test]
    fn deduce_group_format_off() {
        assert_eq!(
            GroupFormat::deduce(&mock_cli(vec![""])),
            GroupFormat::Regular
        );
    }

    #[test]
    fn deduce_user_format_on() {
        assert_eq!(
            UserFormat::deduce(&mock_cli(vec!["--numeric"])),
            UserFormat::Numeric
        );
    }

    #[test]
    fn deduce_user_format_off() {
        assert_eq!(UserFormat::deduce(&mock_cli(vec![""])), UserFormat::Name);
    }

    #[test]
    fn deduce_size_format_off() {
        assert_eq!(
            SizeFormat::deduce(&mock_cli(vec![""])),
            SizeFormat::DecimalBytes
        );
    }

    #[test]
    fn deduce_user_format_bytes() {
        assert_eq!(
            SizeFormat::deduce(&mock_cli(vec!["--bytes"])),
            SizeFormat::JustBytes
        );
    }

    #[test]
    fn deduce_user_format_binary() {
        assert_eq!(
            SizeFormat::deduce(&mock_cli(vec!["--binary"])),
            SizeFormat::BinaryBytes
        );
    }

    #[test]
    fn deduce_grid_options() {
        assert_eq!(
            grid::Options::deduce(&mock_cli(vec!["--across"])),
            grid::Options { across: true }
        );
    }

    #[test]
    fn deduce_time_style_iso_env() {
        let mut vars = MockVars::default();
        vars.set(vars::TIME_STYLE, &OsString::from("iso"));
        assert_eq!(
            TimeFormat::deduce(&mock_cli(vec![""]), &vars),
            TimeFormat::ISOFormat
        );
    }

    #[test]
    fn deduce_time_style_iso_arg() {
        let vars = MockVars::default();
        assert_eq!(
            TimeFormat::deduce(&mock_cli(vec!["--time-style", "iso"]), &vars),
            TimeFormat::ISOFormat
        );
    }

    #[test]
    fn deduce_time_style_long_iso_env() {
        let mut vars = MockVars::default();
        vars.set(vars::TIME_STYLE, &OsString::from("long-iso"));
        assert_eq!(
            TimeFormat::deduce(&mock_cli(vec![""]), &vars),
            TimeFormat::LongISO
        );
    }

    #[test]
    fn deduce_time_style_long_iso_arg() {
        let vars = MockVars::default();
        assert_eq!(
            TimeFormat::deduce(&mock_cli(vec!["--time-style", "long-iso"]), &vars),
            TimeFormat::LongISO
        );
    }

    #[test]
    fn deduce_time_style_full_iso_env() {
        let mut vars = MockVars::default();
        vars.set(vars::TIME_STYLE, &OsString::from("full-iso"));
        assert_eq!(
            TimeFormat::deduce(&mock_cli(vec![""]), &vars),
            TimeFormat::FullISO
        );
    }

    #[test]
    fn deduce_time_style_full_iso_arg() {
        let vars = MockVars::default();
        assert_eq!(
            TimeFormat::deduce(&mock_cli(vec!["--time-style", "full-iso"]), &vars),
            TimeFormat::FullISO
        );
    }

    #[test]
    fn deduce_time_style_relative_env() {
        let mut vars = MockVars::default();
        vars.set(vars::TIME_STYLE, &OsString::from("relative"));
        assert_eq!(
            TimeFormat::deduce(&mock_cli(vec![""]), &vars),
            TimeFormat::Relative
        );
    }

    #[test]
    fn deduce_time_style_relative_arg() {
        let vars = MockVars::default();
        assert_eq!(
            TimeFormat::deduce(&mock_cli(vec!["--time-style", "relative"]), &vars),
            TimeFormat::Relative
        );
    }

    #[test]
    fn deduce_time_style_custom_env() {
        let mut vars = MockVars::default();
        vars.set(vars::TIME_STYLE, &OsString::from("+%Y-%b-%d"));
        assert_eq!(
            TimeFormat::deduce(&mock_cli(vec![""]), &vars),
            TimeFormat::Custom {
                non_recent: String::from("%Y-%b-%d"),
                recent: None
            }
        );
    }

    #[test]
    fn deduce_time_style_custom_arg() {
        assert_eq!(
            TimeFormat::deduce(
                &mock_cli(vec!["--time-style", "+%Y-%b-%d"]),
                &MockVars::default()
            ),
            TimeFormat::Custom {
                non_recent: String::from("%Y-%b-%d"),
                recent: None
            }
        );
    }

    #[test]
    fn deduce_time_style_non_recent_and_recent() {
        assert_eq!(
            TimeFormat::deduce(
                &mock_cli(vec!["--time-style", "+%Y-%m-%d %H\n--%m-%d %H:%M"]),
                &MockVars::default()
            ),
            TimeFormat::Custom {
                non_recent: String::from("%Y-%m-%d %H"),
                recent: Some(String::from("--%m-%d %H:%M"))
            }
        );
    }

    #[test]
    fn deduce_color_scale_size_age_luminance_40_gradient() {
        assert_eq!(
            ColorScaleOptions::deduce(
                &mock_cli(vec!["--color-scale", "size,age"]),
                &MockVars::default()
            ),
            ColorScaleOptions {
                mode: ColorScaleMode::Gradient,
                min_luminance: 40,
                size: true,
                age: true,
            }
        );
    }

    #[test]
    fn deduce_color_scale_size_luminance_60_gradient() {
        let mut vars = MockVars::default();
        vars.set(vars::EZA_MIN_LUMINANCE, &OsString::from("60"));
        assert_eq!(
            ColorScaleOptions::deduce(&mock_cli(vec!["--color-scale", "size"]), &vars),
            ColorScaleOptions {
                mode: ColorScaleMode::Gradient,
                min_luminance: 60,
                size: true,
                age: false,
            }
        );
    }

    #[test]
    fn deduce_color_scale_age_luminance_60_fixed() {
        let mut vars = MockVars::default();
        vars.set(vars::EZA_MIN_LUMINANCE, &OsString::from("60"));
        assert_eq!(
            ColorScaleOptions::deduce(
                &mock_cli(vec!["--color-scale", "age", "--color-scale-mode", "fixed"]),
                &vars
            ),
            ColorScaleOptions {
                mode: ColorScaleMode::Fixed,
                min_luminance: 60,
                size: false,
                age: true,
            }
        );
    }

    #[test]
    fn deduce_color_scale_size_age_luminance_99_fixed() {
        let mut vars = MockVars::default();
        vars.set(vars::EZA_MIN_LUMINANCE, &OsString::from("99"));
        assert_eq!(
            ColorScaleOptions::deduce(
                &mock_cli(vec![
                    "--color-scale",
                    "size,age",
                    "--color-scale-mode",
                    "fixed"
                ]),
                &vars
            ),
            ColorScaleOptions {
                mode: ColorScaleMode::Fixed,
                min_luminance: 99,
                size: true,
                age: true,
            }
        );
    }

    #[test]
    fn deduce_mode_grid() {
        assert_eq!(
            Mode::deduce(&mock_cli(vec!["--grid"]), &MockVars::default(), false),
            Ok(Mode::Grid(grid::Options { across: false }))
        );
    }

    #[test]
    fn deduce_mode_grid_across() {
        assert_eq!(
            Mode::deduce(
                &mock_cli(vec!["--grid", "--across"]),
                &MockVars::default(),
                false
            ),
            Ok(Mode::Grid(grid::Options { across: true }))
        );
    }
    #[test]
    fn deduce_details_options_tree() {
        let cli = mock_cli(vec!["--tree"]);
        assert_eq!(
            details::Options::deduce_tree(&cli, &MockVars::default()),
            details::Options {
                table: None,
                header: false,
                xattr: false,
                secattr: false,
                mounts: false,
                color_scale: ColorScaleOptions::deduce(&cli, &MockVars::default()),
                follow_links: false,
            }
        );
    }

    #[test]
    fn deduce_details_options_tree_mounts() {
        let cli = mock_cli(vec!["--tree", "--mounts"]);
        assert_eq!(
            details::Options::deduce_tree(&cli, &MockVars::default()),
            details::Options {
                table: None,
                header: false,
                xattr: false,
                secattr: false,
                mounts: true,
                color_scale: ColorScaleOptions::deduce(&cli, &MockVars::default()),
                follow_links: false,
            }
        );
    }

    #[test]
    fn deduce_details_options_tree_xattr() {
        let cli = mock_cli(vec!["--tree", "--extended"]);
        assert_eq!(
            details::Options::deduce_tree(&cli, &MockVars::default()),
            details::Options {
                table: None,
                header: false,
                xattr: xattr::ENABLED,
                secattr: false,
                mounts: false,
                color_scale: ColorScaleOptions::deduce(&cli, &MockVars::default()),
                follow_links: false,
            }
        );
    }

    #[test]
    fn deduce_details_options_tree_secattr() {
        let cli = mock_cli(vec!["--tree", "--context"]);
        assert_eq!(
            details::Options::deduce_tree(&cli, &MockVars::default()),
            details::Options {
                table: None,
                header: false,
                xattr: false,
                secattr: xattr::ENABLED,
                mounts: false,
                color_scale: ColorScaleOptions::deduce(&cli, &MockVars::default()),
                follow_links: false,
            }
        );
    }

    #[test]
    fn deduce_details_long_strict_across() {
        assert_eq!(
            details::Options::deduce_long(
                &mock_cli(vec!["--long", "--across"]),
                &MockVars::default(),
                true
            ),
            Err(OptionsError::Useless("across", true, "long"))
        );
    }

    #[test]
    fn deduce_details_long_strict_one_line() {
        assert_eq!(
            details::Options::deduce_long(
                &mock_cli(vec!["--long", "--oneline"]),
                &MockVars::default(),
                true
            ),
            Err(OptionsError::Useless("one-line", true, "long"))
        );
    }

    #[test]
    fn deduce_terminal_width_automatic() {
        assert_eq!(
            TerminalWidth::deduce(&mock_cli(vec![""]), &MockVars::default()),
            Ok(Automatic)
        );
    }

    #[test]
    fn deduce_terminal_width_set_arg() {
        assert_eq!(
            TerminalWidth::deduce(&mock_cli(vec!["--width", "80"]), &MockVars::default()),
            Ok(Set(80))
        );
    }

    #[test]
    fn deduce_terminal_width_set_env() {
        let mut vars = MockVars::default();
        vars.set(vars::COLUMNS, &OsString::from("80"));
        assert_eq!(
            TerminalWidth::deduce(&mock_cli(vec![""]), &vars),
            Ok(Set(80))
        );
    }

    #[test]
    fn deduce_terminal_width_set_env_bad() {
        let mut vars = MockVars::default();
        vars.set(vars::COLUMNS, &OsString::from("bad"));

        let e: Result<i64, ParseIntError> =
            vars.get(vars::COLUMNS).unwrap().to_string_lossy().parse();

        assert_eq!(
            TerminalWidth::deduce(&mock_cli(vec![""]), &vars),
            Err(OptionsError::FailedParse(
                String::from("bad"),
                NumberSource::Env(vars::COLUMNS),
                e.unwrap_err()
            ))
        );
    }
}
