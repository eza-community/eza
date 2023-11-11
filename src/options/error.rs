use std::ffi::OsString;
use std::fmt;
use std::num::ParseIntError;

/// Something wrong with the combination of options the user has picked.
#[derive(PartialEq, Eq, Debug)]
pub enum OptionsError {
    /// The user supplied an illegal choice to an Argument.
    BadArgument(&'static str, OsString),

    /// The user supplied a set of options that are unsupported
    Unsupported(String),

    /// An option was given twice or more in strict mode.
    Duplicate(&'static str, &'static str),

    /// Two options were given that conflict with one another.
    Conflict(&'static str, &'static str),

    /// An option was given that does nothing when another one either is or
    /// isn’t present.
    Useless(&'static str, bool, &'static str),

    /// An option was given that does nothing when either of two other options
    /// are not present.
    Useless2(&'static str, &'static str, &'static str),

    /// A very specific edge case where --tree can’t be used with --all twice.
    TreeAllAll,

    /// A numeric option was given that failed to be parsed as a number.
    FailedParse(String, NumberSource, ParseIntError),

    /// A glob ignore was given that failed to be parsed as a pattern.
    FailedGlobPattern(String),
}

/// The source of a string that failed to be parsed as a number.
#[derive(PartialEq, Eq, Debug)]
pub enum NumberSource {
    /// It came... from a command-line argument!
    Arg(&'static str),

    /// It came... from the environment!
    Env(&'static str),
}

impl From<glob::PatternError> for OptionsError {
    fn from(error: glob::PatternError) -> Self {
        Self::FailedGlobPattern(error.to_string())
    }
}

impl fmt::Display for NumberSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Arg(arg) => write!(f, "option {arg}"),
            Self::Env(env) => write!(f, "environment variable {env}"),
        }
    }
}

impl fmt::Display for OptionsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[rustfmt::skip]
        return match self {
            Self::BadArgument(arg, _) => write!(f, "Bad argument for {arg}"),
            Self::Unsupported(e)             => write!(f, "{e}"),
            Self::Conflict(a, b)             => write!(f, "Option {a} conflicts with option {b}"),
            Self::Duplicate(a, b) if a == b  => write!(f, "Flag {a} was given twice"),
            Self::Duplicate(a, b)            => write!(f, "Flag {a} conflicts with flag {b}"),
            Self::Useless(a, false, b)       => write!(f, "Option {a} is useless without option {b}"),
            Self::Useless(a, true, b)        => write!(f, "Option {a} is useless given option {b}"),
            Self::Useless2(a, b1, b2)        => write!(f, "Option {a} is useless without options {b1} or {b2}"),
            Self::TreeAllAll                 => write!(f, "Option --tree is useless given --all --all"),
            Self::FailedParse(s, n, e)       => write!(f, "Value {s:?} not valid for {n}: {e}"),
            Self::FailedGlobPattern(ref e)   => write!(f, "Failed to parse glob pattern: {e}"),
        };
    }
}

/// A list of legal choices for an argument-taking option.
#[derive(PartialEq, Eq, Debug)]
pub struct Choices(pub &'static [&'static str]);

impl fmt::Display for Choices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "choices: {}", self.0.join(", "))
    }
}
