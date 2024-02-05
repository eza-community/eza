pub use clap::Parser;
use clap::ValueEnum;
use std::{ffi::OsString, fmt::Display};

use crate::output::time::TimeFormat;

#[derive(Parser)]
#[command(author, version, about, long_about)] // Read from `Cargo.toml`
#[clap(disable_help_flag = true)]
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
    #[arg(long, alias = "colour", value_enum, default_value = ShowWhen::Auto, default_missing_value = ShowWhen::Auto, require_equals = false, num_args=0..=1)]
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
    #[arg(long = "git-ignore")]
    pub git_ignore: bool,
    /// list directories before other files.
    #[arg(long = "group-directories-first")]
    pub dirs_first: bool,
    /// list only directories.
    #[arg(short = 'D', long = "only-dirs")]
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
    #[arg(long, default_value = None, default_missing_value = ShowWhen::Auto, num_args = 0..=1, require_equals = false)]
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
    #[arg(short = 't')]
    pub time: Option<OsString>,
    /// use the accessed timestamp field.
    #[arg(short = 'u', long)]
    pub accessed: bool,
    /// use the created timestamp field.
    #[arg(short = 'U', long)]
    pub created: bool,
    /// how to format timestamps (default, iso, long-iso, full-iso, relative).
    #[arg(long = "time-style", value_enum, default_value = TimeFormat::DefaultFormat, default_missing_value = "default", num_args = 0..=1, require_equals = false)]
    pub time_style: Option<TimeFormat>,
    /// display entries as hyperlinks.
    #[arg(long)]
    pub hyperlink: bool,
    /// suppress the permissions field.
    #[arg(long = "no-permissions")]
    pub no_permissions: bool,
    /// suppress the filesize field.
    #[arg(long = "no-filesize")]
    pub no_filesize: bool,
    /// suppress the user field.
    #[arg(long = "no-user")]
    pub no_user: bool,
    /// suppress the time field.
    #[arg(long = "no-time")]
    pub no_time: bool,
    /// suppress git.
    #[arg(long = "no-git")]
    pub no_git: bool,
    /// list root of git-tree status.
    #[arg(long = "git-repos")]
    pub git_repos: bool,
    ///List each git-repos branch name (much faster)
    #[arg(long = "git-repos-no-status")]
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
    #[arg(short = 'f', long = "only-files")]
    pub only_files: bool,
    /// Don't Show quotes
    #[arg(long = "no-quotes")]
    pub no_quotes: bool,
    /// only show group if it has a different name from owner
    #[arg(long = "smart-group")]
    pub smart_group: bool,
    /// show the size of a directory as the size of all files and directories inside
    #[arg(long = "total-size")]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShowWhen {
    // icons, colors, quotes, headers ? eventually
    Always,
    Auto,
    Never,
}
#[derive(Clone, Debug, ValueEnum, PartialEq, Eq)]
pub enum ColorScaleModeArgs {
    Fixed,
    Gradient,
}

#[derive(Clone, Debug, ValueEnum, PartialEq, Eq)]
pub enum TimeStyleArgs {
    Default,
    Iso,
    LongIso,
    FullIso,
    Relative,
}

#[derive(Clone, Debug, ValueEnum, PartialEq, Eq)]
pub enum SortArgs {
    Name,
    Size,
    Time,
    Extension,
    Inode,
    Version,
    Created,
    Accessed,
    Modified,
    Changed,
}

impl ValueEnum for ShowWhen {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Always, Self::Auto, Self::Never]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::Always => Some(clap::builder::PossibleValue::new("always")),
            Self::Auto => Some(clap::builder::PossibleValue::new("auto")),
            Self::Never => Some(clap::builder::PossibleValue::new("never")),
        }
    }

    fn from_str(s: &str, _ignore_case: bool) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "" => Ok(Self::Auto),
            "always" => Ok(Self::Always),
            "auto" | "automatic" => Ok(Self::Auto),
            "never" => Ok(Self::Never),
            e => Err(String::from(e)),
        }
    }
}

impl Display for ShowWhen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShowWhen::Always => write!(f, "always"),
            ShowWhen::Auto => write!(f, "auto"),
            ShowWhen::Never => write!(f, "never"),
        }
    }
}

impl From<ShowWhen> for clap::builder::OsStr {
    fn from(sw: ShowWhen) -> clap::builder::OsStr {
        match sw {
            ShowWhen::Always => clap::builder::OsStr::from("always"),
            ShowWhen::Auto => clap::builder::OsStr::from("auto"),
            ShowWhen::Never => clap::builder::OsStr::from("never"),
        }
    }
}

