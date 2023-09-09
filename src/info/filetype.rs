//! Tests for various types of file (video, image, compressed, etc).
//!
//! Currently this is dependent on the file’s name and extension, because
//! those are the only metadata that we have access to without reading the
//! file’s contents.
//!
//! # Contributors
//! Please keep these lists sorted. If you're using vim, :sort i

use ansiterm::Style;
use phf::{phf_map, Map};

use crate::fs::File;
use crate::theme::FileColours;

#[derive(Debug, Clone)]
pub enum FileType {
    Image,
    Video,
    Music,
    Lossless, // Lossless music, rather than any other kind of data...
    Crypto,
    Document,
    Compressed,
    Temp,
    Compiled,
    Immediate // An “immediate” file is something that can be run or activated somehow in order to
              // kick off the build of a project. It’s usually only present in directories full of
              // source code.
}

/// Mapping from full filenames to file type.
const FILENAME_TYPES: Map<&'static str, FileType> = phf_map! {
    /* Immediate file - kick off the build of a project */
    "Brewfile"           => FileType::Immediate,
    "bsconfig.json"      => FileType::Immediate,
    "BUILD"              => FileType::Immediate,
    "BUILD.bazel"        => FileType::Immediate,
    "build.gradle"       => FileType::Immediate,
    "build.sbt"          => FileType::Immediate,
    "build.xml"          => FileType::Immediate,
    "Cargo.toml"         => FileType::Immediate,
    "CMakeLists.txt"     => FileType::Immediate,
    "composer.json"      => FileType::Immediate,
    "configure"          => FileType::Immediate,
    "Containerfile"      => FileType::Immediate,
    "Dockerfile"         => FileType::Immediate,
    "Earthfile"          => FileType::Immediate,
    "flake.nix"          => FileType::Immediate,
    "Gemfile"            => FileType::Immediate,
    "GNUmakefile"        => FileType::Immediate,
    "Gruntfile.coffee"   => FileType::Immediate,
    "Gruntfile.js"       => FileType::Immediate,
    "jsconfig.json"      => FileType::Immediate,
    "Justfile"           => FileType::Immediate,
    "justfile"           => FileType::Immediate,
    "Makefile"           => FileType::Immediate,
    "makefile"           => FileType::Immediate,
    "meson.build"        => FileType::Immediate,
    "mix.exs"            => FileType::Immediate,
    "package.json"       => FileType::Immediate,
    "Pipfile"            => FileType::Immediate,
    "PKGBUILD"           => FileType::Immediate,
    "Podfile"            => FileType::Immediate,
    "pom.xml"            => FileType::Immediate,
    "Procfile"           => FileType::Immediate,
    "pyproject.toml"     => FileType::Immediate,
    "Rakefile"           => FileType::Immediate,
    "RoboFile.php"       => FileType::Immediate,
    "SConstruct"         => FileType::Immediate,
    "tsconfig.json"      => FileType::Immediate,
    "Vagrantfile"        => FileType::Immediate,
    "webpack.config.cjs" => FileType::Immediate,
    "webpack.config.js"  => FileType::Immediate,
    "WORKSPACE"          => FileType::Immediate,
    /* Cryptology files */
    "id_dsa"             => FileType::Crypto,
    "id_ecdsa"           => FileType::Crypto,
    "id_ecdsa_sk"        => FileType::Crypto,
    "id_ed25519"         => FileType::Crypto,
    "id_ed25519_sk"      => FileType::Crypto,
    "id_rsa"             => FileType::Crypto,
};

