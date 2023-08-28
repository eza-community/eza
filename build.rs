/// The version string isn’t the simplest: we want to show the version,
/// current Git hash, and compilation date when building *debug* versions, but
/// just the version for *release* versions so the builds are reproducible.
///
/// This script generates the string from the environment variables that Cargo
/// adds (http://doc.crates.io/environment-variables.html) and runs `git` to
/// get the SHA1 hash. It then writes the string into a file, which exa then
/// includes at build-time.
///
/// - https://stackoverflow.com/q/43753491/3484614
/// - https://crates.io/crates/vergen

use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

use datetime::{LocalDateTime, ISO};


/// The build script entry point.
fn main() -> io::Result<()> {
    create_version_string_file()?;
    create_file_typing_hash_file()?;
    create_file_icon_hash_file()
}

/// Create the version_string.txt file
fn create_version_string_file() -> io::Result<()> {
    #![allow(clippy::write_with_newline)]

    let tagline = "eza - A modern, maintained replacement for ls";
    let url     = "https://github.com/eza-community/eza";

    let ver =
        if is_debug_build() {
            format!("{}\nv{} \\1;31m(pre-release debug build!)\\0m\n\\1;4;34m{}\\0m", tagline, version_string(), url)
        }
        else if is_development_version() {
            format!("{}\nv{} [{}] built on {} \\1;31m(pre-release!)\\0m\n\\1;4;34m{}\\0m", tagline, version_string(), git_hash(), build_date(), url)
        }
        else {
            format!("{}\nv{}\n\\1;4;34m{}\\0m", tagline, version_string(), url)
        };

    // We need to create these files in the Cargo output directory.
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let path = &out.join("version_string.txt");

    // Bland version text
    let mut f = File::create(path).unwrap_or_else(|_| { panic!("{}", path.to_string_lossy().to_string()) });
    writeln!(f, "{}", strip_codes(&ver))
}

/// Removes escape codes from a string.
fn strip_codes(input: &str) -> String {
    input.replace("\\0m", "")
         .replace("\\1;31m", "")
         .replace("\\1;4;34m", "")
}

/// Retrieve the project’s current Git hash, as a string.
fn git_hash() -> String {
    use std::process::Command;

    String::from_utf8_lossy(
        &Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output().unwrap()
            .stdout).trim().to_string()
}

/// Whether we should show pre-release info in the version string.
///
/// Both weekly releases and actual releases are --release releases,
/// but actual releases will have a proper version number.
fn is_development_version() -> bool {
    cargo_version().ends_with("-pre") || env::var("PROFILE").unwrap() == "debug"
}

/// Whether we are building in debug mode.
fn is_debug_build() -> bool {
    env::var("PROFILE").unwrap() == "debug"
}

/// Retrieves the [package] version in Cargo.toml as a string.
fn cargo_version() -> String {
    env::var("CARGO_PKG_VERSION").unwrap()
}

/// Returns the version and build parameters string.
fn version_string() -> String {
    let mut ver = cargo_version();

    let feats = nonstandard_features_string();
    if ! feats.is_empty() {
        ver.push_str(&format!(" [{}]", &feats));
    }

    ver
}

/// Finds whether a feature is enabled by examining the Cargo variable.
fn feature_enabled(name: &str) -> bool {
    env::var(format!("CARGO_FEATURE_{}", name))
        .map(|e| ! e.is_empty())
        .unwrap_or(false)
}

/// A comma-separated list of non-standard feature choices.
fn nonstandard_features_string() -> String {
    let mut s = Vec::new();

    if feature_enabled("GIT") {
        s.push("+git");
    }
    else {
        s.push("-git");
    }

    s.join(", ")
}

/// Formats the current date as an ISO 8601 string.
fn build_date() -> String {
    let now = LocalDateTime::now();
    format!("{}", now.date().iso())
}

/// Create the perfect hashing for file typing
fn create_file_typing_hash_file() -> io::Result<()> {
    let path = &PathBuf::from(env::var("OUT_DIR").unwrap()).join("filetype_maps.rs");
    let mut file = io::BufWriter::new(File::create(path).unwrap_or_else(|_| {
        panic!("{}", path.to_string_lossy().to_string())
    }));
    generate_filename_type_map(file.get_mut())?;
    generate_extension_type_map(file.get_mut())?;
    file.flush()
}

/// Create the perfect hashing for file icons
fn create_file_icon_hash_file() -> io::Result<()> {
    let path = &PathBuf::from(env::var("OUT_DIR").unwrap()).join("icon_maps.rs");
    let mut file = io::BufWriter::new(File::create(path).unwrap_or_else(|_| {
        panic!("{}", path.to_string_lossy().to_string());
    }));
    generate_filename_icon_map(file.get_mut())?;
    generate_extension_icon_map(file.get_mut())?;
    file.flush()
}

