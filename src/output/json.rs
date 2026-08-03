// SPDX-FileCopyrightText: 2026 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2026 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use std::{
    io::{self, Write},
    path::{Component, PathBuf},
};

use log::debug;

#[cfg(unix)]
use crate::output::render::{GroupRender, OctalPermissionsRender, UserRender};

use crate::{
    fs::{
        self, Dir, DotFilter, File, dir_action::DirAction, feature::git::GitCache, fields as f,
        filter::FileFilter,
    },
    loc::count_roots,
    options::parser::CodeContent,
    output::{
        View,
        details::{self, show_xattr_hint},
        render::{LanguageRender, LocRender, PermissionsPlusRender, TimeRender},
        table::{Column, ENVIRONMENT, Environment, Options as TableOptions},
    },
};

#[derive(PartialEq, Eq, Debug)]
pub struct Options {
    /// Options for the --long option itself
    pub details: Option<details::Options>,
}

pub struct Render<'a> {
    git: Option<&'a GitCache>,

    deref_links: bool,
    total_size: bool,

    dots: DotFilter,
    opts: &'a Options,

    git_ignoring: bool,
    git_repos: bool,

    file_filter: &'a FileFilter,
    dir_action: &'a DirAction,
    view: &'a View,

    environment: &'a Environment,
}

impl<'a> Render<'a> {
    pub fn new(
        git: Option<&'a GitCache>,

        dots: DotFilter,
        opts: &'a Options,

        git_ignoring: bool,
        git_repos: bool,

        options: &'a crate::options::Options,
    ) -> Self {
        // Should not cause problem as usage of the global at both places should not happen, but maybe need advice on how to better handle that ?
        let environment = &*ENVIRONMENT;

        Self {
            git,
            deref_links: options.view.deref_links,
            total_size: options.view.total_size,
            dots,
            opts,
            git_ignoring,
            git_repos,
            environment,
            file_filter: &options.filter,
            dir_action: &options.dir_action,
            view: &options.view,
        }
    }

    pub fn render<W: Write>(
        &self,
        files: Vec<File<'a>>,
        mut dirs: Vec<Dir>,
        w: &mut W,
    ) -> io::Result<()> {
        match (
            files.len(),
            dirs.len(),
            self.dir_action.recurse_options().is_some(),
        ) {
            (0, 1, false) => {
                // Safe unwrap as we verify before that the len is at least one.
                let dir = dirs.get_mut(0).unwrap();
                self.render_directory(dir, w)
            }
            (_, 0, _) => self.render_files(files, w),
            (0, _, true) => self.render_recursive_directories(&mut dirs, false, w),
            (0, _, _) => self.render_directories(dirs, w),
            (_, _, recurse) => self.render_files_directories(files, dirs, recurse, w),
        }?;
        Ok(())
    }

    fn render_files<W: Write>(&self, files: Vec<File<'a>>, w: &mut W) -> io::Result<()> {
        match &self.opts.details {
            None => {
                let fnames: Vec<String> = files.iter().map(|f| self.render_file(f, None)).collect();
                write!(w, "[{}]", fnames.join(","))?;
            }
            Some(details) => {
                let code_loc = match &details.table {
                    Some(t) => {
                        if matches!(
                            t.columns.loc,
                            Some(CodeContent::Percent | CodeContent::Both)
                        ) {
                            let roots: Vec<PathBuf> =
                                files.iter().map(|f| f.path.clone()).collect();
                            let report = count_roots(&roots);

                            Some(report.total().code)
                        } else {
                            None
                        }
                    }
                    None => None,
                };

                let fnames: Vec<String> = files
                    .iter()
                    .map(|f| format!("\"{}\":{{{}}}", f.name, self.render_file(f, code_loc)))
                    .collect();
                write!(w, "{{{}}}", fnames.join(","))?;
            }
        }
        Ok(())
    }

    fn render_directory<W: Write>(&self, dir: &'a mut Dir, w: &mut W) -> io::Result<()> {
        let dir = dir.read()?;
        let files: Vec<File<'a>> = dir
            .files(
                self.dots,
                self.git,
                self.git_ignoring,
                self.deref_links,
                self.total_size,
            )
            .collect();

        self.render_files(files, w)?;

        Ok(())
    }

    fn render_recursive_directories<W: Write>(
        &self,
        dirs: &'a mut Vec<Dir>,
        sub_dir: bool,
        w: &mut W,
    ) -> io::Result<()> {
        write!(w, "{{")?;
        let mut first = true;
        for dir in dirs {
            if first {
                first = false;
            } else {
                write!(w, ",")?;
            }
            if sub_dir {
                write!(w, "\"{}\":{{", dir.path.display())?;
            } else {
                // We can safely unwrap as .. . and the / case ar not possible here. They cannot be subdirs.
                write!(
                    w,
                    "\"{}\":{{",
                    dir.path.file_name().unwrap().to_string_lossy()
                )?;
            }
            let dir_r = dir.read()?;
            let mut files: Vec<File<'a>> = dir_r
                .files(
                    self.dots,
                    self.git,
                    self.git_ignoring,
                    self.deref_links,
                    self.total_size,
                )
                .collect();

            self.file_filter.filter_child_files(true, &mut files);
            self.file_filter.sort_files(&mut files);
            let recurse_opts = self.dir_action.recurse_options().unwrap();
            let depth: usize = dir_r
                .path
                .components()
                .filter(|&c| c != Component::CurDir)
                .count()
                + 1;

            let follow_links = self.view.follow_links;
            if !recurse_opts.tree && !recurse_opts.is_too_deep(depth) {
                let mut child_dirs = files
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

                write!(w, "\"files\":")?;
                self.render_files(files, w)?;
                write!(w, ", \"directories\":")?;
                self.render_recursive_directories(&mut child_dirs, false, w)?;
            };
            write!(w, "}}")?;
        }
        write!(w, "}}")?;
        Ok(())
    }

