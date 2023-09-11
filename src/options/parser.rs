pub use clap::Parser;
use std::ffi::OsString;

#[derive(Parser, Default)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
#[clap(disable_help_flag = true)]
pub struct Opts {
    pub paths: Vec<OsString>,
    /// Show hidden files.
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub all: u8,
    /// display extended file metadata as a table.
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub long: u8,
    /// list each file's Git status, if tracked or ignored.
    #[arg(long, action = clap::ArgAction::Count)]
    pub git: u8,
    /// Display one entry per line.
    #[arg(short = '1', long, action = clap::ArgAction::Count)]
    pub oneline: u8,
    ///recurse into directories as a tree.
    #[arg(short = 'T', long, action = clap::ArgAction::Count)]
    pub tree: u8,
    /// display entries as a grid (default).
    #[arg(short = 'G', long, action = clap::ArgAction::Count)]
    pub grid: u8,
    /// sort the grid across, rather than downwards.
    #[arg(short = 'x', long, action = clap::ArgAction::Count)]
    pub across: u8,
    /// recurse into directories.
    #[arg(short = 'R', long, action = clap::ArgAction::Count)]
    pub recurse: u8,
    /// display type indicator by file names.
    #[arg(short = 'F', long, action = clap::ArgAction::Count)]
    pub classify: u8,
    /// 
    #[arg(short = 'X', long, action = clap::ArgAction::Count)]
    pub dereference: u8,
    /// set screen width in columns.
    #[arg(short = 'w', long)]
    pub width: Option<usize>,
    /// when to use terminal colours (always, auto, never).
    #[arg(long)]
    pub color: Option<OsString>,
    /// highlight levels of file sizes distinctly.
    #[arg(long, action = clap::ArgAction::Count)]
    pub color_scale: u8,
    ///
    #[arg(short = 'A', long, action = clap::ArgAction::Count)]
    pub almost_all: u8,
    /// list directories as files; don't list their contents.
    #[arg(short = 'd', long, action = clap::ArgAction::Count)]
    pub list_dirs: u8,
    /// limit the depth of recursion.
    #[arg(short = 'L', long)]
    pub level: Option<usize>,
    /// reverse the sort order.
    #[arg(short = 'r', long, action = clap::ArgAction::Count)]
    pub reverse: u8,
    /// which field to sort by.
    #[arg(short = 's', long)]
    pub sort: Option<OsString>,
    /// glob patterns (pipe-separated) of files to ignore.
    #[arg(short = 'I', long)]
    pub ignore_glob: Option<OsString>,
    /// ignore files mentioned in '.gitignore'.
    #[arg(long = "git-ignore", action = clap::ArgAction::Count)]
    pub git_ignore: u8,
    /// list directories before other files.
    #[arg(long = "group-directories-first", action = clap::ArgAction::Count)]
    pub dirs_first: u8,
    /// list only directories.
    #[arg(short = 'D', long = "only-dirs", action = clap::ArgAction::Count)]
    pub only_dirs: u8,
    /// list file sizes with binary prefixes.
    #[arg(short = 'b', long, action = clap::ArgAction::Count)]
    pub binary: u8,
    /// list file sizes in bytes, without any prefixes.
    #[arg(short = 'B', long, action = clap::ArgAction::Count)]
    pub bytes: u8,
    /// list each file's group.
    #[arg(short = 'g', long, action = clap::ArgAction::Count)]
    pub group: u8,
    /// list numeric user and group IDs.
    #[arg(short = 'n', long, action = clap::ArgAction::Count)]
    pub numeric: u8,
    /// add a header row to each column.
    #[arg(short = 'h', long, action = clap::ArgAction::Count)]
    pub header: u8,
    /// display icons
    #[arg(long, action = clap::ArgAction::Count)]
    pub icons: u8,
    /// list each file's inode number.
    #[arg(short = 'i', long, action = clap::ArgAction::Count)]
    pub inode: u8,
    /// list each file's number of hard links.
    #[arg(short = 'H', long, action = clap::ArgAction::Count)]
    pub links: u8,
    /// use the modified timestamp field.
    #[arg(short = 'm', long, action = clap::ArgAction::Count)]
    pub modified: u8,
    /// use the changed timestamp field.
    #[arg(long, action = clap::ArgAction::Count)]
    pub changed: u8,
    /// show size of allocated file system blocks.
    #[arg(short = 'S', long, action = clap::ArgAction::Count)]
    pub blocksize: u8,
    /// which timestamp field to list (modified, accessed, created).
    #[arg(short = 't', long)]
    pub time: Option<OsString>,
    /// use the accessed timestamp field.
    #[arg(short = 'u', long, action = clap::ArgAction::Count)]
    pub accessed: u8,
    /// use the created timestamp field.
    #[arg(short = 'U', long, action = clap::ArgAction::Count)]
    pub created: u8,
    /// how to format timestamps (default, iso, long-iso, full-iso, relative).
    #[arg(long = "time-style")]
    pub time_style: Option<OsString>,
    /// display entries as hyperlinks.
    #[arg(long, action = clap::ArgAction::Count)]
    pub hyperlink: u8,
    /// supress the permissions field.
    #[arg(long = "no-permissions", action = clap::ArgAction::Count)]
    pub no_permissions: u8,
    /// suppress the filesize field.
    #[arg(long = "no-filesize", action = clap::ArgAction::Count)]
    pub no_filesize: u8,
    /// suppress the user field.
    #[arg(long = "no-user", action = clap::ArgAction::Count)]
    pub no_user: u8,
    /// suppress the time field.
    #[arg(long = "no-time", action = clap::ArgAction::Count)]
    pub no_time: u8,
    /// don't display icons (always override --icons).
    #[arg(long = "no-icons", action = clap::ArgAction::Count)]
    pub no_icons: u8,
    /// supress git.
    #[arg(long = "no-git", action = clap::ArgAction::Count)]
    pub no_git: u8,
    /// list root of git-tree status.
    #[arg(long = "git-repos", action = clap::ArgAction::Count)]
    pub git_repos: u8,
    ///
    #[arg(long = "git-repos-no-status", action = clap::ArgAction::Count)]
    pub git_repos_no_status: u8,
    /// list each file's permission in octal format.
    #[arg(short = 'o', long, action = clap::ArgAction::Count)]
    pub octal: u8,
    /// Display the number of hard links to file.
    #[arg(short = 'Z', long = "context", action = clap::ArgAction::Count)]
    pub security_context: u8,
    /// Show extended attributes.
    #[arg(short = '@', long, action = clap::ArgAction::Count)]
    pub extended: u8,
    /// show list of command-line options.
    #[arg(short ='?', long, action = clap::ArgAction::Help)]
    pub help: Option<bool>,
    /// show mount details (Linux only)
    #[arg(short = 'm', long, action = clap::ArgAction::Count)]
    pub mount: u8,
}

impl Opts {
    pub fn default() -> Opts {
        Opts {
            paths: vec![],
            all: 0,
            long: 0,
            git: 0,
            oneline: 0,
            recurse: 0,
            list_dirs: 0,
            tree: 0,
            level: None,
            reverse: 0,
            sort: None,
            ignore_glob: None,
            git_ignore: 0,
            dirs_first: 0,
            only_dirs: 0,
            binary: 0,
            bytes: 0,
            group: 0,
            numeric: 0,
            grid: 0,
            across: 0,
            classify: 0,
            dereference: 0,
            width: None,
            color: None,
            color_scale: 0,
            almost_all: 0,
            header: 0,
            icons: 0,
            inode: 0,
            git_repos: 0,
            git_repos_no_status: 0,
            links: 0,
            modified: 0,
            created: 0,
            accessed: 0,
            changed: 0,
            blocksize: 0,
            time: None,
            time_style: None,
            no_filesize: 0,
            no_icons: 0,
            no_permissions: 0,
            no_time: 0,
            no_user: 0,
            extended: 0,
            hyperlink: 0,
            octal: 0,
            security_context: 0,
            help: Some(false),
            no_git: 0,
            mount: 0,
        }
    }
}
