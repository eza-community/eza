// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-License-Identifier: MIT
//! Lines-of-code counting.
//!
//! This is eza’s own, dependency-free source-code counter. Given a file’s
//! name we work out which programming language it’s written in (from its
//! extension, or its whole name for files like `Makefile`), and then we walk
//! its contents a line at a time, classifying each physical line as one of:
//!
//! - **code** — a line that contains at least one character of actual source,
//! - **comment** — a line that is entirely comment (line- or block-style),
//! - **blank** — a line that is empty or only whitespace.
//!
//! The counter is deliberately *comment-aware* rather than a naïve line
//! count: it understands each language’s line- and block-comment syntax and
//! skips over string literals so that a `//` inside `"https://…"` is not
//! mistaken for a comment. Its known limitations are that block comments are
//! treated as non-nesting and that only single-line string literals are
//! tracked; both are rare enough in practice not to skew a project’s totals.

use std::collections::BTreeMap;
use std::io;
use std::ops::AddAssign;
use std::path::{Path, PathBuf};

use phf::{Map, phf_map};
use rayon::prelude::*;

/// A programming language eza knows how to count, along with the comment
/// syntax needed to tell code from commentary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Language {
    /// The human-readable name shown in the summary and language column.
    pub name: &'static str,

    /// Tokens that begin a comment lasting to the end of the line.
    pub line_comments: &'static [&'static str],

    /// `(open, close)` delimiter pairs for block comments.
    pub block_comments: &'static [(&'static str, &'static str)],
}

/// The tally of lines within a single file, or an aggregate across many.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LocCounts {
    /// Total physical lines. Always equals `code + comments + blanks`.
    pub lines: usize,

    /// Lines containing at least one character of source code.
    pub code: usize,

    /// Lines that are entirely comment.
    pub comments: usize,

    /// Empty or whitespace-only lines.
    pub blanks: usize,
}

impl AddAssign for LocCounts {
    fn add_assign(&mut self, rhs: Self) {
        self.lines += rhs.lines;
        self.code += rhs.code;
        self.comments += rhs.comments;
        self.blanks += rhs.blanks;
    }
}

impl LocCounts {
    /// Count the lines of the given `source`, using `lang`’s comment rules.
    #[must_use]
    pub fn from_source(source: &str, lang: &Language) -> Self {
        let mut counts = Self::default();
        // The block-comment terminator we’re currently hunting for, if any.
        // This is threaded across lines so multi-line block comments work.
        let mut block: Option<&'static str> = None;

        for line in source.lines() {
            counts.lines += 1;
            let (has_code, has_comment) = classify_line(line, lang, &mut block);
            if has_code {
                counts.code += 1;
            } else if has_comment {
                counts.comments += 1;
            } else {
                counts.blanks += 1;
            }
        }

        counts
    }

    /// Read `path` and count its lines. Returns `Ok(None)` for files that
    /// aren’t valid UTF-8 (i.e. binaries), which we simply don’t count.
    pub fn from_path(path: &Path, lang: &Language) -> io::Result<Option<Self>> {
        match std::fs::read_to_string(path) {
            Ok(source) => Ok(Some(Self::from_source(&source, lang))),
            Err(e) if e.kind() == io::ErrorKind::InvalidData => Ok(None),
            Err(e) => Err(e),
        }
    }
}

/// Classify a single physical line, updating `block` with any block-comment
/// state that carries over to the next line. Returns `(has_code, has_comment)`.
fn classify_line(line: &str, lang: &Language, block: &mut Option<&'static str>) -> (bool, bool) {
    let mut has_code = false;
    let mut has_comment = false;
    let mut rest = line;

    'scan: loop {
        // Inside a block comment: everything up to the closing delimiter is
        // comment. If it never closes on this line, the block continues.
        if let Some(close) = *block {
            has_comment = true;
            match rest.find(close) {
                Some(pos) => {
                    rest = &rest[pos + close.len()..];
                    *block = None;
                    continue;
                }
                None => break,
            }
        }

        rest = rest.trim_start();
        if rest.is_empty() {
            break;
        }

        // A line comment swallows the remainder of the line.
        if lang.line_comments.iter().any(|lc| rest.starts_with(lc)) {
            has_comment = true;
            break;
        }

        // A block comment opener starts a (possibly multi-line) comment.
        if let Some((open, close)) = lang
            .block_comments
            .iter()
            .find(|(open, _)| rest.starts_with(open))
        {
            has_comment = true;
            *block = Some(close);
            rest = &rest[open.len()..];
            continue 'scan;
        }

        // Anything else is code. Consume one unit, skipping over string
        // literals so their contents can’t be mistaken for comments.
        has_code = true;
        let c = rest.chars().next().unwrap();
        if c == '"' {
            rest = consume_string(&rest[c.len_utf8()..], '"');
        } else {
            rest = &rest[c.len_utf8()..];
        }
    }

    (has_code, has_comment)
}