/// Generate mapping from full filenames to file type. For file types see info/filetype.rs
fn generate_filename_type_map(file: &mut File) -> io::Result<()> {
    writeln!(file, "static FILENAME_TYPES: phf::Map<&'static str, FileType> = {};\n",
             phf_codegen::Map::new()
                 /* Immediate file - kick off the build of a project */
                 .entry("Brewfile",          "FileType::Immediate")
                 .entry("bsconfig.json",     "FileType::Immediate")
                 .entry("BUILD",             "FileType::Immediate")
                 .entry("BUILD.bazel",       "FileType::Immediate")
                 .entry("build.gradle",      "FileType::Immediate")
                 .entry("build.sbt",         "FileType::Immediate")
                 .entry("build.xml",         "FileType::Immediate")
                 .entry("Cargo.lock",        "FileType::Immediate")
                 .entry("Cargo.toml",        "FileType::Immediate")
                 .entry("CMakeLists.txt",    "FileType::Immediate")
                 .entry("composer.json",     "FileType::Immediate")
                 .entry("configure.ac",      "FileType::Immediate")
                 .entry("Configure.ac",      "FileType::Immediate")
                 .entry("Containerfile",     "FileType::Immediate")
                 .entry("Dockerfile",        "FileType::Immediate")
                 .entry("Earthfile",         "FileType::Immediate")
                 .entry("flake.lock",        "FileType::Immediate")
                 .entry("flake.nix",         "FileType::Immediate")
                 .entry("Gemfile",           "FileType::Immediate")
                 .entry("GNUmakefile",       "FileType::Immediate")
                 .entry("Gruntfile.coffee",  "FileType::Immediate")
                 .entry("Gruntfile.js",      "FileType::Immediate")
                 .entry("Justfile",          "FileType::Immediate")
                 .entry("justfile",          "FileType::Immediate")
                 .entry("Makefile",          "FileType::Immediate")
                 .entry("makefile",          "FileType::Immediate")
                 .entry("Makefile.in",       "FileType::Immediate")
                 .entry("makefile.in",       "FileType::Immediate")
                 .entry("meson.build",       "FileType::Immediate")
                 .entry("mix.exs",           "FileType::Immediate")
                 .entry("package.json",      "FileType::Immediate")
                 .entry("Pipfile",           "FileType::Immediate")
                 .entry("PKGBUILD",          "FileType::Immediate")
                 .entry("Podfile",           "FileType::Immediate")
                 .entry("pom.xml",           "FileType::Immediate")
                 .entry("Procfile",          "FileType::Immediate")
                 .entry("Rakefile",          "FileType::Immediate")
                 .entry("RoboFile.php",      "FileType::Immediate")
                 .entry("SConstruct",        "FileType::Immediate")
                 .entry("tsconfig.json",     "FileType::Immediate")
                 .entry("Vagrantfile",       "FileType::Immediate")
                 .entry("webpack.config.cjs","FileType::Immediate")
                 .entry("webpack.config.js", "FileType::Immediate")
                 .entry("WORKSPACE",         "FileType::Immediate")
                 .build()
    )
}

