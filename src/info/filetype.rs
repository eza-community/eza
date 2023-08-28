//! Tests for various types of file (video, image, compressed, etc).
//!
//! Currently this is dependent on the file’s name and extension, because
//! those are the only metadata that we have access to without reading the
//! file’s contents.
//!
//! # Contributors
//! Please keep these lists sorted. If you're using vim, :sort i

use ansi_term::Style;
use phf;

use crate::fs::File;
use crate::theme::FileColours;

#[derive(Debug, Clone)]
enum FileType {
    Image,
    Video,
    Music,
    Lossless, // Lossless music, rather than any other kind of data...
    Crypto,
    Document,
    Compressed,
    Temp,
    Compiled,
    // An “immediate” file is something that can be run or activated somehow
    // in order to kick off the build of a project. It’s usually only present
    // in directories full of source code.
    Immediate
}

// See build.rs for EXTENSION_TYPES and FILENAME_TYPES
include!(concat!(env!("OUT_DIR"), "/filetype_maps.rs"));

#[derive(Debug, Default, PartialEq, Eq)]
pub struct FileExtensions;

impl FileExtensions {
    fn get_file_type(file: &File<'_>) -> Option<FileType> {
        // Case-insensitive readme is checked first for backwards compatibility.
        if file.name.to_lowercase().starts_with("readme") {
            return Some(FileType::Immediate)
        }
        if let Some(file_type) = FILENAME_TYPES.get(&file.name) {
            return Some(file_type.clone())
        }
        if let Some(file_type) = file.ext.as_ref().and_then(|ext| EXTENSION_TYPES.get(ext)) {
            return Some(file_type.clone())
        }
        if file.name.ends_with('~') || (file.name.starts_with('#') && file.name.ends_with('#')) {
            return Some(FileType::Temp)
        }
        if let Some(dir) = file.parent_dir {
            if file.get_source_files().iter().any(|path| dir.contains(path)) {
                return Some(FileType::Compiled)
            }
        }
        None
    }
}

impl FileColours for FileExtensions {
    fn colour_file(&self, file: &File<'_>) -> Option<Style> {
        use ansi_term::Colour::*;

        match FileExtensions::get_file_type(file) {
            Some(FileType::Compiled)   => Some(Yellow.normal()),
            Some(FileType::Compressed) => Some(Red.normal()),
            Some(FileType::Crypto)     => Some(Green.bold()),
            Some(FileType::Document)   => Some(Green.normal()),
            Some(FileType::Image)      => Some(Purple.normal()),
            Some(FileType::Immediate)  => Some(Yellow.bold().underline()),
            Some(FileType::Lossless)   => Some(Cyan.bold()),
            Some(FileType::Music)      => Some(Cyan.normal()),
            Some(FileType::Temp)       => Some(White.normal()),
            Some(FileType::Video)      => Some(Purple.bold()),
            _                          => None
        }
    }
}