/// Consume a string literal body, returning the slice after the closing
/// `quote`. Backslash escapes are honoured; an unterminated string consumes
/// the rest of the line.
fn consume_string(s: &str, quote: char) -> &str {
    let mut chars = s.char_indices();
    while let Some((i, c)) = chars.next() {
        if c == '\\' {
            chars.next();
        } else if c == quote {
            return &s[i + c.len_utf8()..];
        }
    }
    ""
}

/// Work out the language of a file from its whole name (for files like
/// `Makefile`) or, failing that, its already-lowercased extension.
#[must_use]
pub fn language_for(name: &str, ext: Option<&str>) -> Option<&'static Language> {
    if let Some(lang) = BY_FILENAME.get(name) {
        return Some(lang);
    }
    ext.and_then(|e| BY_EXTENSION.get(e)).copied()
}

/// The aggregate line counts for a single language across many files.
#[derive(Debug, Clone)]
pub struct LangStat {
    pub language: &'static Language,
    pub files: usize,
    pub counts: LocCounts,

    /// The name and lowercase extension of one counted file, giving the
    /// summary view a representative file to pick the language’s icon from.
    pub rep_file: (String, Option<String>),
}

/// The result of counting a whole tree: a per-language breakdown, ordered by
/// language name for stable output.
#[derive(Debug, Default, Clone)]
pub struct Report {
    languages: BTreeMap<&'static str, LangStat>,
}

impl Report {
    fn add(&mut self, language: &'static Language, counts: LocCounts, path: &Path) {
        let stat = self.languages.entry(language.name).or_insert_with(|| {
            let name = path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or_default()
                .to_string();
            let ext = path
                .extension()
                .and_then(|s| s.to_str())
                .map(str::to_ascii_lowercase);
            LangStat {
                language,
                files: 0,
                counts: LocCounts::default(),
                rep_file: (name, ext),
            }
        });
        stat.files += 1;
        stat.counts += counts;
    }

    /// The per-language rows, ordered by language name.
    pub fn languages(&self) -> impl Iterator<Item = &LangStat> {
        self.languages.values()
    }

    /// The grand total across every language.
    #[must_use]
    pub fn total(&self) -> LocCounts {
        let mut total = LocCounts::default();
        for stat in self.languages.values() {
            total += stat.counts;
        }
        total
    }

    /// The total number of counted files.
    #[must_use]
    pub fn total_files(&self) -> usize {
        self.languages.values().map(|s| s.files).sum()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.languages.is_empty()
    }
}

