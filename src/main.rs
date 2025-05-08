// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
#![warn(deprecated_in_future)]
#![warn(future_incompatible)]
#![warn(nonstandard_style)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::enum_glob_use)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::non_ascii_literal)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unused_self)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::wildcard_imports)]

use std::env;
use std::ffi::{OsStr, OsString};
use std::io::{self, stdin, ErrorKind, IsTerminal, Read, Write};
use std::path::{Component, PathBuf};
use std::process::exit;

use nu_ansi_term::{AnsiStrings as ANSIStrings, Style};

use crate::fs::feature::git::GitCache;
use crate::fs::filter::GitIgnore;
use crate::fs::{Dir, File};
use crate::options::stdin::FilesInput;
use crate::options::{vars, Options, OptionsResult, Vars};
use crate::output::{details, escape, file_name, grid, grid_details, lines, Mode, View};
use crate::theme::Theme;
use log::*;

mod fs;
mod info;
mod logger;
mod options;
mod output;
mod theme;

fn main() {
    #[cfg(unix)]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    logger::configure(env::var_os(vars::EZA_DEBUG).or_else(|| env::var_os(vars::EXA_DEBUG)));

    let stdout_istty = io::stdout().is_terminal();

    let mut input = String::new();
    let args: Vec<_> = env::args_os().skip(1).collect();
    match Options::parse(args.iter().map(std::convert::AsRef::as_ref), &LiveVars) {
        OptionsResult::Ok(options, mut input_paths) => {
            // List the current directory by default.
            // (This has to be done here, otherwise git_options won’t see it.)
            if input_paths.is_empty() {
                match &options.stdin {
                    FilesInput::Args => {
                        input_paths = vec![OsStr::new(".")];
                    }
                    FilesInput::Stdin(separator) => {
                        stdin()
                            .read_to_string(&mut input)
                            .expect("Failed to read from stdin");
                        input_paths.extend(
                            input
                                .split(&separator.clone().into_string().unwrap_or("\n".to_string()))
                                .map(std::ffi::OsStr::new)
                                .filter(|s| !s.is_empty())
                                .collect::<Vec<_>>(),
                        );
                    }
                }
            }

            let git = git_options(&options, &input_paths);
            let writer = io::stdout();
            let git_repos = git_repos(&options, &input_paths);

            let console_width = options.view.width.actual_terminal_width();
            let theme = options.theme.to_theme(stdout_istty);
            let exa = Exa {
                options,
                writer,
                input_paths,
                theme,
                console_width,
                git,
                git_repos,
            };

            info!("matching on exa.run");
            match exa.run() {
                Ok(exit_status) => {
                    trace!("exa.run: exit Ok({exit_status})");
                    exit(exit_status);
                }

                Err(e) if e.kind() == ErrorKind::BrokenPipe => {
                    warn!("Broken pipe error: {e}");
                    exit(exits::SUCCESS);
                }

                Err(e) => {
                    eprintln!("{e}");
                    trace!("exa.run: exit RUNTIME_ERROR");
                    exit(exits::RUNTIME_ERROR);
                }
            }
        }

        OptionsResult::Help(help_text) => {
            print!("{help_text}");
        }

        OptionsResult::Version(version_str) => {
            print!("{version_str}");
        }

        OptionsResult::InvalidOptions(error) => {
            eprintln!("eza: {error}");

            if let Some(s) = error.suggestion() {
                eprintln!("{s}");
            }

            exit(exits::OPTIONS_ERROR);
        }
    }
}

/// The main program wrapper.
pub struct Exa<'args> {
    /// List of command-line options, having been successfully parsed.
    pub options: Options,

    /// The output handle that we write to.
    pub writer: io::Stdout,

    /// List of the free command-line arguments that should correspond to file
    /// names (anything that isn’t an option).
    pub input_paths: Vec<&'args OsStr>,

    /// The theme that has been configured from the command-line options and
    /// environment variables. If colours are disabled, this is a theme with
    /// every style set to the default.
    pub theme: Theme,

    /// The detected width of the console. This is used to determine which
    /// view to use.
    pub console_width: Option<usize>,

    /// A global Git cache, if the option was passed in.
    /// This has to last the lifetime of the program, because the user might
    /// want to list several directories in the same repository.
    pub git: Option<GitCache>,

    pub git_repos: bool,
}

/// The “real” environment variables type.
/// Instead of just calling `var_os` from within the options module,
/// the method of looking up environment variables has to be passed in.
struct LiveVars;
impl Vars for LiveVars {
    fn get(&self, name: &'static str) -> Option<OsString> {
        env::var_os(name)
    }
}

