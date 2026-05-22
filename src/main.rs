// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
#![warn(future_incompatible)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(clippy::all)]
#![allow(clippy::non_ascii_literal)]

use std::env;
use std::ffi::{OsStr, OsString};
use std::fs as stdfs;
use std::io::{self, ErrorKind, IsTerminal, Read, Write, stdin};
use std::path::{Component, Path, PathBuf};
use std::process::exit;

use nu_ansi_term::{AnsiStrings as ANSIStrings, Style};
use options::parser::get_command;

use crate::fs::feature::git::GitCache;
use crate::fs::filter::{FileFilterFlags::OnlyFiles, GitIgnore};
use crate::fs::{Dir, File};
use crate::options::stdin::FilesInput;
use crate::options::{Options, Vars, vars};
use crate::output::{Mode, View, details, escape, file_name, grid, grid_details, lines};
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

    let cli = get_command().get_matches();

    let stdout_istty = io::stdout().is_terminal();
    let mut input = String::new();
    let mut input_paths: Vec<&OsStr> = match cli.get_many("FILE") {
        Some(x) => x.map(OsString::as_os_str).collect(),
        None => vec![],
    };
    match Options::deduce(&cli, &LiveVars) {
        Ok(options) => {
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
                                .map(OsStr::new)
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
        Err(error) => {
            eprintln!("eza: {error}");
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
    if !options.should_scan_for_git() {
        return None;
    }
    let mut paths: Vec<PathBuf> = args.iter().map(PathBuf::from).collect();

    // When --git-ignore is on AND we’re recursing, also pre-discover child
    // Git repositories so their `.gitignore` files are honored during the
    // traversal. Without this, `eza --tree --git-ignore` run from a parent
    // of a repository misses that repository’s `.gitignore` because
    // `GitRepo::discover` only walks UP from the input paths. See #1086.
    if options.filter.git_ignore == GitIgnore::CheckAndIgnore
        && let Some(recurse) = options.dir_action.recurse_options()
    {
        let max_depth = recurse.max_depth.unwrap_or(usize::MAX);
        let mut extra: Vec<PathBuf> = Vec::new();
        for path in &paths {
            collect_child_git_repos(path, max_depth, &mut extra);
        }
        paths.extend(extra);
        // Process deepest paths first so a child repo’s discovery
        // isn’t skipped by `GitCache`’s "already covered by an
        // existing repo" shortcut when its parent is also in the list
        // (e.g. listing from a parent of a submodule).
        paths.sort_by_key(|p| std::cmp::Reverse(p.components().count()));
    }

    Some(paths.into_iter().collect())
}

/// Walk down `start` looking for directories that are Git repositories
/// (contain a `.git` directory or file — the latter is used by submodules
/// and worktrees) and push them to `out`. Bounded by `max_depth` which
/// matches the `--level` flag the user already opted into; depth `0` is
/// `start` itself.
fn collect_child_git_repos(start: &Path, max_depth: usize, out: &mut Vec<PathBuf>) {
    fn walk(dir: &Path, depth: usize, max_depth: usize, out: &mut Vec<PathBuf>) {
        let entries = match stdfs::read_dir(dir) {
            Ok(e) => e,
            Err(_) => return,
        };

        let mut has_git = false;
        let mut subdirs: Vec<PathBuf> = Vec::new();
        for entry in entries.flatten() {
            let name = entry.file_name();
            if name == ".git" {
                has_git = true;
            }
            // Only descend into real subdirectories. Skip dotfiles to avoid
            // pointlessly walking into `.git` itself and other hidden trees.
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                let name_str = name.to_string_lossy();
                if !name_str.starts_with('.') {
                    subdirs.push(entry.path());
                }
            }
        }

        if has_git {
            out.push(dir.to_path_buf());
        }
        if depth >= max_depth {
            return;
        }
        for sub in subdirs {
            walk(&sub, depth + 1, max_depth, out);
        }
    }
    walk(start, 0, max_depth, out);
}

#[cfg(test)]
mod tests {
    use super::collect_child_git_repos;
    use std::fs;
    use std::path::{Path, PathBuf};

    /// Create a directory containing an empty `.git` directory at `path`,
    /// simulating a Git repository for the walk-down’s purposes (it only
    /// checks for the literal existence of `.git`, not validity).
    fn mark_as_repo(path: &Path) {
        fs::create_dir_all(path.join(".git")).unwrap();
    }

    /// Create a temp directory unique to this test, returning its path. The
    /// caller is responsible for cleanup; for these tests we leave it on
    /// disk since `std::env::temp_dir()` is auto-cleared periodically and
    /// adding a `tempfile` dependency for one test isn’t worth it.
    fn temp_workdir(label: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("eza-test-{}-{}", label, std::process::id(),));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).unwrap();
        path
    }

    #[test]
    fn finds_child_repo_under_non_repo_parent() {
        let root = temp_workdir("child-repo");
        mark_as_repo(&root.join("repo"));
        let mut out = Vec::new();
        collect_child_git_repos(&root, usize::MAX, &mut out);
        assert_eq!(out, vec![root.join("repo")]);
    }

    #[test]
    fn finds_nested_repo() {
        let root = temp_workdir("nested");
        mark_as_repo(&root.join("a/b/c/repo"));
        let mut out = Vec::new();
        collect_child_git_repos(&root, usize::MAX, &mut out);
        assert_eq!(out, vec![root.join("a/b/c/repo")]);
    }

    #[test]
    fn finds_sibling_repos() {
        let root = temp_workdir("siblings");
        mark_as_repo(&root.join("alpha"));
        mark_as_repo(&root.join("beta"));
        let mut out = Vec::new();
        collect_child_git_repos(&root, usize::MAX, &mut out);
        out.sort();
        assert_eq!(out, vec![root.join("alpha"), root.join("beta")]);
    }

    #[test]
    fn includes_start_when_it_is_a_repo() {
        let root = temp_workdir("start-is-repo");
        mark_as_repo(&root);
        mark_as_repo(&root.join("submod"));
        let mut out = Vec::new();
        collect_child_git_repos(&root, usize::MAX, &mut out);
        out.sort();
        let mut expected = vec![root.clone(), root.join("submod")];
        expected.sort();
        assert_eq!(out, expected);
    }

    #[test]
    fn respects_max_depth() {
        let root = temp_workdir("max-depth");
        mark_as_repo(&root.join("a/b/c/repo")); // depth 4
        let mut shallow = Vec::new();
        collect_child_git_repos(&root, 2, &mut shallow);
        assert!(shallow.is_empty(), "depth 2 should miss repo at depth 4");
        let mut deep = Vec::new();
        collect_child_git_repos(&root, 5, &mut deep);
        assert_eq!(deep, vec![root.join("a/b/c/repo")]);
    }

    #[test]
    fn handles_dot_git_as_a_file() {
        // Submodules use a `.git` *file* whose contents point to the real
        // gitdir. The walker only checks existence of the entry, so files
        // work the same as directories.
        let root = temp_workdir("dot-git-file");
        let submod = root.join("submod");
        fs::create_dir_all(&submod).unwrap();
        fs::write(submod.join(".git"), "gitdir: ../.git/modules/submod\n").unwrap();
        let mut out = Vec::new();
        collect_child_git_repos(&root, usize::MAX, &mut out);
        assert_eq!(out, vec![submod]);
    }

    #[test]
    fn does_not_descend_into_dot_directories() {
        let root = temp_workdir("dot-dir");
        // A `.git/` containing nested directories shouldn’t be searched.
        // If we descended into it we’d crash on permission-denied or worse,
        // pretend the user’s `.git/modules/foo` is a normal child repo.
        let bogus = root.join(".git/modules/inner");
        fs::create_dir_all(&bogus).unwrap();
        let mut out = Vec::new();
        collect_child_git_repos(&root, usize::MAX, &mut out);
        // The root itself has `.git`, so it counts; nothing under it should.
        assert_eq!(out, vec![root.clone()]);
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

impl Exa<'_> {
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
                trace!("matching on new Dir");
                dirs.push(f.to_dir());
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

        let mut denied_dirs = vec![];

        for mut dir in dir_files {
            let dir = match dir.read() {
                Ok(dir) => dir,
                Err(e) => {
                    if e.kind() == ErrorKind::PermissionDenied {
                        eprintln!(
                            "Permission denied: {} - code: {}",
                            dir.path.display(),
                            exits::PERMISSION_DENIED
                        );
                        denied_dirs.push(dir.path);
                        continue;
                    }

                    eprintln!("{}: {}", dir.path.display(), e);
                    continue;
                }
            };

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
                    let child_dirs = children
                        .iter()
                        .filter(|f| {
                            (if follow_links {
                                f.points_to_directory()
                            } else {
                                f.is_directory()
                            }) && !f.is_all_all
                        })
                        .map(fs::File::to_dir)
                        .collect::<Vec<Dir>>();

                    self.print_files(Some(dir), children)?;
                    match self.print_dirs(child_dirs, false, false, exit_status) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }
                    continue;
                }
            }

            self.print_files(Some(dir), children)?;
        }

        if !denied_dirs.is_empty() {
            eprintln!(
                "\nSkipped {} directories due to permission denied: ",
                denied_dirs.len()
            );
            for path in denied_dirs {
                eprintln!("  {}", path.display());
            }
        }

        Ok(exit_status)
    }

    /// Prints the list of files using whichever view is selected.
    fn print_files(&mut self, dir: Option<&Dir>, mut files: Vec<File<'_>>) -> io::Result<()> {
        if files.is_empty() {
            return Ok(());
        }
        let recursing = self.options.dir_action.recurse_options().is_some();
        let only_files = self.options.filter.flags.contains(&OnlyFiles);
        if recursing && only_files {
            files = files
                .into_iter()
                .filter(|f| !f.is_directory())
                .collect::<Vec<_>>();
        }
        let theme = &self.theme;
        let View {
            ref mode,
            ref file_style,
            ..
        } = self.options.view;

        match (mode, self.console_width) {
            (Mode::Grid(opts), Some(console_width)) => {
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

            (Mode::Grid(opts), None) => {
                let filter = &self.options.filter;
                let r = grid::Render {
                    files,
                    theme,
                    file_style,
                    opts,
                    console_width: 80,
                    filter,
                };
                r.render(&mut self.writer)
            }

            (Mode::Lines, _) => {
                let filter = &self.options.filter;
                let r = lines::Render {
                    files,
                    theme,
                    file_style,
                    filter,
                };
                r.render(&mut self.writer)
            }

            (Mode::Details(opts), _) => {
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

            (Mode::GridDetails(opts), Some(console_width)) => {
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

            (Mode::GridDetails(opts), None) => {
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
