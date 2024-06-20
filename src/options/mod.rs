//! Parsing command-line strings into exa options.
//!
//! This module imports exa’s configuration types, such as `View` (the details
//! of displaying multiple files) and `DirAction` (what to do when encountering
//! a directory), and implements `deduce` methods on them so they can be
//! configured using command-line options.
//!
//!
//! ## Useless and overridden options
//!
//! Let’s say exa was invoked with just one argument: `exa --inode`. The
//! `--inode` option is used in the details view, where it adds the inode
//! column to the output. But because the details view is *only* activated with
//! the `--long` argument, adding `--inode` without it would not have any
//! effect.
//!
//! For a long time, exa’s philosophy was that the user should be warned
//! whenever they could be mistaken like this. If you tell exa to display the
//! inode, and it *doesn’t* display the inode, isn’t that more annoying than
//! having it throw an error back at you?
//!
//! However, this doesn’t take into account *configuration*. Say a user wants
//! to configure exa so that it lists inodes in the details view, but otherwise
//! functions normally. A common way to do this for command-line programs is to
//! define a shell alias that specifies the details they want to use every
//! time. For the inode column, the alias would be:
//!
//! `alias exa="exa --inode"`
//!
//! Using this alias means that although the inode column will be shown in the
//! details view, you’re now *only* allowed to use the details view, as any
//! other view type will result in an error. Oops!
//!
//! Another example is when an option is specified twice, such as `exa
//! --sort=Name --sort=size`. Did the user change their mind about sorting, and
//! accidentally specify the option twice?
//!
//! Again, exa rejected this case, throwing an error back to the user instead
//! of trying to guess how they want their output sorted. And again, this
//! doesn’t take into account aliases being used to set defaults. A user who
//! wants their files to be sorted case-insensitively may configure their shell
//! with the following:
//!
//! `alias exa="exa --sort=Name"`
//!
//! Just like the earlier example, the user now can’t use any other sort order,
//! because exa refuses to guess which one they meant. It’s *more* annoying to
//! have to go back and edit the command than if there were no error.
//!
//! Fortunately, there’s a heuristic for telling which options came from an
//! alias and which came from the actual command-line: aliased options are
//! nearer the beginning of the options array, and command-line options are
//! nearer the end. This means that after the options have been parsed, exa
//! needs to traverse them *backwards* to find the last-most-specified one.
//!
//! For example, invoking exa with `exa --sort=size` when that alias is present
//! would result in a full command-line of:
//!
//! `exa --sort=Name --sort=size`
//!
//! `--sort=size` should override `--sort=Name` because it’s closer to the end
//! of the arguments array. In fact, because there’s no way to tell where the
//! arguments came from — it’s just a heuristic — this will still work even
//! if no aliases are being used!
//!
//! Finally, this isn’t just useful when options could override each other.
//! Creating an alias `exal="exa --long --inode --header"` then invoking `exal
//! --grid --long` shouldn’t complain about `--long` being given twice when
//! it’s clear what the user wants.

use crate::fs::dir_action::DirAction;
use crate::fs::filter::{FileFilter, GitIgnore};
use crate::options::stdin::FilesInput;
use crate::output::{details, grid_details, Mode, View};
use crate::theme::Options as ThemeOptions;

mod dir_action;
mod file_name;
mod filter;
#[rustfmt::skip] // this module becomes unreadable with rustfmt
mod theme;
mod view;

mod error;
pub use self::error::{NumberSource, OptionsError};

pub mod parser;
use crate::options::parser::Opts;

pub mod vars;
pub use self::vars::Vars;

pub mod stdin;
/// These **options** represent a parsed, error-checked versions of the
/// user’s command-line options.
#[derive(Debug)]
pub struct Options {
    /// The action to perform when encountering a directory rather than a
    /// regular file.
    pub dir_action: DirAction,

    /// How to sort and filter files before outputting them.
    pub filter: FileFilter,

    /// The user’s preference of view to use (lines, grid, details, or
    /// grid-details) along with the options on how to render file names.
    /// If the view requires the terminal to have a width, and there is no
    /// width, then the view will be downgraded.
    pub view: View,

    /// The options to make up the styles of the UI and file names.
    pub theme: ThemeOptions,

    /// Whether to read file names from stdin instead of the command-line
    pub stdin: FilesInput,
}

impl Options {
    /// Whether the View specified in this set of options includes a Git
    /// status column. It’s only worth trying to discover a repository if the
    /// results will end up being displayed.
    pub fn should_scan_for_git(&self) -> bool {
        if self.filter.git_ignore == GitIgnore::CheckAndIgnore {
            return true;
        }

        match self.view.mode {
            Mode::Details(details::Options {
                table: Some(ref table),
                ..
            })
            | Mode::GridDetails(grid_details::Options {
                details:
                    details::Options {
                        table: Some(ref table),
                        ..
                    },
                ..
            }) => table.columns.git,
            _ => false,
        }
    }

    /// Determines the complete set of options based on the given command-line
    /// arguments, after they’ve been parsed.
    pub fn deduce<V: Vars>(matches: &Opts, vars: &V) -> Result<Self, OptionsError> {
        if cfg!(not(feature = "git")) && (matches.git || matches.git_ignore) {
            return Err(OptionsError::Unsupported(String::from(
                "Options --git and --git-ignore can't be used because `git` feature was disabled in this build of exa"
            )));
        }
        let strict = vars
            .get_with_fallback(vars::EXA_STRICT, vars::EZA_STRICT)
            .is_some();

        let view = View::deduce(matches, vars, strict)?;
        let dir_action = DirAction::deduce(matches, matches!(view.mode, Mode::Details(_)), strict)?;
        let filter = FileFilter::deduce(matches, strict)?;
        let theme = ThemeOptions::deduce(matches, vars);
        let stdin = FilesInput::deduce(matches, vars);

        Ok(Self {
            dir_action,
            filter,
            view,
            theme,
            stdin,
        })
    }
}
