// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use std::ffi::OsString;

use clap::{arg, builder::PossibleValue, value_parser, Error, ValueEnum};

use crate::{
    fs::filter::{SortCase, SortField},
    output::{file_name::Absolute, time::TimeFormat},
};

const SORT_FIELDS_HELP: &str = "[default: name] [possible values:
  name, Name, .name, .Name, ext, ext, created,
  date, age, accessed, changed,
  size, inode, type, none]";

const TIME_FIELDS_HELP: &str = "[possible values:
  mod|modified, acc|accessed, ch|changed, cr|created]";

const FORMAT_STYLE_FIELDS_HELP: &str = "[possible values:
  default, iso, long-iso, full-iso, relative, \"+<CUSTOM_FORMAT>\"]";

pub fn get_command() -> clap::Command {
    clap::Command::new(clap::crate_name!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        .disable_help_flag(true)
        .disable_version_flag(true)
        .args_override_self(true)

        .arg(arg!([FILES]).value_parser(clap::value_parser!(OsString)).hide_short_help(true))

        .next_help_heading("META OPTIONS")
        .arg(arg!(--stdin "read file names from stdin"))
        .arg(arg!(-'?' --help "Print help").action(clap::ArgAction::HelpShort))
        .arg(arg!(-v --version "Print help").action(clap::ArgAction::Version))

        .next_help_heading("LAYOUT OPTIONS")
        .arg(arg!(-'1' --oneline "display one entry per line"))
        .arg(arg!(-l --long "display extended file metadata as a table"))
        .arg(arg!(-G --grid "display entries as a grid (default)"))
        .arg(arg!(-x --across "sort the grid across, rather than downwards"))
        .arg(arg!(-R --recurse "recurse into directories"))
        .arg(arg!(-T --tree "recurse into directories as a tree"))
        .arg(arg!(-L --level <DEPTH> "limit the depth of recursion")
            .value_parser(value_parser!(usize)))
        .arg(arg!(--"follow-symlinks" "drill down into symbolic links that point to directories"))
        .arg(arg!(-w --width <COLS> "set screen width in columns")
            .value_parser(value_parser!(usize)))

        .next_help_heading("DISPLAY OPTIONS")
        .arg(arg!(-F --classify <WHEN> "display type indicator by file names")
            .num_args(0..=1)
            .value_parser(value_parser!(ShowWhen))
            .default_missing_value("auto"))
        .arg(arg!(-X --dereference  "dereference symbolic links when displaying information"))
        .arg(arg!(--absolute "display entries with their absolute path")
            .num_args(0..=1)
            .action(clap::ArgAction::Set)
            .value_parser(value_parser!(Absolute))
            .default_missing_value("on")
            .default_value("off")
            .hide_default_value(true))
        .arg(arg!(--color <WHEN> "When to use colours.")
            .alias("colour")
            .num_args(0..=1)
            .value_parser(value_parser!(ShowWhen))
            .default_missing_value("auto")
            .default_value("auto"))
        .arg(arg!(--"color-scale" <FIELDS> "highlight value of FIELDS distinctly")
            .num_args(0..)
            .value_parser(value_parser!(ColorScaleArgs))
            .default_missing_value("all")
            .value_delimiter(','))
        .arg(arg!(--"color-scale-mode" <MODE> "mode for --color-scale")
            .num_args(1)
            .value_parser(value_parser!(ColorScaleModeArgs))
            .default_value("gradient"))
        .arg(arg!(--icons <WHEN> "when to display icons")
            .num_args(0..=1)
            .value_parser(value_parser!(ShowWhen))
            .default_missing_value("auto"))
        .arg(arg!(--hyperlink "display entries as hyperlinks"))
        .arg(arg!(--"no-quotes" "don't quote file names with spaces"))

        .next_help_heading("FILTERING OPTIONS")
        .arg(arg!(-a --all... "show hidden files. Use this twice to also show the '.' and '..' directories"))
        .arg(arg!(-A --"almost-all" "equivalent to --all; included for compatibility with `ls -A`"))
        .arg(arg!(-d --"list-dirs" "list directories as files; don't list their contents")
            .conflicts_with_all(["recurse", "tree"]))
        .arg(arg!(-D --"only-dirs" "list only directories"))
        .arg(arg!(-f --"only-files" "list only files"))
        .arg(arg!(--"show-symlinks" "explicitly show symbolic links (with --only-dirs and --only-files)"))
        .arg(arg!(--"no-symlinks" "do not show symbolic links"))
        .arg(arg!(-I --"ignore-glob" <GLOBS> "glob patterns (pipe-separated) of files to ignore"))
        .arg(arg!(--"git-ignore" "ignore files mentioned in '.gitignore'"))

        .next_help_heading("SORTING OPTIONS")
        .arg(arg!(--"group-directories-first" "list directories before other files").id("dirs-first"))
        .arg(arg!(--"group-directories-last" "list directories after other files").id("dirs-last"))
        .arg(arg!(-s --sort <FIELD>)
            .help(format!("which field to sort by {SORT_FIELDS_HELP}"))
            .value_parser(value_parser!(SortField))
            .default_value("name")
            .hide_default_value(true)
            .hide_possible_values(true))
        .arg(arg!(-r --reverse "reverse the sort order"))

        .next_help_heading("LONG VIEW OPTIONS")
        .arg(arg!(-h --header "add a header row to each column"))
        .arg(arg!(-i --inode "list each file's inode number"))
        .arg(arg!(-o --"octal-permissions" "list each file's permission in octal format"))
        .arg(arg!(-H --links "list each file's number of hard links"))
        .arg(arg!(-b --binary "show file sizes with binary prefixes"))
        .arg(arg!(-B --bytes "show file sizes in bytes, without any prefixes"))
        .arg(arg!(--"total-size" "show the size of a directory as the one of its content (unix only)"))
        .arg(arg!(-S --blocksize "list size of allocated file system blocks"))
        .arg(arg!(-g --group "list each file's group"))
        .arg(arg!(--"smart-group" "only show group if it has a different name from owner"))
        .arg(arg!(-n --numeric "show user and group as their numeric IDs"))
        .arg(arg!(-t --time <FIELD>).help(format!("which timestamp field to show {TIME_FIELDS_HELP}"))
            .value_parser(value_parser!(TimeArgs))
            .conflicts_with_all(["modified", "accessed", "changed", "created"])
            .hide_possible_values(true))
        .arg(arg!(-m --modified "show the modified timestamp field (replace default field, combinable)"))
        .arg(arg!(-u --accessed "show the accessed timestamp field (replace default field, combinable)"))
        .arg(arg!(--changed "show the changed timestamp field (replace default field, combinable)"))
        .arg(arg!(-U --created "show the created timestamp field (replace default field, combinable)"))
        .arg(arg!(--"time-style" <STYLE>)
            .help(format!("how to format timestamps {FORMAT_STYLE_FIELDS_HELP}"))
            .value_parser(TimeFormatParser)
            .hide_possible_values(false))
        .arg(arg!(-O --flags "list file flags (Mac, BSD, and Windows only)").id("file-flags"))
        .arg(arg!(-Z --context "list each file's security context").id("security-context"))
        .arg(arg!(--git "list each file's Git status, if tracked or ignored"))
        .arg(arg!(--"git-repos" "list root of git-tree status"))
        .arg(arg!(--"git-repos-no-status" "list each git-repos branch name (much faster)"))
        .arg(arg!(-M --mounts "show mount details (Linux and macOS only)"))
        .arg(arg!(-'@' --extended "list each file's extended attributes and sizes"))
        .arg(arg!(--"no-permissions" "suppress the permissions field"))
        .arg(arg!(--"no-filesize" "suppress the filesize field"))
        .arg(arg!(--"no-user" "suppress the user field"))
        .arg(arg!(--"no-time" "suppress the time field"))
        .arg(arg!(--"no-git" "suppress Git fields (overrides --git, --git-repos, --git-repos-no-status)"))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShowWhen {
    // icons, colors, quotes, headers ? eventually
    Always,
    Auto,
    Never,
}

impl ValueEnum for ShowWhen {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Always, Self::Auto, Self::Never]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(
            match self {
                Self::Always => "always",
                Self::Auto => "auto",
                Self::Never => "never",
            }
            .into(),
        )
    }

    fn from_str(s: &str, _ignore_case: bool) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "" | "auto" | "automatic" => Ok(Self::Auto),
            "always" => Ok(Self::Always),
            "never" => Ok(Self::Never),
            e => Err(String::from(e)),
        }
    }
}