/// Generate mapping from lowercase file extension to file type.  If an image, video, music, or
/// lossless extension is added also update the extension icon map. For file types see
/// info/filetype.rs
fn generate_extension_type_map(file: &mut File) -> io::Result<()> {
    // Extension are converted to lower case for comparison
    writeln!(file, "static EXTENSION_TYPES: phf::Map<&'static str, FileType> = {};\n",
             phf_codegen::Map::new()
                 /* Immediate file - kick off the build of a project */
                 .entry("ninja",     "FileType::Immediate")
                 /* Image files */
                 .entry("arw",       "FileType::Image")
                 .entry("avif",      "FileType::Image")
                 .entry("bmp",       "FileType::Image")
                 .entry("cbr",       "FileType::Image")
                 .entry("cbz",       "FileType::Image")
                 .entry("cr2",       "FileType::Image")
                 .entry("dvi",       "FileType::Image")
                 .entry("eps",       "FileType::Image")
                 .entry("gif",       "FileType::Image")
                 .entry("heif",      "FileType::Image")
                 .entry("ico",       "FileType::Image")
                 .entry("j2c",       "FileType::Image")
                 .entry("j2k",       "FileType::Image")
                 .entry("jfi",       "FileType::Image")
                 .entry("jfif",      "FileType::Image")
                 .entry("jif",       "FileType::Image")
                 .entry("jp2",       "FileType::Image")
                 .entry("jpe",       "FileType::Image")
                 .entry("jpeg",      "FileType::Image")
                 .entry("jpf",       "FileType::Image")
                 .entry("jpg",       "FileType::Image")
                 .entry("jpx",       "FileType::Image")
                 .entry("jxl",       "FileType::Image")
                 .entry("nef",       "FileType::Image")
                 .entry("orf",       "FileType::Image")
                 .entry("pbm",       "FileType::Image")
                 .entry("pgm",       "FileType::Image")
                 .entry("png",       "FileType::Image")
                 .entry("pnm",       "FileType::Image")
                 .entry("ppm",       "FileType::Image")
                 .entry("ps",        "FileType::Image")
                 .entry("pxm",       "FileType::Image")
                 .entry("raw",       "FileType::Image")
                 .entry("stl",       "FileType::Image")
                 .entry("svg",       "FileType::Image")
                 .entry("tif",       "FileType::Image")
                 .entry("tiff",      "FileType::Image")
                 .entry("webp",      "FileType::Image")
                 .entry("xpm",       "FileType::Image")
                 /* Video files */
                 .entry("avi",       "FileType::Video")
                 .entry("flv",       "FileType::Video")
                 .entry("heic",      "FileType::Video")
                 .entry("m2ts",      "FileType::Video")
                 .entry("m2v",       "FileType::Video")
                 .entry("m4v",       "FileType::Video")
                 .entry("mkv",       "FileType::Video")
                 .entry("mov",       "FileType::Video")
                 .entry("mp4",       "FileType::Video")
                 .entry("mpeg",      "FileType::Video")
                 .entry("mpg",       "FileType::Video")
                 .entry("ogm",       "FileType::Video")
                 .entry("ogv",       "FileType::Video")
                 .entry("vob",       "FileType::Video")
                 .entry("webm",      "FileType::Video")
                 .entry("wmv",       "FileType::Video")
                 /* Music files */
                 .entry("aac",       "FileType::Music")
                 .entry("m4a",       "FileType::Music")
                 .entry("mka",       "FileType::Music")
                 .entry("mp2",       "FileType::Music")
                 .entry("mp3",       "FileType::Music")
                 .entry("ogg",       "FileType::Music")
                 .entry("opus",      "FileType::Music")
                 .entry("wma",       "FileType::Music")
                 /* Lossless music, rather than any other kind of data... */
                 .entry("alac",      "FileType::Lossless")
                 .entry("ape",       "FileType::Lossless")
                 .entry("flac",      "FileType::Lossless")
                 .entry("wav",       "FileType::Lossless")
                 /* Cryptology files */
                 .entry("asc",       "FileType::Crypto")
                 .entry("enc",       "FileType::Crypto")
                 .entry("gpg",       "FileType::Crypto")
                 .entry("p12",       "FileType::Crypto")
                 .entry("pfx",       "FileType::Crypto")
                 .entry("pgp",       "FileType::Crypto")
                 .entry("sig",       "FileType::Crypto")
                 .entry("signature", "FileType::Crypto")
                 /* Document files */
                 .entry("djvu",      "FileType::Document")
                 .entry("doc",       "FileType::Document")
                 .entry("docx",      "FileType::Document")
                 // .entry("dvi",       "FileType::Document") // already an image format
                 .entry("eml",       "FileType::Document")
                 // .entry("eps",       "FileType::Document") // already an image format
                 .entry("fotd",      "FileType::Document")
                 .entry("key",       "FileType::Document")
                 .entry("keynote",   "FileType::Document")
                 .entry("numbers",   "FileType::Document")
                 .entry("odp",       "FileType::Document")
                 .entry("odt",       "FileType::Document")
                 .entry("pages",     "FileType::Document")
                 .entry("pdf",       "FileType::Document")
                 .entry("ppt",       "FileType::Document")
                 .entry("pptx",      "FileType::Document")
                 .entry("rtf",       "FileType::Document")
                 .entry("xls",       "FileType::Document")
                 .entry("xlsx",      "FileType::Document")
                 /* Compressed/archive files */
                 .entry("7z",        "FileType::Compressed")
                 .entry("a",         "FileType::Compressed")
                 .entry("ar",        "FileType::Compressed")
                 .entry("bz",        "FileType::Compressed")
                 .entry("bz2",       "FileType::Compressed")
                 .entry("cpio",      "FileType::Compressed")
                 .entry("deb",       "FileType::Compressed")
                 .entry("dmg",       "FileType::Compressed")
                 .entry("gz",        "FileType::Compressed")
                 .entry("iso",       "FileType::Compressed")
                 .entry("lz",        "FileType::Compressed")
                 .entry("lz4",       "FileType::Compressed")
                 .entry("lzh",       "FileType::Compressed")
                 .entry("lzma",      "FileType::Compressed")
                 .entry("lzo",       "FileType::Compressed")
                 .entry("par",       "FileType::Compressed")
                 .entry("rar",       "FileType::Compressed")
                 .entry("rpm",       "FileType::Compressed")
                 .entry("tar",       "FileType::Compressed")
                 .entry("taz",       "FileType::Compressed")
                 .entry("tbz",       "FileType::Compressed")
                 .entry("tbz2",      "FileType::Compressed")
                 .entry("tc",        "FileType::Compressed")
                 .entry("tgz",       "FileType::Compressed")
                 .entry("tlz",       "FileType::Compressed")
                 .entry("txz",       "FileType::Compressed")
                 .entry("tz",        "FileType::Compressed")
                 .entry("tzo",       "FileType::Compressed")
                 .entry("xz",        "FileType::Compressed")
                 .entry("z",         "FileType::Compressed")
                 .entry("zip",       "FileType::Compressed")
                 .entry("zst",       "FileType::Compressed")
                 /* Temporary files */
                 .entry("bak",       "FileType::Temp")
                 .entry("bk",        "FileType::Temp")
                 .entry("bkp",       "FileType::Temp")
                 .entry("swn",       "FileType::Temp")
                 .entry("swo",       "FileType::Temp")
                 .entry("swp",       "FileType::Temp")
                 .entry("tmp",       "FileType::Temp")
                 /* Compiler output files */
                 .entry("class",     "FileType::Compiled")
                 .entry("elc",       "FileType::Compiled")
                 .entry("hi",        "FileType::Compiled")
                 .entry("ko",        "FileType::Compiled")
                 .entry("o",         "FileType::Compiled")
                 .entry("pyc",       "FileType::Compiled")
                 .entry("zwc",       "FileType::Compiled")
                 .build()
    )
}

