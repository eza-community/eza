/*
This file is a special renderer for json
*/

use std::{
    fmt::Display,
    io::{self, Write},
};

use super::{details::TableIter, TextCell};

#[derive(Debug, Clone)]
struct JsonFile {
    cell: TextCell,
    childrens: Option<Vec<JsonFile>>,
}

#[derive(Debug, Clone)]
pub struct JsonRenderer {
    header: Option<TextCell>,
    files: Vec<JsonFile>,
}

impl JsonFile {
    pub fn new(cell: TextCell) -> Self {
        Self {
            cell,
            childrens: None,
        }
    }

    pub fn display<W: Write>(&self, w: &mut W, header: &Option<TextCell>) -> io::Result<()> {
        let mut header_idx = 0;

        if header.is_some() {
            writeln!(w, "{{")?;
        } else {
            writeln!(w, "[")?;
        }
        for (i, cell) in self.cell.contents.iter().enumerate() {
            if cell.is_empty() || cell.trim().is_empty() {
                continue;
            };
            if let Some(ref header) = header {
                write!(w, "\"{}\": ", header.contents[header_idx])?;
                header_idx += 1;
            }
            write!(w, "\"{cell}\"")?;
            if (i + 1) < self.cell.contents.len() {
                writeln!(w, ", ")?;
            }
        }
        if self.childrens.is_some() {
            if header.is_some() {
                writeln!(w, ", \"childends\": [")
            } else {
                writeln!(w, ", [")
            }?;
            for (i, f) in self.childrens.as_ref().unwrap().iter().enumerate() {
                if i != 0 {
                    writeln!(w, ",")?;
                }
                f.display(w, header)?;
            }
            writeln!(w, "]")?;
        }
        if header.is_some() {
            writeln!(w, "}}")
        } else {
            writeln!(w, "]")
        }
    }
}

impl JsonRenderer {
    pub fn new(header: Option<TextCell>, files: TableIter<'_>) -> Self {
        Self {
            header,
            files: files.map(|row| JsonFile::new(row)).collect(),
        }
    }

    pub fn render<W: Write>(&self, w: &mut W) -> io::Result<()> {
        writeln!(w, "{{\n\"files\":[")?;
        for (i, f) in self.files.iter().enumerate() {
            if i != 0 {
                writeln!(w, ",")?;
            }
            f.display(w, &self.header)?;
        }
        writeln!(w, "]\n}}")
    }
}
