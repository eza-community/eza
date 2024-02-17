pub use clap::Parser;
use clap::{builder::TypedValueParser, Arg, Command, Error, ValueEnum};
use std::ffi::OsString;

use crate::output::time::TimeFormat;

#[allow(clippy::struct_excessive_bools)]
#[derive(Default, Parser)]
#[command(author, version, about, long_about)] // Read from `Cargo.toml`
#[clap(disable_help_flag = true, args_override_self = true)]
pub struct Opts {
    pub paths: Vec<OsString>,
    /// Show hidden files.
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub all: u8,
    /// display extended file metadata as a table.
    #[arg(short, long)]
    pub long: bool,
    /// list each file's Git status, if tracked or ignored.
    #[arg(long)]
    pub git: bool,
    /// Display one entry per line.
    #[arg(short = '1', long)]
    pub oneline: bool,
    ///recurse into directories as a tree.
    #[arg(short = 'T', long)]
    pub tree: bool,
    /// display entries as a grid (default).
    #[arg(short = 'G', long)]
    pub grid: bool,
    /// sort the grid across, rather than downwards.
    #[arg(short = 'x', long)]
    pub across: bool,
    /// recurse into directories.
    #[arg(short = 'R', long)]
    pub recurse: bool,
    /// display type indicator by file names.
    #[arg(short = 'F', long)]
    pub classify: bool,
    #[arg(short = 'X', long)]
    pub dereference: bool,
    /// set screen width in columns.
    #[arg(short = 'w', long)]
    pub width: Option<usize>,
    /// when to use terminal colours (always, auto, never).
    #[arg(long, alias = "colour", value_enum, default_value_t = ShowWhen::Auto, default_missing_value = "auto", require_equals = false, num_args=0..=1)]
    pub color: ShowWhen,
    /// highlight levels of 'field' distinctly(all, age, size).
    #[arg(long, alias = "colour-scale", value_enum, default_value = None, default_missing_value = None, num_args = 0..=1, require_equals = false)]
    pub color_scale: Option<ColorScaleArgs>,
    /// use gradient or fixed colors in --color-scale (fixed, gradient)
    #[arg(long, alias = "colour-scale-mode", value_enum, default_value_t = ColorScaleModeArgs::Gradient, default_missing_value = "gradient", num_args = 0..=1, require_equals = false)]
    pub color_scale_mode: ColorScaleModeArgs,
    #[arg(short = 'A', long)]
    pub almost_all: bool,
    /// list directories as files; don't list their contents.
    #[arg(short = 'd', long)]
    pub list_dirs: bool,
    /// limit the depth of recursion.
    #[arg(short = 'L', long)]
    pub level: Option<usize>,
    /// reverse the sort order.
    #[arg(short = 'r', long)]
    pub reverse: bool,
    /// which field to sort by.
    #[arg(short = 's', long, num_args = 0..=1, require_equals = false)]
    pub sort: Option<OsString>, // ValueEnum here means we lose the sort field deducing :/
    /// glob patterns (pipe-separated) of files to ignore.
    #[arg(short = 'I', long)]
    pub ignore_glob: Option<OsString>,
    /// ignore files mentioned in '.gitignore'.
    #[arg(long)]
    pub git_ignore: bool,
    /// list directories before other files.
    #[arg(long = "group-directories-first")]
    pub dirs_first: bool,
    /// list only directories.
    #[arg(short = 'D', long)]
    pub only_dirs: bool,
    /// list file sizes with binary prefixes.
    #[arg(short = 'b', long)]
    pub binary: bool,
    /// list file sizes in bytes, without any prefixes.
    #[arg(short = 'B', long)]
    pub bytes: bool,
    /// list each file's group.
    #[arg(short = 'g', long)]
    pub group: bool,
    /// list numeric user and group IDs.
    #[arg(short = 'n', long)]
    pub numeric: bool,
    /// add a header row to each column.
    #[arg(short = 'h', long)]
    pub header: bool,
    /// display icons
    #[arg(long, default_value = None, default_missing_value = "auto", num_args = 0..=1, require_equals = false)]
    pub icons: Option<ShowWhen>,
    /// list each file's inode number.
    #[arg(short = 'i', long)]
    pub inode: bool,
    /// list each file's number of hard links.
    #[arg(short = 'H', long)]
    pub links: bool,
    /// use the modified timestamp field.
    #[arg(short = 'm', long)]
    pub modified: bool,
    /// use the changed timestamp field.
    #[arg(long)]
    pub changed: bool,
    /// show size of allocated file system blocks.
    #[arg(short = 'S', long)]
    pub blocksize: bool,
    /// which timestamp field to list (modified, accessed, created).
    #[arg(short = 't', long)]
    pub time: Option<OsString>,
    /// use the accessed timestamp field.
    #[arg(short = 'u', long)]
    pub accessed: bool,
    /// use the created timestamp field.
    #[arg(short = 'U', long)]
    pub created: bool,
    /// how to format timestamps (default, iso, long-iso, full-iso, relative).
    #[arg(long, value_enum, default_value = TimeFormat::DefaultFormat, default_missing_value = "default", num_args = 0..=1, require_equals = false, value_parser = TimeFormatParser)]
    pub time_style: Option<TimeFormat>,
    /// display entries as hyperlinks.
    #[arg(long)]
    pub hyperlink: bool,
    /// suppress the permissions field.
    #[arg(long)]
    pub no_permissions: bool,
    /// suppress the filesize field.
    #[arg(long)]
    pub no_filesize: bool,
    /// suppress the user field.
    #[arg(long)]
    pub no_user: bool,
    /// suppress the time field.
    #[arg(long)]
    pub no_time: bool,
    /// suppress git.
    #[arg(long)]
    pub no_git: bool,
    /// list root of git-tree status.
    #[arg(long)]
    pub git_repos: bool,
    ///List each git-repos branch name (much faster)
    #[arg(long)]
    pub git_repos_no_status: bool,
    /// list each file's permission in octal format.
    #[arg(
        short = 'o',
        long,
        alias = "octal-permission",
        alias = "octal-permissions"
    )]
    pub octal: bool,
    /// Display the number of hard links to file.
    #[arg(short = 'Z', long = "context")]
    pub security_context: bool,
    /// Show extended attributes.
    #[arg(short = '@', long)]
    pub extended: bool,
    /// Show list of command-line options.
    #[arg(short ='?', long, action = clap::ArgAction::Help)]
    pub help: (),
    /// Show mount details (Linux only)
    #[arg(short = 'M', long)]
    pub mounts: bool,
    /// Show only files
    #[arg(short = 'f', long)]
    pub only_files: bool,
    /// Don't Show quotes
    #[arg(long)]
    pub no_quotes: bool,
    /// only show group if it has a different name from owner
    #[arg(long)]
    pub smart_group: bool,
    /// show the size of a directory as the size of all files and directories inside
    #[arg(long)]
    pub total_size: bool,
    /// use stdin as the sole input
    #[arg(long = "stdin")]
    pub stdin: bool,
    #[arg(short = 'O', long = "flags")]
    pub file_flags: bool,

    #[arg(long = "no-symlinks")]
    pub no_symlinks: bool,

    #[arg(long = "show-symlinks")]
    pub show_symlinks: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ShowWhen {
    // icons, colors, quotes, headers ? eventually
    Always,
    #[default]
    Auto,
    Never,
}