/// Generate mapping from full filenames to file type. This mapping should also contain all the
/// "dot" directories that have a custom icon.  See output/render/icons.rs for a partial list of
/// icon constants.
fn generate_filename_icon_map(file: &mut File) -> io::Result<()> {
    writeln!(file, "static FILENAME_ICONS: phf::Map<&'static str, char> = {};\n",
             phf_codegen::Map::new()
                 .entry(".atom",              "'\u{e764}'") // 
                 .entry(".bashprofile",       "'\u{e615}'") // 
                 .entry(".bashrc",            "'\u{f489}'") // 
                 .entry(".emacs",             "'\u{e632}'") // 
                 .entry(".git",               "'\u{f1d3}'") // 
                 .entry(".gitattributes",     "'\u{f1d3}'") // 
                 .entry(".gitconfig",         "'\u{f1d3}'") // 
                 .entry(".github",            "'\u{f408}'") // 
                 .entry(".gitignore",         "'\u{f1d3}'") // 
                 .entry(".gitignore_global",  "'\u{f1d3}'") // 
                 .entry(".gitmodules",        "'\u{f1d3}'") // 
                 .entry(".idea",              "'\u{e7b5}'") // 
                 .entry(".rvm",               "'\u{e21e}'") // 
                 .entry(".Trash",             "'\u{f1f8}'") // 
                 .entry(".vimrc",             "'\u{e7c5}'") // 
                 .entry(".vscode",            "'\u{e70c}'") // 
                 .entry(".zshrc",             "'\u{f489}'") // 
                 .entry("bin",                "'\u{e5fc}'") // 
                 .entry("Cargo.lock",         "'\u{e7a8}'") // 
                 .entry("config",             "'\u{e5fc}'") // 
                 .entry("docker-compose.yml", "'\u{f308}'") // 
                 .entry("Dockerfile",         "'\u{f308}'") // 
                 .entry("ds_store",           "'\u{f179}'") // 
                 .entry("Earthfile",          "'\u{f0ac}'") // 
                 .entry("gitignore_global",   "'\u{f1d3}'") // 
                 .entry("gitlab-ci.yml",      "'\u{f296}'") // 
                 .entry("go.mod",             "'\u{e626}'") // 
                 .entry("go.sum",             "'\u{e626}'") // 
                 .entry("gradle",             "'\u{e256}'") // 
                 .entry("gruntfile.coffee",   "'\u{e611}'") // 
                 .entry("gruntfile.js",       "'\u{e611}'") // 
                 .entry("gruntfile.ls",       "'\u{e611}'") // 
                 .entry("gulpfile.coffee",    "'\u{e610}'") // 
                 .entry("gulpfile.js",        "'\u{e610}'") // 
                 .entry("gulpfile.ls",        "'\u{e610}'") // 
                 .entry("hidden",             "'\u{f023}'") // 
                 .entry("include",            "'\u{e5fc}'") // 
                 .entry("lib",                "'\u{f121}'") // 
                 .entry("LICENSE",            "'\u{f02d}'") // 
                 .entry("localized",          "'\u{f179}'") // 
                 .entry("Makefile",           "'\u{f489}'") // 
                 .entry("node_modules",       "'\u{e718}'") // 
                 .entry("npmignore",          "'\u{e71e}'") // 
                 .entry("PKGBUILD",           "'\u{f303}'") // 
                 .entry("rubydoc",            "'\u{e73b}'") // 
                 .entry("Vagrantfile",        "'\u{2371}'") // ⍱
                 .entry("yarn.lock",          "'\u{e718}'") // 
                 .build()
    )
}

