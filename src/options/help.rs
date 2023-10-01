use std::fmt;

use crate::fs::feature::xattr;
use crate::options::flags;
use crate::options::parser::MatchedFlags;

static INFO: &str = "\
\x1b[1;33meza\x1b[0m a modern replacement for ls

\x1b[2mUsage: \x1b[0;1;33meza\x1b[0;1m [options] [files...]";

static USAGE_PART1: &str = "

\x1b[0;1;4;34mMETA OPTIONS\x1b[0m
  \x1b[1;32m-?, \x1b[36m--help   \x1b[0m      Show list of command-line options
  \x1b[1;32m-v, \x1b[36m--version\x1b[0m      Show version of eza

\x1b[1;4;34mDISPLAY OPTIONS\x1b[0m
\x1b[1;32m  -1, \x1b[36m--oneline       \x1b[0m   Display one entry per line
\x1b[1;32m  -l, \x1b[36m--long          \x1b[0m   Display extended file metadata as a table
\x1b[1;32m  -G, \x1b[36m--grid          \x1b[0m   Display entries as a grid ( default )
\x1b[1;32m  -x, \x1b[36m--across        \x1b[0m   Sort the grid across, rather than downwards
\x1b[1;32m  -R, \x1b[36m--recurse       \x1b[0m   Recurse into directories
\x1b[1;32m  -T, \x1b[36m--tree          \x1b[0m   Recurse into directories as a tree
\x1b[1;32m  -F, \x1b[36m--classify      \x1b[0m   Display type indicator by file names
\x1b[1;32m  -w, \x1b[36m--width\x1b[33;1m COLS\x1b[0m       Set screen width in columns
      \x1b[1;36m--colo[u]r \x1b[33mWHEN \x1b[0m   When to use terminal colours  \x1b[33;1malways, auto, never\x1b[0m
      \x1b[1;36m--colo[u]r-scale \x1b[0m  Highlight levels of file sizes distinctly
      \x1b[1;36m--icons          \x1b[0m  Display icons
      \x1b[1;36m--no-icons       \x1b[0m  Don't display icons ( always overrides \x1b[1;36m--icons\x1b[0m )
      \x1b[1;36m--hyperlink      \x1b[0m  Display entries as hyperlinks

\x1b[1;4;34mFILTERING AND SORTING OPTIONS\x1b[0m 
\x1b[1;32m -a, \x1b[36m--all                \x1b[0m       Show hidden and 'dot' files, 
                                 use this twice to also show the '.' and '..' directories
\x1b[1;32m -d, \x1b[36m--list-dirs          \x1b[0m       List directories as files; don't list their contents
\x1b[1;32m -L, \x1b[36m--level \x1b[33mDEPTH        \x1b[0m       Limit the depth of recursion
\x1b[1;32m -r, \x1b[36m--reverse            \x1b[0m       Reverse the sort order
\x1b[1;32m -s, \x1b[36m--sort \x1b[33mSORT_FIELD    \x1b[0m       Which field to sort by
                                \x1b[33;1m name, Name, extension, Extension, size, type, modified,  accessed
                                 created, inode, none, \x1b[0m{\x1b[33;1m date, time, old, new \x1b[0m( refer to \x1b[33;1mmodified\x1b[0m ) }
  \x1b[1;36m    --group-directories-first\x1b[0m  List directories before other files
  \x1b[1;32m-D, \x1b[36m--only-dirs           \x1b[0m     List only directories
  \x1b[1;32m-I, \x1b[36m--ignore-glob \x1b[33mGLOBS\x1b[0m        Glob patterns ( pipe-separated '\x1b[1;33m|\x1b[0m' ) of files to ignore";

static GIT_FILTER_HELP: &str = "  \
  \x1b[1;36m    --git-ignore\x1b[0m               Ignore files mentioned in '.gitignore'";

static USAGE_PART2: &str = "
\
\x1b[1;4;34mLONG VIEW OPTIONS\x1b[0m
  \x1b[1;32m-b, \x1b[36m--binary               \x1b[0m List file sizes with binary prefixes
  \x1b[1;32m-B, \x1b[36m--bytes                \x1b[0m List file sizes in bytes, without any prefixes
  \x1b[1;32m-g, \x1b[36m--group                \x1b[0m List each file's group
  \x1b[1;32m-h, \x1b[36m--header               \x1b[0m Add a header row to each column
  \x1b[1;32m-H, \x1b[36m--links                \x1b[0m List each file's number of mhard links
  \x1b[1;32m-i, \x1b[36m--inode                \x1b[0m List each file's inode number
  \x1b[1;32m-m, \x1b[36m--modified             \x1b[0m Use the modified timestamp field
  \x1b[1;32m-M, \x1b[36m--mounts               \x1b[0m Show mount details ( Linux and MacOS only )
  \x1b[1;32m-n, \x1b[36m--numeric              \x1b[0m List numeric user and group IDs
  \x1b[1;32m-S, \x1b[36m--blocksize            \x1b[0m Show size of allocated file system blocks
  \x1b[1;32m-t, \x1b[36m--time \x1b[33mFIELD   \x1b[0m         Which timestamp field to list \x1b[33;1m modified, accessed, created \x1b[33;0m
  \x1b[1;32m-u, \x1b[36m--accessed             \x1b[0m Use the accessed timestamp field
  \x1b[1;32m-U, \x1b[36m--created              \x1b[0m Use the created timestamp field
  \x1b[1;36m    --changed              \x1b[0m Use the changed timestamp field
  \x1b[1;36m    --time-style \x1b[33mTIMESTAMP \x1b[0m How to format timestamps \x1b[33;1m default, iso, long-iso, full-iso, relative \x1b[33;0m
  \x1b[1;36m    --no-permissions       \x1b[0m Suppress the permissions field
  \x1b[1;32m-o, \x1b[36m--octal-permissions    \x1b[0m List each file's permission in octal format
  \x1b[1;36m    --no-filesize          \x1b[0m Suppress the filesize field
  \x1b[1;36m    --no-user              \x1b[0m Suppress the user field
  \x1b[1;36m    --no-time              \x1b[0m Suppress the time field";

static GIT_VIEW_HELP:   &str = "  \
  \x1b[1;36m    --git                  \x1b[0m List each file's Git status, if tracked or ignored
  \x1b[1;36m    --no-git               \x1b[0m Suppress Git status ( always overrides \x1b[1;36m--git, --git-repos, --git-repos-no-status \x1b[0m)
  \x1b[1;36m    --git-repos            \x1b[0m List root of git-tree status";

static EXTENDED_HELP:   &str = "  \
  \x1b[1;32m-@, \x1b[36m--extended     \x1b[0m         List each file's extended attributes and sizes";

static SECATTR_HELP: &str = "  \
  \x1b[1;32m-Z, \x1b[36m--context      \x1b[0m         List each file's security context";

/// All the information needed to display the help text, which depends
/// on which features are enabled and whether the user only wants to
/// see one section’s help.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct HelpString;

impl HelpString {
    /// Determines how to show help, if at all, based on the user’s
    /// command-line arguments. This one works backwards from the other
    /// ‘deduce’ functions, returning Err if help needs to be shown.
    ///
    /// We don’t do any strict-mode error checking here: it’s OK to give
    /// the --help or --long flags more than once. Actually checking for
    /// errors when the user wants help is kind of petty!
    pub fn deduce(matches: &MatchedFlags<'_>) -> Option<Self> {
        if matches.count(&flags::HELP) > 0 {
            Some(Self)
        } else {
            None
        }
    }
}

impl fmt::Display for HelpString {
    /// Format this help options into an actual string of help
    /// text to be displayed to the user.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{INFO}")?;

        write!(f, "{USAGE_PART1}")?;

        if cfg!(feature = "git") {
            write!(f, "\n{GIT_FILTER_HELP}")?;
        }

        write!(f, "\n{USAGE_PART2}")?;

        if cfg!(feature = "git") {
            write!(f, "\n{GIT_VIEW_HELP}")?;
        }

        if xattr::ENABLED {
            write!(f, "\n{EXTENDED_HELP}")?;
            write!(f, "\n{SECATTR_HELP}")?;
        }

        writeln!(f)
    }
}

#[cfg(test)]
mod test {
    use crate::options::{Options, OptionsResult};
    use std::ffi::OsStr;

    #[test]
    fn help() {
        let args = vec![OsStr::new("--help")];
        let opts = Options::parse(args, &None);
        assert!(matches!(opts, OptionsResult::Help(_)));
    }

    #[test]
    fn help_with_file() {
        let args = vec![OsStr::new("--help"), OsStr::new("me")];
        let opts = Options::parse(args, &None);
        assert!(matches!(opts, OptionsResult::Help(_)));
    }

    #[test]
    fn unhelpful() {
        let args = vec![];
        let opts = Options::parse(args, &None);
        assert!(!matches!(opts, OptionsResult::Help(_))) // no help when --help isn’t passed
    }
}