#[derive(Clone, Debug, Default, ValueEnum, PartialEq, Eq)]
pub enum ColorScaleModeArgs {
    Fixed,
    #[default]
    Gradient,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ColorScaleArgs {
    All,
    Age,
    Size,
}

impl ValueEnum for ColorScaleArgs {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            ColorScaleArgs::All,
            ColorScaleArgs::Age,
            ColorScaleArgs::Size,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(
            match self {
                ColorScaleArgs::All => "all",
                ColorScaleArgs::Age => "age",
                ColorScaleArgs::Size => "size",
            }
            .into(),
        )
    }

    fn from_str(s: &str, _ignore_case: bool) -> Result<Self, String> {
        match s.to_ascii_lowercase().as_str() {
            "all" | "age,size" | "size,age" => Ok(ColorScaleArgs::All),
            "age" => Ok(ColorScaleArgs::Age),
            "size" => Ok(ColorScaleArgs::Size),
            _ => Err(format!("Unknown color-scale value: {s}")),
        }
    }
}

#[derive(Debug, Clone)]
struct TimeFormatParser;
impl TypedValueParser for TimeFormatParser {
    type Value = TimeFormat;

    fn parse_ref(
        &self,
        _cmd: &Command,
        _arg: Option<&Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, Error> {
        match value.to_str().unwrap() {
            "default" => Ok(TimeFormat::DefaultFormat),
            "iso" => Ok(TimeFormat::ISOFormat),
            "long-iso" => Ok(TimeFormat::LongISO),
            "full-iso" => Ok(TimeFormat::FullISO),
            "relative" => Ok(TimeFormat::Relative),
            fmt => {
                if fmt.starts_with('+') {
                    let mut lines = fmt.strip_prefix('+').unwrap().lines();

                    // line 1 will be None when:
                    //   - there is nothing after `+`
                    // line 1 will be empty when:
                    //   - `+` is followed immediately by `\n`
                    let Some(non_recent) = lines.next() else
                        {
                            return Err(Error::raw(
                                clap::error::ErrorKind::InvalidValue,
                                format!(
                                    "Invalid custom timestamp format: {fmt}.\n\
        Please start the format with a plus sign (+) to indicate a custom format.\n\
        For example: +\"%Y-%m-%d %H:%M:%S\"",
                                ),
                            ))
                        };
                    let non_recent = if non_recent.is_empty() {
                        return Err(Error::raw(
                            clap::error::ErrorKind::InvalidValue,
                            format!(
                                "Invalid custom timestamp format: {fmt}.\n\
        Please start the format with a plus sign (+) to indicate a custom format.\n\
        For example: +\"%Y-%m-%d %H:%M:%S\"",
                            ),
                        ));
                    } else {
                        non_recent
                    };

                    // line 2 will be None when:
                    //   - there is not a single `\n`
                    //   - there is nothing after the first `\n`
                    // line 2 will be empty when:
                    //   - there exist at least 2 `\n`, and no content between the 1st and 2nd `\n`
                    let empty_recent_format_msg =
                        "Custom timestamp format for recent files is empty, \
                    please supply a chrono format string at the second line.";
                    let recent = lines.next().map(|rec| {
                        if rec.is_empty() {
                            panic!("{}", empty_recent_format_msg)
                        } else {
                            String::from(rec)
                        }
                    });
                    Ok(TimeFormat::Custom {
                        non_recent: Some(String::from(non_recent)),
                        recent,
                    })
                } else {
                    Err(Error::raw(
                        clap::error::ErrorKind::InvalidValue,
                        format!(
                            "Invalid custom timestamp format: {fmt}.\n\
        Please start the format with a plus sign (+) to indicate a custom format.\n\
        For example: +\"%Y-%m-%d %H:%M:%S\"",
                        ),
                    ))
                }
            }
        }
    }
}
