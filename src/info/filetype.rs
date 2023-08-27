//! Tests for various types of file (video, image, compressed, etc).
//!
//! Currently this is dependent on the file’s name and extension, because
//! those are the only metadata that we have access to without reading the
//! file’s contents.
//!
//! # Contributors
//! Please keep these lists sorted. If you're using vim, :sort i

use nu_ansi_term::Style;

use crate::fs::File;
use crate::output::icons::FileIcon;
use crate::theme::FileColors;


#[derive(Debug, Default, PartialEq, Eq)]
pub struct FileExtensions;

impl FileExtensions {

    /// An “immediate” file is something that can be run or activated somehow
    /// in order to kick off the build of a project. It’s usually only present
    /// in directories full of source code.
    #[allow(clippy::case_sensitive_file_extension_comparisons)]
    fn is_immediate(&self, file: &File<'_>) -> bool {
	file.name.to_lowercase().starts_with("readme")
            || file.name.ends_with(".ninja")
            || matches!(
                file.name.as_str(),
                "BUILD"
                    | "Brewfile"
                    | "bsconfig.json"
                    | "BUILD.bazel"
                    | "build.gradle"
                    | "build.sbt"
                    | "build.xml"
                    | "Cargo.lock"
                    | "Cargo.toml"
                    | "CMakeLists.txt"
                    | "composer.json"
                    | "configure.ac"
                    | "Configure.ac"
                    | "Containerfile"
                    | "Dockerfile"
                    | "Earthfile"
                    | "flake.lock"
                    | "flake.nix"
                    | "Gemfile"
                    | "GNUmakefile"
                    | "Gruntfile.coffee"
                    | "Gruntfile.js"
                    | "Justfile"
                    | "justfile"
                    | "Makefile"
                    | "makefile"
                    | "Makefile.in"
                    | "makefile.in"
                    | "meson.build"
                    | "mix.exs"
                    | "package.json"
                    | "Pipfile"
                    | "PKGBUILD"
                    | "Podfile"
                    | "pom.xml"
                    | "Procfile"
                    | "Rakefile"
                    | "RoboFile.php"
                    | "SConstruct"
                    | "tsconfig.json"
                    | "Vagrantfile"
                    | "webpack.config.cjs"
                    | "webpack.config.js"
                    | "WORKSPACE"
            )
    }

    fn is_image(&self, file: &File<'_>) -> bool {
        file.extension_is_one_of( &[
            "arw",
            "avif",
            "bmp",
            "cbr",
            "cbz",
            "cr2",
            "dvi",
            "eps",
            "gif",
            "heif",
            "ico",
            "j2c",
            "j2k",
            "jfi",
            "jfif",
            "jif",
            "jp2",
            "jpe",
            "jpeg",
            "jpf",
            "jpg",
            "jpx",
            "jxl",
            "nef",
            "orf",
            "pbm",
            "pgm",
            "png",
            "pnm",
            "ppm",
            "ps",
            "pxm",
            "raw",
            "stl",
            "svg",
            "tif",
            "tiff",
            "webp",
            "xpm",
        ])
    }

    fn is_video(&self, file: &File<'_>) -> bool {
        file.extension_is_one_of( &[
            "avi",
            "flv",
            "heic",
            "m2ts",
            "m2v",
            "m4v",
            "mkv",
            "mov",
            "mp4",
            "mpeg",
            "mpg",
            "ogm",
            "ogv",
            "vob",
            "webm",
            "wmv",
        ])
    }

    fn is_music(&self, file: &File<'_>) -> bool {
        file.extension_is_one_of( &[
            "aac",
            "m4a",
            "mka",
            "mp2",
            "mp3",
            "ogg",
            "opus",
            "wma",
        ])
    }

    // Lossless music, rather than any other kind of data...
    fn is_lossless(&self, file: &File<'_>) -> bool {
        file.extension_is_one_of( &[
            "alac",
            "ape",
            "flac",
            "wav",
        ])
    }

    fn is_crypto(&self, file: &File<'_>) -> bool {
        file.extension_is_one_of( &[
            "asc",
            "enc",
            "gpg",
            "p12",
            "pfx",
            "pgp",
            "sig",
            "signature",
        ])
    }

    fn is_document(&self, file: &File<'_>) -> bool {
        file.extension_is_one_of( &[
            "djvu",
            "doc",
            "docx",
            "dvi",
            "eml",
            "eps",
            "fotd",
            "key",
            "keynote",
            "numbers",
            "odp",
            "odt",
            "pages",
            "pdf",
            "ppt",
            "pptx",
            "rtf",
            "xls",
            "xlsx",
        ])
    }

    fn is_compressed(&self, file: &File<'_>) -> bool {
        file.extension_is_one_of( &[
            "7z",
            "a",
            "ar",
            "bz",
            "bz2",
            "bz3",
            "cpio",
            "deb",
            "dmg",
            "gz",
            "iso",
            "lz",
            "lz4",
            "lzh",
            "lzma",
            "lzo",
            "par",
            "rar",
            "rpm",
            "tar",
            "taz",
            "tbz",
            "tbz2",
            "tc",
            "tgz",
            "tlz",
            "txz",
            "tz",
            "tzo",
            "xz",
            "Z",
            "z",
            "zip",
            "zst",
        ])
    }

    fn is_temp(&self, file: &File<'_>) -> bool {
        file.name.ends_with('~')
            || (file.name.starts_with('#') && file.name.ends_with('#'))
            || file.extension_is_one_of( &[
                "bak",
                "bk",
                "bkp",
                "swn",
                "swo",
                "swp",
                "tmp",
            ])
    }

    fn is_compiled(&self, file: &File<'_>) -> bool {
        if file.extension_is_one_of( &[
            "class",
            "elc",
            "hi",
            "ko",
            "o",
            "pyc",
            "zwc",
        ]) {
            true
        }
        else if let Some(dir) = file.parent_dir {
            file.get_source_files().iter().any(|path| dir.contains(path))
        }
        else {
            false
        }
    }
}

impl FileColors for FileExtensions {
    fn color_file(&self, file: &File<'_>) -> Option<Style> {
        use nu_ansi_term::Color::*;

        Some(match file {
            f if self.is_compiled(f)    => Yellow.normal(),
            f if self.is_compressed(f)  => Red.normal(),
            f if self.is_crypto(f)      => Green.bold(),
            f if self.is_document(f)    => Green.normal(),
            f if self.is_image(f)       => Purple.normal(),
            f if self.is_immediate(f)   => Yellow.bold().underline(),
            f if self.is_lossless(f)    => Cyan.bold(),
            f if self.is_music(f)       => Cyan.normal(),
            f if self.is_temp(f)        => White.normal(),
            f if self.is_video(f)       => Purple.bold(),
            _                           => return None,
        })
    }
}

impl FileIcon for FileExtensions {
    fn icon_file(&self, file: &File<'_>) -> Option<char> {
        use crate::output::icons::Icons;

        if self.is_music(file) || self.is_lossless(file) {
            Some(Icons::Audio.value())
        }
        else if self.is_image(file) {
            Some(Icons::Image.value())
        }
        else if self.is_video(file) {
            Some(Icons::Video.value())
        }
        else if self.is_compressed(file) {
            Some(Icons::Compressed.value())
        }
        else {
            None
        }
    }
}