/// Mapping from lowercase file extension to file type.  If an image, video, music, or lossless
/// extension is added also update the extension icon map.
const EXTENSION_TYPES: Map<&'static str, FileType> = phf_map! {
    /* Immediate file - kick off the build of a project */
    "ninja"      => FileType::Immediate,
    /* Image files */
    "arw"        => FileType::Image,
    "avif"       => FileType::Image,
    "bmp"        => FileType::Image,
    "cbr"        => FileType::Image,
    "cbz"        => FileType::Image,
    "cr2"        => FileType::Image,
    "dvi"        => FileType::Image,
    "eps"        => FileType::Image,
    "gif"        => FileType::Image,
    "heic"       => FileType::Image,
    "heif"       => FileType::Image,
    "ico"        => FileType::Image,
    "j2c"        => FileType::Image,
    "j2k"        => FileType::Image,
    "jfi"        => FileType::Image,
    "jfif"       => FileType::Image,
    "jif"        => FileType::Image,
    "jp2"        => FileType::Image,
    "jpe"        => FileType::Image,
    "jpeg"       => FileType::Image,
    "jpf"        => FileType::Image,
    "jpg"        => FileType::Image,
    "jpx"        => FileType::Image,
    "jxl"        => FileType::Image,
    "nef"        => FileType::Image,
    "orf"        => FileType::Image,
    "pbm"        => FileType::Image,
    "pgm"        => FileType::Image,
    "png"        => FileType::Image,
    "pnm"        => FileType::Image,
    "ppm"        => FileType::Image,
    "ps"         => FileType::Image,
    "psd"        => FileType::Image,
    "pxm"        => FileType::Image,
    "raw"        => FileType::Image,
    "stl"        => FileType::Image,
    "svg"        => FileType::Image,
    "tif"        => FileType::Image,
    "tiff"       => FileType::Image,
    "webp"       => FileType::Image,
    "xcf"        => FileType::Image,
    "xpm"        => FileType::Image,
    /* Video files */
    "avi"        => FileType::Video,
    "flv"        => FileType::Video,
    "heics"      => FileType::Video,
    "m2ts"       => FileType::Video,
    "m2v"        => FileType::Video,
    "m4v"        => FileType::Video,
    "mkv"        => FileType::Video,
    "mov"        => FileType::Video,
    "mp4"        => FileType::Video,
    "mpeg"       => FileType::Video,
    "mpg"        => FileType::Video,
    "ogm"        => FileType::Video,
    "ogv"        => FileType::Video,
    "video"      => FileType::Video,
    "vob"        => FileType::Video,
    "webm"       => FileType::Video,
    "wmv"        => FileType::Video,
    /* Music files */
    "aac"        => FileType::Music,
    "m4a"        => FileType::Music,
    "mka"        => FileType::Music,
    "mp2"        => FileType::Music,
    "mp3"        => FileType::Music,
    "ogg"        => FileType::Music,
    "opus"       => FileType::Music,
    "wma"        => FileType::Music,
    /* Lossless music, rather than any other kind of data... */
    "alac"       => FileType::Lossless,
    "ape"        => FileType::Lossless,
    "flac"       => FileType::Lossless,
    "wav"        => FileType::Lossless,
    /* Cryptology files */
    "asc"        => FileType::Crypto, // GnuPG ASCII armored file
    "gpg"        => FileType::Crypto, // GnuPG encrypted file
    "kbx"        => FileType::Crypto, // GnuPG keybox
    "md5"        => FileType::Crypto, // MD5 checksum
    "p12"        => FileType::Crypto, // PKCS#12 certificate (Netscape)
    "pem"        => FileType::Crypto, // Privacy enhanced mail certificate
    "pfx"        => FileType::Crypto, // PKCS#12 certificate (Microsoft)
    "pgp"        => FileType::Crypto, // PGP security key
    "pub"        => FileType::Crypto, // Public key
    "sha1"       => FileType::Crypto, // SHA-1 hash
    "sha224"     => FileType::Crypto, // SHA-224 hash
    "sha256"     => FileType::Crypto, // SHA-256 hash
    "sha384"     => FileType::Crypto, // SHA-384 hash
    "sha512"     => FileType::Crypto, // SHA-512 hash
    "sig"        => FileType::Crypto, // GnuPG signed file
    "signature"  => FileType::Crypto, // e-Filing Digital Signature File (India)
    /* Document files */
    "djvu"       => FileType::Document,
    "doc"        => FileType::Document,
    "docx"       => FileType::Document,
    "eml"        => FileType::Document,
    "fotd"       => FileType::Document,
    "gdoc"       => FileType::Document,
    "key"        => FileType::Document,
    "keynote"    => FileType::Document,
    "numbers"    => FileType::Document,
    "odp"        => FileType::Document,
    "ods"        => FileType::Document,
    "odt"        => FileType::Document,
    "pages"      => FileType::Document,
    "pdf"        => FileType::Document,
    "ppt"        => FileType::Document,
    "pptx"       => FileType::Document,
    "rtf"        => FileType::Document,
    "xls"        => FileType::Document,
    "xlsm"       => FileType::Document,
    "xlsx"       => FileType::Document,
    /* Compressed/archive files */
    "7z"         => FileType::Compressed,
    "ar"         => FileType::Compressed,
    "br"         => FileType::Compressed,
    "bz"         => FileType::Compressed,
    "bz2"        => FileType::Compressed,
    "bz3"        => FileType::Compressed,
    "cpio"       => FileType::Compressed,
    "deb"        => FileType::Compressed,
    "dmg"        => FileType::Compressed,
    "gz"         => FileType::Compressed,
    "iso"        => FileType::Compressed,
    "lz"         => FileType::Compressed,
    "lz4"        => FileType::Compressed,
    "lzh"        => FileType::Compressed,
    "lzma"       => FileType::Compressed,
    "lzo"        => FileType::Compressed,
    "phar"       => FileType::Compressed,
    "qcow"       => FileType::Compressed,
    "qcow2"      => FileType::Compressed,
    "rar"        => FileType::Compressed,
    "rpm"        => FileType::Compressed,
    "tar"        => FileType::Compressed,
    "taz"        => FileType::Compressed,
    "tbz"        => FileType::Compressed,
    "tbz2"       => FileType::Compressed,
    "tc"         => FileType::Compressed,
    "tgz"        => FileType::Compressed,
    "tlz"        => FileType::Compressed,
    "txz"        => FileType::Compressed,
    "tz"         => FileType::Compressed,
    "xz"         => FileType::Compressed,
    "vdi"        => FileType::Compressed,
    "vhd"        => FileType::Compressed,
    "vmdk"       => FileType::Compressed,
    "z"          => FileType::Compressed,
    "zip"        => FileType::Compressed,
    "zst"        => FileType::Compressed,
    /* Temporary files */
    "bak"        => FileType::Temp,
    "bk"         => FileType::Temp,
    "bkp"        => FileType::Temp,
    "download"   => FileType::Temp,
    "swn"        => FileType::Temp,
    "swo"        => FileType::Temp,
    "swp"        => FileType::Temp,
    "tmp"        => FileType::Temp,
    /* Compiler output files */
    "a"          => FileType::Compiled, // Unix static library
    "bundle"     => FileType::Compiled, // Mac OS X application bundle
    "class"      => FileType::Compiled, // Java class file
    "dll"        => FileType::Compiled, // Windows dynamic link library
    "dylib"      => FileType::Compiled, // Mach-O dynamic library
    "elc"        => FileType::Compiled, // Emacs compiled lisp
    "ko"         => FileType::Compiled, // Linux kernel module
    "lib"        => FileType::Compiled, // Windows static library
    "o"          => FileType::Compiled, // Compiled object file
    "obj"        => FileType::Compiled, // Compiled object file
    "pyc"        => FileType::Compiled, // Python compiled code
    "pyd"        => FileType::Compiled, // Python dynamic module
    "pyo"        => FileType::Compiled, // Python optimized code
    "so"         => FileType::Compiled, // Unix shared library
    "zwc"        => FileType::Compiled, // zsh compiled file
};

impl FileType {
    /// Lookup the file type based on the file's name, by the file name
    /// lowercase extension, or if the file could be compiled from related
    /// source code.
    fn get_file_type(file: &File<'_>) -> Option<FileType> {
        // Case-insensitive readme is checked first for backwards compatibility.
        if file.name.to_lowercase().starts_with("readme") {
            return Some(Self::Immediate)
        }
        if let Some(file_type) = FILENAME_TYPES.get(&file.name) {
            return Some(file_type.clone())
        }
        if let Some(file_type) = file.ext.as_ref().and_then(|ext| EXTENSION_TYPES.get(ext)) {
            return Some(file_type.clone())
        }
        if file.name.ends_with('~') || (file.name.starts_with('#') && file.name.ends_with('#')) {
            return Some(Self::Temp)
        }
        if let Some(dir) = file.parent_dir {
            if file.get_source_files().iter().any(|path| dir.contains(path)) {
                return Some(Self::Compiled)
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct FileTypeColor;

impl FileColours for FileTypeColor {
    /// Map from the file type to the display style/color for the file.
    fn colour_file(&self, file: &File<'_>) -> Option<Style> {
        use ansiterm::Colour::*;

        match FileType::get_file_type(file) {
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