    fn render_directories<W: Write>(&self, dirs: Vec<Dir>, w: &mut W) -> io::Result<()> {
        write!(w, "{{")?;
        let mut first = true;
        for mut dir in dirs {
            if first {
                first = false;
            } else {
                write!(w, ",")?;
            }
            write!(w, "\"{}\":", dir.path.display())?;
            self.render_directory(&mut dir, w)?;
        }
        write!(w, "}}")?;
        Ok(())
    }

    fn render_files_directories<W: Write>(
        &self,
        files: Vec<File<'a>>,
        mut dirs: Vec<Dir>,
        recurse: bool,
        w: &mut W,
    ) -> io::Result<()> {
        write!(w, "{{\"files\":")?;
        self.render_files(files, w)?;
        write!(w, ", \"directories\":")?;
        if recurse {
            self.render_recursive_directories(&mut dirs, false, w)?;
        } else {
            self.render_directories(dirs, w)?;
        }
        write!(w, "}}")?;
        Ok(())
    }

    fn render_file(&self, f: &File<'a>, code_loc: Option<usize>) -> String {
        match &self.opts.details {
            None => format!("\"{}\"", f.name),
            Some(o) => self.render_file_long(f, o, code_loc),
        }
    }

    fn render_file_long(
        &self,
        f: &File<'a>,
        o: &details::Options,
        code_loc: Option<usize>,
    ) -> String {
        if let Some(table_opts) = &o.table {
            let columns = table_opts
                .columns
                .collect(self.git.is_some(), self.git_repos);

            let fobj = JsonFileObject::create_for_for_file(
                f,
                table_opts,
                columns,
                self.environment,
                show_xattr_hint(self.opts.details.as_ref().is_some_and(|d| d.secattr), f),
                self.git,
                code_loc,
            );
            fobj.render()
        } else {
            String::new()
        }
    }
}

struct JsonFileObject<'a> {
    /// Reusing the table column to map everything we want to be displayed
    internal: Vec<(Column, String)>,

    options: &'a TableOptions,

    pub git: Option<&'a GitCache>,

    code_loc: Option<usize>,
}

impl<'a> JsonFileObject<'a> {
    /// Render a json object with the columns in the map
    fn render(self) -> String {
        self.internal
            .iter()
            .map(|(c, v)| format!("\"{}\": {}", c.header(), v))
            .collect::<Vec<String>>()
            .join(",")
    }

    fn create_for_for_file(
        f: &File<'a>,
        options: &'a TableOptions,
        columns: Vec<Column>,
        env: &Environment,
        xattrs: bool,
        git: Option<&'a GitCache>,
        code_loc: Option<usize>,
    ) -> Self {
        let mut res = Self {
            internal: vec![],
            options,
            git,
            code_loc,
        };

        columns
            .iter()
            .for_each(|c| res.add_column(f, c, env, xattrs));

        res
    }

    fn add_column(&mut self, f: &File, c: &Column, env: &Environment, xattrs: bool) {
        let column_opt = self.get_column(f, c, env, xattrs);

        if let Some(column) = column_opt {
            self.internal.push((*c, format!("\"{column}\"")))
        }
    }

    fn get_column(&self, f: &File, c: &Column, env: &Environment, xattrs: bool) -> Option<String> {
        match c {
            Column::Permissions => f.permissions_plus(xattrs).render_json(),
            Column::Timestamp(time_type) => time_type
                .get_corresponding_time(f)
                .render_json(env.time_offset, self.options.time_format.clone()),
            Column::FileSize => f.size().render_json(self.options.size_format, &env.numeric),
            #[cfg(unix)]
            Column::User => f
                .user()
                .render_json(&*env.lock_users(), self.options.user_format),
            Column::GitStatus => Some(self.git_status(f).render_json()),
            #[cfg(unix)]
            Column::Blocksize => f
                .blocksize()
                .render_json(self.options.size_format, &env.numeric),
            Column::FileFlags => f.flags().render_json(self.options.flags_format),
            #[cfg(unix)]
            Column::Group => f.group().render_json(
                &*env.lock_users(),
                self.options.user_format,
                self.options.group_format,
                f.user(),
            ),
            #[cfg(unix)]
            Column::Inode => Some(f.inode().render_json()),
            #[cfg(unix)]
            Column::HardLinks => Some(f.links().render_json(&env.numeric)),
            #[cfg(unix)]
            Column::Octal => f
                .permissions()
                .map(|p| f::OctalPermissions { permissions: p })
                .render_json(),
            #[cfg(unix)]
            Column::SecurityContext => f.security_context().render_json(),

            Column::Language => f.language().render_json(),
            Column::Loc(code_content) => {
                f.loc()
                    .render_json(*code_content, self.code_loc, &env.numeric)
            }
            Column::SubdirGitRepo(status) => self.subdir_git_repo(f, *status).render_json(),
        }
    }

    fn subdir_git_repo(&self, file: &File<'_>, status: bool) -> f::SubdirGitRepo {
        debug!("Getting subdir repo status for path {:?}", file.path);

        if file.is_directory() {
            return f::SubdirGitRepo::from_path(&file.path, status);
        }
        f::SubdirGitRepo::default()
    }

    fn git_status(&self, file: &File<'_>) -> f::Git {
        self.git
            .map(|g| g.get(&file.path, file.is_directory()))
            .unwrap_or_default()
    }
}