/// Create a Git cache populated with the arguments that are going to be
/// listed before they’re actually listed, if the options demand it.
fn git_options(options: &Options, args: &[&OsStr]) -> Option<GitCache> {
    if options.should_scan_for_git() {
        Some(args.iter().map(PathBuf::from).collect())
    } else {
        None
    }
}

#[cfg(not(feature = "git"))]
fn git_repos(_options: &Options, _args: &[&OsStr]) -> bool {
    return false;
}

#[cfg(feature = "git")]
fn get_files_in_dir(paths: &mut Vec<PathBuf>, path: PathBuf) {
    let temp_paths = if path.is_dir() {
        match path.read_dir() {
            Err(_) => {
                vec![path]
            }
            Ok(d) => d
                .filter_map(|entry| entry.ok().map(|e| e.path()))
                .collect::<Vec<PathBuf>>(),
        }
    } else {
        vec![path]
    };
    paths.extend(temp_paths);
}

#[cfg(feature = "git")]
fn git_repos(options: &Options, args: &[&OsStr]) -> bool {
    let option_enabled = match options.view.mode {
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
        }) => table.columns.subdir_git_repos || table.columns.subdir_git_repos_no_stat,
        _ => false,
    };
    if option_enabled {
        let paths: Vec<PathBuf> = args.iter().map(PathBuf::from).collect::<Vec<PathBuf>>();
        let mut files: Vec<PathBuf> = Vec::new();
        for path in paths {
            get_files_in_dir(&mut files, path);
        }
        let repos: Vec<bool> = files
            .iter()
            .map(git2::Repository::open)
            .map(|repo| repo.is_ok())
            .collect();
        repos.contains(&true)
    } else {
        false
    }
}

impl<'args> Exa<'args> {
    /// # Errors
    ///
    /// Will return `Err` if printing to stderr fails.
    pub fn run(mut self) -> io::Result<i32> {
        debug!("Running with options: {:#?}", self.options);

        let mut files = Vec::new();
        let mut dirs = Vec::new();
        let mut exit_status = 0;

        for file_path in &self.input_paths {
            let f = File::from_args(
                PathBuf::from(file_path),
                None,
                None,
                self.options.view.deref_links,
                self.options.view.total_size,
                None,
            );

            // We don't know whether this file exists, so we have to try to get
            // the metadata to verify.
            if let Err(e) = f.metadata() {
                exit_status = 2;
                writeln!(io::stderr(), "{file_path:?}: {e}")?;
                continue;
            }

            if f.points_to_directory() && !self.options.dir_action.treat_dirs_as_files() {
                trace!("matching on to_dir");
                match f.to_dir() {
                    Ok(d) => dirs.push(d),
                    Err(e) if e.kind() == ErrorKind::PermissionDenied => {
                        eprintln!("{file_path:?}: {e}");
                        exit(exits::PERMISSION_DENIED);
                    }
                    Err(e) => writeln!(io::stderr(), "{file_path:?}: {e}")?,
                }
            } else {
                files.push(f);
            }
        }

        // We want to print a directory’s name before we list it, *except* in
        // the case where it’s the only directory, *except* if there are any
        // files to print as well. (It’s a double negative)

        let no_files = files.is_empty();
        let is_only_dir = dirs.len() == 1 && no_files;

        self.options.filter.filter_argument_files(&mut files);
        self.print_files(None, files)?;

        self.print_dirs(dirs, no_files, is_only_dir, exit_status)
    }