#[derive(Clone, Debug, ValueEnum, PartialEq, Eq)]
pub enum ColorScaleArgs {
    All,
    Age,
    Size,
}

#[derive(Clone, Debug, ValueEnum, PartialEq, Eq)]
pub enum ColorScaleModeArgs {
    Fixed,
    Gradient,
}

impl ValueEnum for SortField {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Name(SortCase::AaBbCc),
            Self::Name(SortCase::ABCabc),
            Self::NameMixHidden(SortCase::AaBbCc),
            Self::NameMixHidden(SortCase::ABCabc),
            Self::Size,
            Self::Extension(SortCase::AaBbCc),
            Self::Extension(SortCase::ABCabc),
            Self::ModifiedDate,
            Self::ModifiedAge,
            Self::ChangedDate,
            Self::AccessedDate,
            Self::CreatedDate,
            #[cfg(unix)]
            Self::FileInode,
            Self::FileType,
            Self::Unsorted,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Name(SortCase::AaBbCc) => PossibleValue::new("name").alias("filename"),
            Self::Name(SortCase::ABCabc) => PossibleValue::new("Name").alias("Filename"),
            Self::NameMixHidden(SortCase::AaBbCc) => PossibleValue::new(".name").alias(".filename"),
            Self::NameMixHidden(SortCase::ABCabc) => PossibleValue::new(".Name").alias(".Filename"),
            Self::Size => PossibleValue::new("size"),
            Self::Extension(SortCase::AaBbCc) => PossibleValue::new("ext").alias("extension"),
            Self::Extension(SortCase::ABCabc) => PossibleValue::new("Ext").alias("Extension"),
            // “new” sorts oldest at the top and newest at the bottom; “old” sorts newest at the
            // top and oldest at the bottom. I think this is the right way round to do this:
            // “size” puts the smallest at  the top and the largest at the bottom, doesn’t it?
            Self::ModifiedDate => {
                PossibleValue::new("date").aliases(vec!["time", "mod", "modified", "new", "newest"])
            }
            // Similarly, “age” means that files with the least age (the newest files) get sorted
            //  at the top, and files with the most age (the oldest) at the bottom.
            Self::ModifiedAge => PossibleValue::new("age").aliases(vec!["old", "oldest"]),
            Self::ChangedDate => PossibleValue::new("changed").alias("ch"),
            Self::AccessedDate => PossibleValue::new("accessed").alias("acc"),
            Self::CreatedDate => PossibleValue::new("created").alias("cr"),
            #[cfg(unix)]
            Self::FileInode => PossibleValue::new("inode"),
            Self::FileType => PossibleValue::new("type"),
            Self::Unsorted => PossibleValue::new("none"),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TimeArgs {
    Modified,
    Changed,
    Accessed,
    Created,
}

impl ValueEnum for TimeArgs {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Modified, Self::Changed, Self::Accessed, Self::Created]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Modified => PossibleValue::new("modified").alias("mod"),
            Self::Changed => PossibleValue::new("changed").alias("ch"),
            Self::Accessed => PossibleValue::new("accessed").alias("acc"),
            Self::Created => PossibleValue::new("created").alias("cr"),
        })
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct TimeFormatParser;

impl clap::builder::ValueParserFactory for TimeFormat {
    type Parser = TimeFormatParser;
    fn value_parser() -> Self::Parser {
        TimeFormatParser
    }
}

impl clap::builder::TypedValueParser for TimeFormatParser {
    type Value = TimeFormat;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        _arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, Error> {
        match TimeFormat::try_from_str(value.to_str().unwrap()) {
            Err(s) => Err(Error::raw(clap::error::ErrorKind::InvalidValue, s).with_cmd(cmd)),
            Ok(v) => Ok(v),
        }
    }
}

impl ValueEnum for Absolute {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::On, Self::Off, Self::Follow]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::On => PossibleValue::new("on").alias("yes"),
            Self::Off => PossibleValue::new("off").alias("no"),
            Self::Follow => PossibleValue::new("follow"),
        })
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    pub fn mock_cli<I, T>(itr: I) -> clap::ArgMatches
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        get_command().no_binary_name(true).get_matches_from(itr)
    }

    pub fn mock_cli_try<I, T>(itr: I) -> Result<clap::ArgMatches, clap::error::Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        get_command().no_binary_name(true).try_get_matches_from(itr)
    }
}