/// Recursively count every recognised source file under `roots`, using
/// `is_ignored` to skip files (e.g. those matched by `.gitignore`). Hidden
/// entries and symbolic links are always skipped, so `.git` and friends never
/// get walked. Counting itself is parallelised across a thread pool.
pub fn count_tree<F>(roots: &[PathBuf], is_ignored: &F) -> Report
where
    F: Fn(&Path) -> bool,
{
    let mut jobs: Vec<(PathBuf, &'static Language)> = Vec::new();
    for root in roots {
        collect_jobs(root, is_ignored, &mut jobs);
    }

    let counted: Vec<(&'static Language, LocCounts, &PathBuf)> = jobs
        .par_iter()
        .filter_map(|(path, lang)| {
            LocCounts::from_path(path, lang)
                .ok()
                .flatten()
                .map(|counts| (*lang, counts, path))
        })
        .collect();

    let mut report = Report::default();
    for (lang, counts, path) in counted {
        report.add(lang, counts, path);
    }
    report
}

/// Walk one path, gathering `(file, language)` jobs for every recognised
/// source file beneath it.
fn collect_jobs<F>(path: &Path, is_ignored: &F, jobs: &mut Vec<(PathBuf, &'static Language)>)
where
    F: Fn(&Path) -> bool,
{
    let Ok(meta) = std::fs::symlink_metadata(path) else {
        return;
    };
    let file_type = meta.file_type();

    // Never follow symlinks: it risks cycles and double-counting.
    if file_type.is_symlink() {
        return;
    }

    if file_type.is_file() {
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .map(str::to_ascii_lowercase);
        if let Some(lang) = language_for(name, ext.as_deref()) {
            jobs.push((path.to_path_buf(), lang));
        }
        return;
    }

    if file_type.is_dir() {
        let Ok(entries) = std::fs::read_dir(path) else {
            return;
        };
        for entry in entries.flatten() {
            // Skip hidden entries by default — this also keeps us out of `.git`.
            if entry.file_name().to_string_lossy().starts_with('.') {
                continue;
            }
            let child = entry.path();
            if is_ignored(&child) {
                continue;
            }
            collect_jobs(&child, is_ignored, jobs);
        }
    }
}

/// Count the given `roots`, respecting a repository’s `.gitignore` if the
/// `git` feature is enabled and the roots live inside one. This is the entry
/// point used by both the `--loc` percentage columns and the `--code` summary.
#[must_use]
pub fn count_roots(roots: &[PathBuf]) -> Report {
    #[cfg(feature = "git")]
    {
        if let Some(first) = roots.first()
            && let Ok(repo) = git2::Repository::discover(first)
        {
            // `is_path_ignored` wants a path it can resolve against the work
            // tree; a `./`-prefixed relative path (as produced by walking `.`)
            // confuses it, so canonicalise first, falling back to the raw path.
            let is_ignored = |p: &Path| {
                let resolved = std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
                repo.is_path_ignored(&resolved).unwrap_or(false)
            };
            return count_tree(roots, &is_ignored);
        }
    }
    count_tree(roots, &|_: &Path| false)
}

// Comment-syntax building blocks, shared between the many languages that use
// the same conventions.
const C_LINE: &[&str] = &["//"];
const C_BLOCK: &[(&str, &str)] = &[("/*", "*/")];
const HASH_LINE: &[&str] = &["#"];
const NO_BLOCK: &[(&str, &str)] = &[];

macro_rules! languages {
    ($( $konst:ident = ($name:literal, $line:expr, $block:expr); )*) => {
        $(
            static $konst: Language = Language {
                name: $name,
                line_comments: $line,
                block_comments: $block,
            };
        )*
    };
}

languages! {
    RUST       = ("Rust",         C_LINE,               &[("/*", "*/")]);
    C          = ("C",            C_LINE,               C_BLOCK);
    ODIN       = ("Odin",         C_LINE,               C_BLOCK);
    CPP        = ("C++",          C_LINE,               C_BLOCK);
    CSHARP     = ("C#",           C_LINE,               C_BLOCK);
    JAVA       = ("Java",         C_LINE,               C_BLOCK);
    KOTLIN     = ("Kotlin",       C_LINE,               C_BLOCK);
    SCALA      = ("Scala",        C_LINE,               C_BLOCK);
    SWIFT      = ("Swift",        C_LINE,               C_BLOCK);
    GO         = ("Go",           C_LINE,               C_BLOCK);
    JAVASCRIPT = ("JavaScript",   C_LINE,               C_BLOCK);
    TYPESCRIPT = ("TypeScript",   C_LINE,               C_BLOCK);
    JSX        = ("JSX",          C_LINE,               C_BLOCK);
    TSX        = ("TSX",          C_LINE,               C_BLOCK);
    DART       = ("Dart",         C_LINE,               C_BLOCK);
    ZIG        = ("Zig",          C_LINE,               NO_BLOCK);
    OBJC       = ("Objective-C",  C_LINE,               C_BLOCK);
    PHP        = ("PHP",          &["//", "#"],         C_BLOCK);
    CSS        = ("CSS",          NO_LINE,              C_BLOCK);
    SCSS       = ("SCSS",         C_LINE,               C_BLOCK);
    GLSL       = ("GLSL",         C_LINE,               C_BLOCK);
    PYTHON     = ("Python",       HASH_LINE,            &[("\"\"\"", "\"\"\""), ("'''", "'''")]);
    RUBY       = ("Ruby",         HASH_LINE,            &[("=begin", "=end")]);
    PERL       = ("Perl",         HASH_LINE,            &[("=pod", "=cut")]);
    SHELL      = ("Shell",        HASH_LINE,            NO_BLOCK);
    FISH       = ("Fish",         HASH_LINE,            NO_BLOCK);
    POWERSHELL = ("PowerShell",   HASH_LINE,            &[("<#", "#>")]);
    LUA        = ("Lua",          &["--"],              &[("--[[", "]]")]);
    HASKELL    = ("Haskell",      &["--"],              &[("{-", "-}")]);
    ELM        = ("Elm",          &["--"],              &[("{-", "-}")]);
    SQL        = ("SQL",          &["--"],              C_BLOCK);
    NIX        = ("Nix",          HASH_LINE,            C_BLOCK);
    TOML       = ("TOML",         HASH_LINE,            NO_BLOCK);
    YAML       = ("YAML",         HASH_LINE,            NO_BLOCK);
    JSON       = ("JSON",         NO_LINE,              NO_BLOCK);
    MARKDOWN   = ("Markdown",     NO_LINE,              NO_BLOCK);
    HTML       = ("HTML",         NO_LINE,              &[("<!--", "-->")]);
    XML        = ("XML",          NO_LINE,              &[("<!--", "-->")]);
    ELIXIR     = ("Elixir",       HASH_LINE,            NO_BLOCK);
    ERLANG     = ("Erlang",       &["%"],               NO_BLOCK);
    CLOJURE    = ("Clojure",      &[";"],               NO_BLOCK);
    LISP       = ("Lisp",         &[";"],               &[("#|", "|#")]);
    SCHEME     = ("Scheme",       &[";"],               &[("#|", "|#")]);
    OCAML      = ("OCaml",        NO_LINE,              &[("(*", "*)")]);
    FSHARP     = ("F#",           C_LINE,               &[("(*", "*)")]);
    VIM        = ("Vim script",   &["\""],              NO_BLOCK);
    MAKE       = ("Makefile",     HASH_LINE,            NO_BLOCK);
    DOCKER     = ("Dockerfile",   HASH_LINE,            NO_BLOCK);
    TEX        = ("TeX",          &["%"],               NO_BLOCK);
    R          = ("R",            HASH_LINE,            NO_BLOCK);
    JULIA      = ("Julia",        HASH_LINE,            &[("#=", "=#")]);
    ASSEMBLY   = ("Assembly",     &[";"],               NO_BLOCK);
    PROTOBUF   = ("Protocol Buffers", C_LINE,           C_BLOCK);
}

const NO_LINE: &[&str] = &[];

/// Look-up from an exact file name to its language.
static BY_FILENAME: Map<&'static str, &'static Language> = phf_map! {
    "Makefile"       => &MAKE,
    "makefile"       => &MAKE,
    "GNUmakefile"    => &MAKE,
    "Dockerfile"     => &DOCKER,
    "Containerfile"  => &DOCKER,
    "Rakefile"       => &RUBY,
    "Gemfile"        => &RUBY,
    "CMakeLists.txt" => &MAKE,
};

/// Look-up from a (lowercase) extension to its language.
static BY_EXTENSION: Map<&'static str, &'static Language> = phf_map! {
    "rs"    => &RUST,
    "c"     => &C,
    "h"     => &C,
    "odin"  => &ODIN,
    "cc"    => &CPP,
    "cpp"   => &CPP,
    "cxx"   => &CPP,
    "hpp"   => &CPP,
    "hh"    => &CPP,
    "cs"    => &CSHARP,
    "java"  => &JAVA,
    "kt"    => &KOTLIN,
    "kts"   => &KOTLIN,
    "scala" => &SCALA,
    "sc"    => &SCALA,
    "swift" => &SWIFT,
    "go"    => &GO,
    "js"    => &JAVASCRIPT,
    "mjs"   => &JAVASCRIPT,
    "cjs"   => &JAVASCRIPT,
    "ts"    => &TYPESCRIPT,
    "jsx"   => &JSX,
    "tsx"   => &TSX,
    "dart"  => &DART,
    "zig"   => &ZIG,
    "m"     => &OBJC,
    "mm"    => &OBJC,
    "php"   => &PHP,
    "css"   => &CSS,
    "scss"  => &SCSS,
    "sass"  => &SCSS,
    "glsl"  => &GLSL,
    "vert"  => &GLSL,
    "frag"  => &GLSL,
    "py"    => &PYTHON,
    "pyw"   => &PYTHON,
    "rb"    => &RUBY,
    "pl"    => &PERL,
    "pm"    => &PERL,
    "sh"    => &SHELL,
    "bash"  => &SHELL,
    "zsh"   => &SHELL,
    "ksh"   => &SHELL,
    "fish"  => &FISH,
    "ps1"   => &POWERSHELL,
    "psm1"  => &POWERSHELL,
    "lua"   => &LUA,
    "hs"    => &HASKELL,
    "elm"   => &ELM,
    "sql"   => &SQL,
    "nix"   => &NIX,
    "toml"  => &TOML,
    "yaml"  => &YAML,
    "yml"   => &YAML,
    "json"  => &JSON,
    "md"    => &MARKDOWN,
    "markdown" => &MARKDOWN,
    "html"  => &HTML,
    "htm"   => &HTML,
    "xml"   => &XML,
    "ex"    => &ELIXIR,
    "exs"   => &ELIXIR,
    "erl"   => &ERLANG,
    "hrl"   => &ERLANG,
    "clj"   => &CLOJURE,
    "cljs"  => &CLOJURE,
    "lisp"  => &LISP,
    "el"    => &LISP,
    "scm"   => &SCHEME,
    "ml"    => &OCAML,
    "mli"   => &OCAML,
    "fs"    => &FSHARP,
    "fsx"   => &FSHARP,
    "vim"   => &VIM,
    "tex"   => &TEX,
    "r"     => &R,
    "jl"    => &JULIA,
    "s"     => &ASSEMBLY,
    "asm"   => &ASSEMBLY,
    "proto" => &PROTOBUF,
};

#[cfg(test)]
mod test {
    use super::*;

    fn count(source: &str, lang: &Language) -> LocCounts {
        LocCounts::from_source(source, lang)
    }

    #[test]
    fn empty_file_is_all_zero() {
        assert_eq!(count("", &RUST), LocCounts::default());
    }

    #[test]
    fn totals_always_add_up() {
        let c = count("fn main() {}\n\n// hi\n", &RUST);
        assert_eq!(c.lines, c.code + c.comments + c.blanks);
    }

    #[test]
    fn counts_code_comments_and_blanks() {
        let source = "fn main() {\n    // a comment\n\n    println!(\"hi\");\n}\n";
        let c = count(source, &RUST);
        assert_eq!(
            c,
            LocCounts {
                lines: 5,
                code: 3,
                comments: 1,
                blanks: 1,
            }
        );
    }

    #[test]
    fn block_comments_span_lines() {
        let source = "code();\n/* start\nstill comment\nend */\nmore();\n";
        let c = count(source, &C);
        assert_eq!(
            c,
            LocCounts {
                lines: 5,
                code: 2,
                comments: 3,
                blanks: 0,
            }
        );
    }

    #[test]
    fn code_then_trailing_comment_is_code() {
        let c = count("let x = 1; // set x\n", &RUST);
        assert_eq!(c.code, 1);
        assert_eq!(c.comments, 0);
    }

    #[test]
    fn comment_token_inside_string_is_code() {
        let c = count("let url = \"https://example.com\";\n", &RUST);
        assert_eq!(c.code, 1);
        assert_eq!(c.comments, 0);
    }

    #[test]
    fn block_open_and_close_on_same_line_after_code() {
        let c = count("do_thing(); /* inline */ do_more();\n", &C);
        assert_eq!(c.code, 1);
        assert_eq!(c.comments, 0);
    }

    #[test]
    fn hash_languages() {
        let c = count("# comment\nname = 1\n", &TOML);
        assert_eq!(c.code, 1);
        assert_eq!(c.comments, 1);
    }

    #[test]
    fn detects_language_by_extension() {
        assert_eq!(language_for("main.rs", Some("rs")), Some(&RUST));
        assert_eq!(language_for("app.py", Some("py")), Some(&PYTHON));
        assert_eq!(language_for("mystery.xyz", Some("xyz")), None);
    }

    #[test]
    fn detects_language_by_filename() {
        assert_eq!(language_for("Makefile", None), Some(&MAKE));
        assert_eq!(language_for("Dockerfile", Some("")), Some(&DOCKER));
    }
}