    fn print_dirs(
        &mut self,
        dir_files: Vec<Dir>,
        mut first: bool,
        is_only_dir: bool,
        exit_status: i32,
    ) -> io::Result<i32> {
        let View {
            file_style: file_name::Options { quote_style, .. },
            ..
        } = self.options.view;
        for dir in dir_files {
            // Put a gap between directories, or between the list of files and
            // the first directory.
            if first {
                first = false;
            } else {
                writeln!(&mut self.writer)?;
            }

            if !is_only_dir {
                let mut bits = Vec::new();
                escape(
                    dir.path.display().to_string(),
                    &mut bits,
                    Style::default(),
                    Style::default(),
                    quote_style,
                );
                writeln!(&mut self.writer, "{}:", ANSIStrings(&bits))?;
            }

            let mut children = Vec::new();
            let git_ignore = self.options.filter.git_ignore == GitIgnore::CheckAndIgnore;
            for file in dir.files(
                self.options.filter.dot_filter,
                self.git.as_ref(),
                git_ignore,
                self.options.view.deref_links,
                self.options.view.total_size,
            ) {
                children.push(file);
            }
            let recursing = self.options.dir_action.recurse_options().is_some();
            self.options
                .filter
                .filter_child_files(recursing, &mut children);
            self.options.filter.sort_files(&mut children);

            if let Some(recurse_opts) = self.options.dir_action.recurse_options() {
                let depth = dir
                    .path
                    .components()
                    .filter(|&c| c != Component::CurDir)
                    .count()
                    + 1;
                let follow_links = self.options.view.follow_links;
                if !recurse_opts.tree && !recurse_opts.is_too_deep(depth) {
                    let mut child_dirs = Vec::new();
                    for child_dir in children.iter().filter(|f| {
                        (if follow_links {
                            f.points_to_directory()
                        } else {
                            f.is_directory()
                        }) && !f.is_all_all
                    }) {
                        match child_dir.to_dir() {
                            Ok(d) => child_dirs.push(d),
                            Err(e) => {
                                writeln!(io::stderr(), "{}: {}", child_dir.path.display(), e)?;
                            }
                        }
                    }

                    self.print_files(Some(&dir), children)?;
                    match self.print_dirs(child_dirs, false, false, exit_status) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }
                    continue;
                }
            }

            self.print_files(Some(&dir), children)?;
        }

        Ok(exit_status)
    }

    /// Prints the list of files using whichever view is selected.
    fn print_files(&mut self, dir: Option<&Dir>, files: Vec<File<'_>>) -> io::Result<()> {
        if files.is_empty() {
            return Ok(());
        }
        let theme = &self.theme;
        let View {
            ref mode,
            ref file_style,
            ..
        } = self.options.view;

        match (mode, self.console_width) {
            (Mode::Grid(ref opts), Some(console_width)) => {
                let filter = &self.options.filter;
                let r = grid::Render {
                    files,
                    theme,
                    file_style,
                    opts,
                    console_width,
                    filter,
                };
                r.render(&mut self.writer)
            }

            (Mode::Grid(_), None) | (Mode::Lines, _) => {
                let filter = &self.options.filter;
                let r = lines::Render {
                    files,
                    theme,
                    file_style,
                    filter,
                };
                r.render(&mut self.writer)
            }

            (Mode::Details(ref opts), _) => {
                let filter = &self.options.filter;
                let recurse = self.options.dir_action.recurse_options();

                let git_ignoring = self.options.filter.git_ignore == GitIgnore::CheckAndIgnore;
                let git = self.git.as_ref();
                let git_repos = self.git_repos;
                let r = details::Render {
                    dir,
                    files,
                    theme,
                    file_style,
                    opts,
                    recurse,
                    filter,
                    git_ignoring,
                    git,
                    git_repos,
                };
                r.render(&mut self.writer)
            }

            (Mode::GridDetails(ref opts), Some(console_width)) => {
                let details = &opts.details;
                let row_threshold = opts.row_threshold;

                let filter = &self.options.filter;
                let git_ignoring = self.options.filter.git_ignore == GitIgnore::CheckAndIgnore;
                let git = self.git.as_ref();
                let git_repos = self.git_repos;

                let r = grid_details::Render {
                    dir,
                    files,
                    theme,
                    file_style,
                    details,
                    filter,
                    row_threshold,
                    git_ignoring,
                    git,
                    console_width,
                    git_repos,
                };
                r.render(&mut self.writer)
            }

            (Mode::GridDetails(ref opts), None) => {
                let opts = &opts.to_details_options();
                let filter = &self.options.filter;
                let recurse = self.options.dir_action.recurse_options();
                let git_ignoring = self.options.filter.git_ignore == GitIgnore::CheckAndIgnore;
                let git = self.git.as_ref();
                let git_repos = self.git_repos;

                let r = details::Render {
                    dir,
                    files,
                    theme,
                    file_style,
                    opts,
                    recurse,
                    filter,
                    git_ignoring,
                    git,
                    git_repos,
                };
                r.render(&mut self.writer)
            }
        }
    }
}

mod exits {

    /// Exit code for when exa runs OK.
    pub const SUCCESS: i32 = 0;

    /// Exit code for when there was at least one I/O error during execution.
    pub const RUNTIME_ERROR: i32 = 1;

    /// Exit code for when the command-line options are invalid.
    pub const OPTIONS_ERROR: i32 = 3;

    /// Exit code for missing file permissions
    pub const PERMISSION_DENIED: i32 = 13;
}