/// Generate mapping from lowercase file extension to icons.  If an image, video, or audio
/// extension is add also update the extension filetype map.  See output/render/icons.rs for
/// a partial list of icon constants.
fn generate_extension_icon_map(file: &mut File) -> io::Result<()> {
    writeln!(file, "static EXTENSION_ICONS: phf::Map<&'static str, char> = {};\n",
             phf_codegen::Map::new()
                 .entry("7z",            "'\u{f410}'")  // 
                 .entry("a",             "'\u{f17c}'")  // 
                 .entry("acc",           "'\u{f001}'")  // 
                 .entry("acf",           "'\u{f1b6}'")  // 
                 .entry("ai",            "'\u{e7b4}'")  // 
                 .entry("alac",          "'\u{f001}'")  // 
                 .entry("android",       "'\u{e70e}'")  // 
                 .entry("ape",           "'\u{f001}'")  // 
                 .entry("apk",           "'\u{e70e}'")  // 
                 .entry("apple",         "'\u{f179}'")  // 
                 .entry("ar",            "'\u{f410}'")  // 
                 .entry("arw",           "'\u{f1c5}'")  // 
                 .entry("asm",           "'\u{e637}'")  // 
                 .entry("avi",           "'\u{f03d}'")  // 
                 .entry("avif",          "'\u{f1c5}'")  // 
                 .entry("avro",          "'\u{e60b}'")  // 
                 .entry("awk",           "'\u{f489}'")  // 
                 .entry("bash",          "'\u{f489}'")  // 
                 .entry("bashrc",        "'\u{f489}'")  // 
                 .entry("bash_history",  "'\u{f489}'")  // 
                 .entry("bash_profile",  "'\u{f489}'")  // 
                 .entry("bat",           "'\u{ebc4}'")  // 
                 .entry("bats",          "'\u{f489}'")  // 
                 .entry("bib",           "'\u{e69b}'")  // 
                 .entry("bin",           "'\u{eae8}'")  // 
                 .entry("bmp",           "'\u{f1c5}'")  // 
                 .entry("bst",           "'\u{e69b}'")  // 
                 .entry("bz",            "'\u{f410}'")  // 
                 .entry("bz2",           "'\u{f410}'")  // 
                 .entry("c",             "'\u{e61e}'")  // 
                 .entry("c++",           "'\u{e61d}'")  // 
                 .entry("cab",           "'\u{e70f}'")  // 
                 .entry("cbr",           "'\u{f1c5}'")  // 
                 .entry("cbz",           "'\u{f1c5}'")  // 
                 .entry("cc",            "'\u{e61d}'")  // 
                 .entry("cert",          "'\u{eafa}'")  // 
                 .entry("cfg",           "'\u{e615}'")  // 
                 .entry("cjs",           "'\u{e74e}'")  // 
                 .entry("class",         "'\u{e256}'")  // 
                 .entry("clj",           "'\u{e768}'")  // 
                 .entry("cljs",          "'\u{e76a}'")  // 
                 .entry("cls",           "'\u{e69b}'")  // 
                 .entry("cmd",           "'\u{e70f}'")  // 
                 .entry("coffee",        "'\u{f0f4}'")  // 
                 .entry("conf",          "'\u{e615}'")  // 
                 .entry("config",        "'\u{e615}'")  // 
                 .entry("cp",            "'\u{e61d}'")  // 
                 .entry("cpio",          "'\u{f410}'")  // 
                 .entry("cpp",           "'\u{e61d}'")  // 
                 .entry("cr2",           "'\u{f1c5}'")  // 
                 .entry("crt",           "'\u{eafa}'")  // 
                 .entry("cs",            "'\u{f031b}'") // 󰌛
                 .entry("csh",           "'\u{f489}'")  // 
                 .entry("cshtml",        "'\u{f1fa}'")  // 
                 .entry("csproj",        "'\u{f031b}'") // 󰌛
                 .entry("css",           "'\u{e749}'")  // 
                 .entry("csv",           "'\u{f1c3}'")  // 
                 .entry("csx",           "'\u{f031b}'") // 󰌛
                 .entry("cts",           "'\u{e628}'")  // 
                 .entry("cu",            "'\u{e64b}'")  // 
                 .entry("cxx",           "'\u{e61d}'")  // 
                 .entry("d",             "'\u{e7af}'")  // 
                 .entry("dart",          "'\u{e798}'")  // 
                 .entry("db",            "'\u{f1c0}'")  // 
                 .entry("deb",           "'\u{e77d}'")  // 
                 .entry("desktop",       "'\u{ebd1}'")  // 
                 .entry("diff",          "'\u{f440}'")  // 
                 .entry("djvu",          "'\u{f02d}'")  // 
                 .entry("dll",           "'\u{e70f}'")  // 
                 .entry("dmg",           "'\u{e271}'")  // 
                 .entry("doc",           "'\u{f1c2}'")  // 
                 .entry("docx",          "'\u{f1c2}'")  // 
                 .entry("drawio",        "'\u{ebba}'")  // 
                 .entry("ds_store",      "'\u{f179}'")  // 
                 .entry("dump",          "'\u{f1c0}'")  // 
                 .entry("dvi",           "'\u{f1c5}'")  // 
                 .entry("ebook",         "'\u{e28b}'")  // 
                 .entry("ebuild",        "'\u{f30d}'")  // 
                 .entry("editorconfig",  "'\u{e615}'")  // 
                 .entry("ejs",           "'\u{e618}'")  // 
                 .entry("el",            "'\u{e632}'")  // 
                 .entry("elm",           "'\u{e62c}'")  // 
                 .entry("eml",           "'\u{f003}'")  // 
                 .entry("env",           "'\u{f462}'")  // 
                 .entry("eot",           "'\u{f031}'")  // 
                 .entry("eps",           "'\u{f1c5}'")  // 
                 .entry("epub",          "'\u{e28a}'")  // 
                 .entry("erb",           "'\u{e73b}'")  // 
                 .entry("erl",           "'\u{e7b1}'")  // 
                 .entry("ex",            "'\u{e62d}'")  // 
                 .entry("exe",           "'\u{f17a}'")  // 
                 .entry("exs",           "'\u{e62d}'")  // 
                 .entry("fish",          "'\u{f489}'")  // 
                 .entry("flac",          "'\u{f001}'")  // 
                 .entry("flv",           "'\u{f03d}'")  // 
                 .entry("font",          "'\u{f031}'")  // 
                 .entry("fs",            "'\u{e7a7}'")  // 
                 .entry("fsi",           "'\u{e7a7}'")  // 
                 .entry("fsx",           "'\u{e7a7}'")  // 
                 .entry("gdoc",          "'\u{f1c2}'")  // 
                 .entry("gem",           "'\u{e21e}'")  // 
                 .entry("gemfile",       "'\u{e21e}'")  // 
                 .entry("gemspec",       "'\u{e21e}'")  // 
                 .entry("gform",         "'\u{f298}'")  // 
                 .entry("gif",           "'\u{f1c5}'")  // 
                 .entry("git",           "'\u{f1d3}'")  // 
                 .entry("gitattributes", "'\u{f1d3}'")  // 
                 .entry("gitignore",     "'\u{f1d3}'")  // 
                 .entry("gitmodules",    "'\u{f1d3}'")  // 
                 .entry("go",            "'\u{e626}'")  // 
                 .entry("gpg",           "'\u{e60a}'")  // 
                 .entry("gradle",        "'\u{e256}'")  // 
                 .entry("groovy",        "'\u{e775}'")  // 
                 .entry("gsheet",        "'\u{f1c3}'")  // 
                 .entry("gslides",       "'\u{f1c4}'")  // 
                 .entry("guardfile",     "'\u{e21e}'")  // 
                 .entry("gz",            "'\u{f410}'")  // 
                 .entry("h",             "'\u{f0fd}'")  // 
                 .entry("hbs",           "'\u{e60f}'")  // 
                 .entry("heic",          "'\u{f03d}'")  // 
                 .entry("heif",          "'\u{f1c5}'")  // 
                 .entry("hpp",           "'\u{f0fd}'")  // 
                 .entry("hs",            "'\u{e777}'")  // 
                 .entry("htm",           "'\u{f13b}'")  // 
                 .entry("html",          "'\u{f13b}'")  // 
                 .entry("hxx",           "'\u{f0fd}'")  // 
                 .entry("ical",          "'\u{eab0}'")  // 
                 .entry("icalendar",     "'\u{eab0}'")  // 
                 .entry("ico",           "'\u{f1c5}'")  // 
                 .entry("ics",           "'\u{eab0}'")  // 
                 .entry("ifb",           "'\u{eab0}'")  // 
                 .entry("image",         "'\u{f1c5}'")  // 
                 .entry("img",           "'\u{e271}'")  // 
                 .entry("iml",           "'\u{e7b5}'")  // 
                 .entry("ini",           "'\u{f17a}'")  // 
                 .entry("ipynb",         "'\u{e678}'")  // 
                 .entry("iso",           "'\u{e271}'")  // 
                 .entry("j2c",           "'\u{f1c5}'")  // 
                 .entry("j2k",           "'\u{f1c5}'")  // 
                 .entry("jad",           "'\u{e256}'")  // 
                 .entry("jar",           "'\u{e256}'")  // 
                 .entry("java",          "'\u{e256}'")  // 
                 .entry("jfi",           "'\u{f1c5}'")  // 
                 .entry("jfif",          "'\u{f1c5}'")  // 
                 .entry("jif",           "'\u{f1c5}'")  // 
                 .entry("jl",            "'\u{e624}'")  // 
                 .entry("jmd",           "'\u{f48a}'")  // 
                 .entry("jp2",           "'\u{f1c5}'")  // 
                 .entry("jpe",           "'\u{f1c5}'")  // 
                 .entry("jpeg",          "'\u{f1c5}'")  // 
                 .entry("jpf",           "'\u{f1c5}'")  // 
                 .entry("jpg",           "'\u{f1c5}'")  // 
                 .entry("jpx",           "'\u{f1c5}'")  // 
                 .entry("js",            "'\u{e74e}'")  // 
                 .entry("json",          "'\u{e60b}'")  // 
                 .entry("jsx",           "'\u{e7ba}'")  // 
                 .entry("jxl",           "'\u{f1c5}'")  // 
                 .entry("kdb",           "'\u{f23e}'")  // 
                 .entry("kdbx",          "'\u{f23e}'")  // 
                 .entry("key",           "'\u{eb11}'")  // 
                 .entry("ko",            "'\u{f17c}'")  // 
                 .entry("ksh",           "'\u{f489}'")  // 
                 .entry("latex",         "'\u{e69b}'")  // 
                 .entry("less",          "'\u{e758}'")  // 
                 .entry("lhs",           "'\u{e777}'")  // 
                 .entry("license",       "'\u{f02d}'")  // 
                 .entry("localized",     "'\u{f179}'")  // 
                 .entry("lock",          "'\u{f023}'")  // 
                 .entry("log",           "'\u{f18d}'")  // 
                 .entry("lua",           "'\u{e620}'")  // 
                 .entry("lz",            "'\u{f410}'")  // 
                 .entry("lz4",           "'\u{f410}'")  // 
                 .entry("lzh",           "'\u{f410}'")  // 
                 .entry("lzma",          "'\u{f410}'")  // 
                 .entry("lzo",           "'\u{f410}'")  // 
                 .entry("m",             "'\u{e61e}'")  // 
                 .entry("m2ts",          "'\u{f03d}'")  // 
                 .entry("m2v",           "'\u{f03d}'")  // 
                 .entry("m4a",           "'\u{f001}'")  // 
                 .entry("m4v",           "'\u{f03d}'")  // 
                 .entry("magnet",        "'\u{f076}'")  // 
                 .entry("markdown",      "'\u{f48a}'")  // 
                 .entry("md",            "'\u{f48a}'")  // 
                 .entry("mjs",           "'\u{e74e}'")  // 
                 .entry("mk",            "'\u{f489}'")  // 
                 .entry("mka",           "'\u{f001}'")  // 
                 .entry("mkd",           "'\u{f48a}'")  // 
                 .entry("mkv",           "'\u{f03d}'")  // 
                 .entry("ml",            "'\u{e67a}'")  // 
                 .entry("mli",           "'\u{e67a}'")  // 
                 .entry("mll",           "'\u{e67a}'")  // 
                 .entry("mly",           "'\u{e67a}'")  // 
                 .entry("mm",            "'\u{e61d}'")  // 
                 .entry("mobi",          "'\u{e28b}'")  // 
                 .entry("mov",           "'\u{f03d}'")  // 
                 .entry("mp2",           "'\u{f001}'")  // 
                 .entry("mp3",           "'\u{f001}'")  // 
                 .entry("mp4",           "'\u{f03d}'")  // 
                 .entry("mpeg",          "'\u{f03d}'")  // 
                 .entry("mpg",           "'\u{f03d}'")  // 
                 .entry("msi",           "'\u{e70f}'")  // 
                 .entry("mts",           "'\u{e628}'")  // 
                 .entry("mustache",      "'\u{e60f}'")  // 
                 .entry("nef",           "'\u{f1c5}'")  // 
                 .entry("ninja",         "'\u{f0774}'") // 󰝴
                 .entry("nix",           "'\u{f313}'")  // 
                 .entry("node",          "'\u{f0399}'") // 󰎙
                 .entry("npmignore",     "'\u{e71e}'")  // 
                 .entry("o",             "'\u{eae8}'")  // 
                 .entry("odp",           "'\u{f1c4}'")  // 
                 .entry("ods",           "'\u{f1c3}'")  // 
                 .entry("odt",           "'\u{f1c2}'")  // 
                 .entry("ogg",           "'\u{f001}'")  // 
                 .entry("ogm",           "'\u{f03d}'")  // 
                 .entry("ogv",           "'\u{f03d}'")  // 
                 .entry("opus",          "'\u{f001}'")  // 
                 .entry("orf",           "'\u{f1c5}'")  // 
                 .entry("org",           "'\u{e633}'")  // 
                 .entry("otf",           "'\u{f031}'")  // 
                 .entry("out",           "'\u{eb2c}'")  // 
                 .entry("par",           "'\u{f410}'")  // 
                 .entry("part",          "'\u{f43a}'")  // 
                 .entry("patch",         "'\u{f440}'")  // 
                 .entry("pbm",           "'\u{f1c5}'")  // 
                 .entry("pdf",           "'\u{f1c1}'")  // 
                 .entry("pem",           "'\u{eb11}'")  // 
                 .entry("pgm",           "'\u{f1c5}'")  // 
                 .entry("php",           "'\u{e73d}'")  // 
                 .entry("pl",            "'\u{e769}'")  // 
                 .entry("plx",           "'\u{e769}'")  // 
                 .entry("pm",            "'\u{e769}'")  // 
                 .entry("png",           "'\u{f1c5}'")  // 
                 .entry("pnm",           "'\u{f1c5}'")  // 
                 .entry("pod",           "'\u{e769}'")  // 
                 .entry("ppm",           "'\u{f1c5}'")  // 
                 .entry("ppt",           "'\u{f1c4}'")  // 
                 .entry("pptx",          "'\u{f1c4}'")  // 
                 .entry("procfile",      "'\u{e21e}'")  // 
                 .entry("properties",    "'\u{e60b}'")  // 
                 .entry("ps",            "'\u{f1c5}'")  // 
                 .entry("ps1",           "'\u{ebc7}'")  // 
                 .entry("psd",           "'\u{e7b8}'")  // 
                 .entry("psd1",          "'\u{ebc7}'")  // 
                 .entry("psm1",          "'\u{ebc7}'")  // 
                 .entry("pxm",           "'\u{f1c5}'")  // 
                 .entry("py",            "'\u{e606}'")  // 
                 .entry("pyc",           "'\u{e606}'")  // 
                 .entry("qcow2",         "'\u{e271}'")  // 
                 .entry("r",             "'\u{f25d}'")  // 
                 .entry("rakefile",      "'\u{e21e}'")  // 
                 .entry("rar",           "'\u{f410}'")  // 
                 .entry("raw",           "'\u{f1c5}'")  // 
                 .entry("razor",         "'\u{f1fa}'")  // 
                 .entry("rb",            "'\u{e21e}'")  // 
                 .entry("rdata",         "'\u{f25d}'")  // 
                 .entry("rdb",           "'\u{e76d}'")  // 
                 .entry("rdoc",          "'\u{f48a}'")  // 
                 .entry("rds",           "'\u{f25d}'")  // 
                 .entry("readme",        "'\u{f48a}'")  // 
                 .entry("rlib",          "'\u{e7a8}'")  // 
                 .entry("rmd",           "'\u{f48a}'")  // 
                 .entry("rmeta",         "'\u{e7a8}'")  // 
                 .entry("rpm",           "'\u{e7bb}'")  // 
                 .entry("rs",            "'\u{e7a8}'")  // 
                 .entry("rspec",         "'\u{e21e}'")  // 
                 .entry("rspec_parallel","'\u{e21e}'")  // 
                 .entry("rspec_status",  "'\u{e21e}'")  // 
                 .entry("rss",           "'\u{f09e}'")  // 
                 .entry("rst",           "'\u{f15c}'")  // 
                 .entry("rtf",           "'\u{f0219}'") // 󰈙
                 .entry("ru",            "'\u{e21e}'")  // 
                 .entry("rubydoc",       "'\u{e73b}'")  // 
                 .entry("s",             "'\u{e637}'")  // 
                 .entry("sass",          "'\u{e603}'")  // 
                 .entry("scala",         "'\u{e737}'")  // 
                 .entry("scss",          "'\u{e749}'")  // 
                 .entry("service",       "'\u{eba2}'")  // 
                 .entry("sh",            "'\u{f489}'")  // 
                 .entry("shell",         "'\u{f489}'")  // 
                 .entry("slim",          "'\u{e73b}'")  // 
                 .entry("sln",           "'\u{e70c}'")  // 
                 .entry("so",            "'\u{f17c}'")  // 
                 .entry("sql",           "'\u{f1c0}'")  // 
                 .entry("sqlite3",       "'\u{e7c4}'")  // 
                 .entry("stl",           "'\u{f1c5}'")  // 
                 .entry("sty",           "'\u{e69b}'")  // 
                 .entry("styl",          "'\u{e600}'")  // 
                 .entry("stylus",        "'\u{e600}'")  // 
                 .entry("svelte",        "'\u{e697}'")  // 
                 .entry("svg",           "'\u{f1c5}'")  // 
                 .entry("swift",         "'\u{e755}'")  // 
                 .entry("t",             "'\u{e769}'")  // 
                 .entry("tar",           "'\u{f410}'")  // 
                 .entry("taz",           "'\u{f410}'")  // 
                 .entry("tbz",           "'\u{f410}'")  // 
                 .entry("tbz2",          "'\u{f410}'")  // 
                 .entry("tc",            "'\u{f410}'")  // 
                 .entry("tex",           "'\u{e69b}'")  // 
                 .entry("tgz",           "'\u{f410}'")  // 
                 .entry("tif",           "'\u{f1c5}'")  // 
                 .entry("tiff",          "'\u{f1c5}'")  // 
                 .entry("tlz",           "'\u{f410}'")  // 
                 .entry("toml",          "'\u{e615}'")  // 
                 .entry("torrent",       "'\u{e275}'")  // 
                 .entry("ts",            "'\u{e628}'")  // 
                 .entry("tsv",           "'\u{f1c3}'")  // 
                 .entry("tsx",           "'\u{e7ba}'")  // 
                 .entry("ttf",           "'\u{f031}'")  // 
                 .entry("twig",          "'\u{e61c}'")  // 
                 .entry("txt",           "'\u{f15c}'")  // 
                 .entry("txz",           "'\u{f410}'")  // 
                 .entry("tz",            "'\u{f410}'")  // 
                 .entry("tzo",           "'\u{f410}'")  // 
                 .entry("unity",         "'\u{e721}'")  // 
                 .entry("unity3d",       "'\u{e721}'")  // 
                 .entry("vdi",           "'\u{e271}'")  // 
                 .entry("vhd",           "'\u{e271}'")  // 
                 .entry("video",         "'\u{f03d}'")  // 
                 .entry("vim",           "'\u{e7c5}'")  // 
                 .entry("vmdk",          "'\u{e271}'")  // 
                 .entry("vob",           "'\u{f03d}'")  // 
                 .entry("vue",           "'\u{f0844}'") // 󰡄
                 .entry("war",           "'\u{e256}'")  // 
                 .entry("wav",           "'\u{f001}'")  // 
                 .entry("webm",          "'\u{f03d}'")  // 
                 .entry("webp",          "'\u{f1c5}'")  // 
                 .entry("windows",       "'\u{f17a}'")  // 
                 .entry("wma",           "'\u{f001}'")  // 
                 .entry("wmv",           "'\u{f03d}'")  // 
                 .entry("woff",          "'\u{f031}'")  // 
                 .entry("woff2",         "'\u{f031}'")  // 
                 .entry("xhtml",         "'\u{f13b}'")  // 
                 .entry("xls",           "'\u{f1c3}'")  // 
                 .entry("xlsm",          "'\u{f1c3}'")  // 
                 .entry("xlsx",          "'\u{f1c3}'")  // 
                 .entry("xml",           "'\u{f05c0}'") // 󰗀
                 .entry("xpm",           "'\u{f1c5}'")  // 
                 .entry("xul",           "'\u{f05c0}'") // 󰗀
                 .entry("xz",            "'\u{f410}'")  // 
                 .entry("yaml",          "'\u{f481}'")  // 
                 .entry("yml",           "'\u{f481}'")  // 
                 .entry("z",             "'\u{f410}'")  // 
                 .entry("zig",           "'\u{21af}'")  // ↯
                 .entry("zip",           "'\u{f410}'")  // 
                 .entry("zsh",           "'\u{f489}'")  // 
                 .entry("zsh-theme",     "'\u{f489}'")  // 
                 .entry("zshrc",         "'\u{f489}'")  // 
                 .entry("zst",           "'\u{f410}'")  // 
                 .build()
    )
}