impl From<clap::builder::OsStr> for ShowWhen {
    fn from(s: clap::builder::OsStr) -> ShowWhen {
        match s.to_str() {
            Some("always") => ShowWhen::Always,
            Some("auto") => ShowWhen::Auto,
            Some("never") => ShowWhen::Never,
            _ => ShowWhen::Auto,
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
        match self {
            ColorScaleArgs::All => Some(clap::builder::PossibleValue::new("all")),
            ColorScaleArgs::Age => Some(clap::builder::PossibleValue::new("age")),
            ColorScaleArgs::Size => Some(clap::builder::PossibleValue::new("size")),
        }
    }

    fn from_str(s: &str, ignore_case: bool) -> Result<Self, String> {
        if ignore_case {
            match s.to_ascii_lowercase().as_str() {
                "all" | "age,size" | "size,age" => Ok(ColorScaleArgs::All),
                "age" => Ok(ColorScaleArgs::Age),
                "size" => Ok(ColorScaleArgs::Size),
                _ => Err(format!("Unknown color-scale value: {s}")),
            }
        } else {
            match s {
                "all" | "age,size" | "size,age" => Ok(ColorScaleArgs::All),
                "age" => Ok(ColorScaleArgs::Age),
                "size" => Ok(ColorScaleArgs::Size),
                _ => Err(format!("Unknown color-scale value: {s}")),
            }
        }
    }
}

impl Display for ColorScaleArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorScaleArgs::All => write!(f, "all"),
            ColorScaleArgs::Age => write!(f, "age"),
            ColorScaleArgs::Size => write!(f, "size"),
        }
    }
}

impl From<clap::builder::OsStr> for SortArgs {
    fn from(value: clap::builder::OsStr) -> Self {
        match value.to_ascii_lowercase().to_str() {
            Some("name") => SortArgs::Name,
            Some("size") => SortArgs::Size,
            Some("time" | "age" | "date" | "") => SortArgs::Time,
            Some("extension") => SortArgs::Extension,
            Some("inode") => SortArgs::Inode,
            Some("version") => SortArgs::Version,
            Some("created") => SortArgs::Created,
            Some("accessed") => SortArgs::Accessed,
            Some("modified") => SortArgs::Modified,
            Some("changed") => SortArgs::Changed,
            _ => SortArgs::Name,
        }
    }
}

impl SortArgs {
    pub fn as_str(&self) -> &str {
        match self {
            SortArgs::Name => "name",
            SortArgs::Size => "size",
            SortArgs::Time => "time",
            SortArgs::Extension => "extension",
            SortArgs::Inode => "inode",
            SortArgs::Version => "version",
            SortArgs::Created => "created",
            SortArgs::Accessed => "accessed",
            SortArgs::Modified => "modified",
            SortArgs::Changed => "changed",
        }
    }
}

impl From<SortArgs> for clap::builder::OsStr {
    fn from(value: SortArgs) -> Self {
        match value {
            SortArgs::Name => clap::builder::OsStr::from("name"),
            SortArgs::Size => clap::builder::OsStr::from("size"),
            SortArgs::Time => clap::builder::OsStr::from("time"),
            SortArgs::Extension => clap::builder::OsStr::from("extension"),
            SortArgs::Inode => clap::builder::OsStr::from("inode"),
            SortArgs::Version => clap::builder::OsStr::from("version"),
            SortArgs::Created => clap::builder::OsStr::from("created"),
            SortArgs::Accessed => clap::builder::OsStr::from("accessed"),
            SortArgs::Modified => clap::builder::OsStr::from("modified"),
            SortArgs::Changed => clap::builder::OsStr::from("changed"),
        }
    }
}
impl Display for SortArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortArgs::Name => write!(f, "name"),
            SortArgs::Size => write!(f, "size"),
            SortArgs::Time => write!(f, "time"),
            SortArgs::Extension => write!(f, "extension"),
            SortArgs::Inode => write!(f, "inode"),
            SortArgs::Version => write!(f, "version"),
            SortArgs::Created => write!(f, "created"),
            SortArgs::Accessed => write!(f, "accessed"),
            SortArgs::Modified => write!(f, "modified"),
            SortArgs::Changed => write!(f, "changed"),
        }
    }
}
#[derive(Clone, Debug, ValueEnum)]
pub enum TimeArgs {
    Modified,
    Changed,
    Accessed,
    Created,
}
impl Display for TimeArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeArgs::Modified => write!(f, "modified"),
            TimeArgs::Accessed => write!(f, "accessed"),
            TimeArgs::Created => write!(f, "created"),
            TimeArgs::Changed => write!(f, "changed"),
        }
    }
}

impl Default for Opts {
    fn default() -> Self {
        Opts {
            paths: vec![],
            all: 0,
            long: false,
            git: false,
            oneline: false,
            recurse: false,
            list_dirs: false,
            tree: false,
            level: None,
            reverse: false,
            sort: None,
            ignore_glob: None,
            git_ignore: false,
            dirs_first: false,
            only_dirs: false,
            binary: false,
            bytes: false,
            group: false,
            numeric: false,
            grid: false,
            across: false,
            classify: false,
            dereference: false,
            width: None,
            color: ShowWhen::Auto,
            color_scale: None,
            color_scale_mode: ColorScaleModeArgs::Gradient,
            almost_all: false,
            header: false,
            icons: None,
            inode: false,
            git_repos: false,
            git_repos_no_status: false,
            links: false,
            modified: false,
            created: false,
            accessed: false,
            changed: false,
            blocksize: false,
            time: None,
            time_style: None,
            no_filesize: false,
            no_permissions: false,
            no_time: false,
            no_user: false,
            extended: false,
            hyperlink: false,
            octal: false,
            security_context: false,
            help: (),
            no_git: false,
            mounts: false,
            only_files: false,
            no_quotes: false,
            smart_group: false,
            total_size: false,
            stdin: false,
            file_flags: false,
            no_symlinks: false,
            show_symlinks: false,
        }
    }
}
