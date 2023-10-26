use std::io::{self, Write};

use term_grid as tg;

use crate::fs::filter::FileFilter;
use crate::fs::File;
use crate::output::file_name::{Classify, Options as FileStyle};
use crate::output::file_name::{EmbedHyperlinks, ShowIcons};
use crate::theme::Theme;

use super::file_name::QuoteStyle;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Options {
    pub across: bool,
}

impl Options {
    pub fn direction(self) -> tg::Direction {
        if self.across {
            tg::Direction::LeftToRight
        } else {
            tg::Direction::TopToBottom
        }
    }
}

pub struct Render<'a> {
    pub files: Vec<File<'a>>,
    pub theme: &'a Theme,
    pub file_style: &'a FileStyle,
    pub opts: &'a Options,
    pub console_width: usize,
    pub filter: &'a FileFilter,
}

impl<'a> Render<'a> {
    pub fn render<W: Write>(mut self, w: &mut W) -> io::Result<()> {
        let mut grid = tg::Grid::new(tg::GridOptions {
            direction: self.opts.direction(),
            filling: tg::Filling::Spaces(2),
        });

        grid.reserve(self.files.len());

        self.filter.sort_files(&mut self.files);
        for file in &self.files {
            let filename = self.file_style.for_file(file, self.theme);

            // Calculate classification width
            let classification_width =
                if let Classify::AddFileIndicators = filename.options.classify {
                    match filename.classify_char(file) {
                        Some(s) => s.len(),
                        None => 0,
                    }
                } else {
                    0
                };
            let space_filename_offset = match self.file_style.quote_style {
                QuoteStyle::QuoteSpaces if file.name.contains(' ') => 2,
                QuoteStyle::NoQuotes => 0,
                QuoteStyle::QuoteSpaces => 0, // Default case
            };
            let contents = filename.paint();
            let width = match (
                filename.options.embed_hyperlinks,
                filename.options.show_icons,
            ) {
                (
                    EmbedHyperlinks::On,
                    ShowIcons::Always(spacing) | ShowIcons::Automatic(spacing),
                ) => {
                    filename.bare_width()
                        + classification_width
                        + 1
                        + (spacing as usize)
                        + space_filename_offset
                }
                (EmbedHyperlinks::On, ShowIcons::Never) => {
                    filename.bare_width() + classification_width + space_filename_offset
                }
                (
                    EmbedHyperlinks::Off,
                    ShowIcons::Always(spacing) | ShowIcons::Automatic(spacing),
                ) => filename.bare_width() + 1 + (spacing as usize) + space_filename_offset,
                (EmbedHyperlinks::Off, _) => *contents.width(),
            };

            grid.add(tg::Cell {
                contents: contents.strings().to_string(),
                // with hyperlink escape sequences,
                // the actual *contents.width() is larger than actually needed, so we take only the filename
                width,
            });
        }

        if let Some(display) = grid.fit_into_width(self.console_width) {
            write!(w, "{display}")
        } else {
            // File names too long for a grid - drop down to just listing them!
            // This isn’t *quite* the same as the lines view, which also
            // displays full link paths.
            for file in &self.files {
                let name_cell = self.file_style.for_file(file, self.theme).paint();
                writeln!(w, "{}", name_cell.strings())?;
            }

            Ok(())
        }
    }
}
