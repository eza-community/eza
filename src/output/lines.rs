use std::io::{self, Write};

use nu_ansi_term::AnsiStrings as ANSIStrings;

use crate::fs::filter::FileFilter;
use crate::fs::File;
use crate::output::cell::TextCellContents;
use crate::output::file_name::Options as FileStyle;
use crate::theme::Theme;

/// The lines view literally just displays each file, line-by-line.
pub struct Render<'a> {
    pub files: Vec<File<'a>>,
    pub theme: &'a Theme,
    pub file_style: &'a FileStyle,
    pub filter: &'a FileFilter,
}

impl<'a> Render<'a> {
    pub fn render<W: Write>(mut self, w: &mut W) -> io::Result<()> {
        self.filter.sort_files(&mut self.files);
        for file in &self.files {
            let name_cell = self.render_file(file);
            writeln!(w, "{}", ANSIStrings(&name_cell))?;
        }

        Ok(())
    }

    fn render_file<'f>(&self, file: &'f File<'a>) -> TextCellContents {
        self.file_style
            .for_file(file, self.theme)
            .with_link_paths()
            .with_mount_details(false)
            .paint()
    }

    pub fn render_as_json<W: Write>(mut self, w: &mut W) -> io::Result<()> {
        self.filter.sort_files(&mut self.files);
        write!(w, "{}", "{\"files\":[")?;
        for (i, file) in self.files.iter().enumerate() {
            let name_cell = self.render_file(file);
            write!(w, "\"{}\"", ANSIStrings(&name_cell))?;
            if (i + 1) < self.files.len() {
                write!(w, "{}", ",")?;
            }
        }
        write!(w, "{}", "]}\n")?;

        Ok(())
    }
}
