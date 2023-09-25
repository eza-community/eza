use std::fmt;
use std::num::ParseIntError;

/// Something wrong with the combination of options the user has picked.
#[derive(PartialEq, Eq, Debug)]
pub enum OptionsError {
    /// The user supplied an illegal choice to an Argument.
    BadArgument(String, String),

    /// The user supplied a set of options that are unsupported
    Unsupported(String),

    /// An option was given twice or more in strict mode.
    Duplicate(String, String),

    /// Two options were given that conflict with one another.
    Conflict(String, String),

    /// An option was given that does nothing when another one either is or
    /// isn’t present.
    Useless(String, bool, String),

    /// An option was given that does nothing when either of two other options
    /// are not present.
    Useless2(String, String, String),

    /// A very specific edge case where --tree can’t be used with --all twice.
    TreeAllAll,

    /// A numeric option was given that failed to be parsed as a number.
    FailedParse(String, NumberSource, ParseIntError),

    /// A glob ignore was given that failed to be parsed as a pattern.
    FailedGlobPattern(String),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum NumberSource {
    Var(String),
    Arg(String),
}

impl fmt::Display for NumberSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Var(s) => write!(f, "variable {s}"),
            Self::Arg(s) => write!(f, "argument {s}"),
        }
    }
}

impl From<glob::PatternError> for OptionsError {
    fn from(error: glob::PatternError) -> Self {
        Self::FailedGlobPattern(error.to_string())
    }
}

impl fmt::Display for OptionsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadArgument(arg, attempt) => {
                write!(f, "Argument {arg} doesn’t take {attempt}")
            }
            Self::Unsupported(e) => write!(f, "{e}"),
            Self::Conflict(a, b) => write!(f, "Option {a} conflicts with option {b}"),
            Self::Duplicate(a, b) if a == b => write!(f, "Flag {a} was given twice"),
            Self::Duplicate(a, b) => write!(f, "Flag {a} conflicts with flag {b}"),
            Self::Useless(a, false, b) => write!(f, "Option {a} is useless without option {b}"),
            Self::Useless(a, true, b) => write!(f, "Option {a} is useless given option {b}"),
            Self::Useless2(a, b1, b2) => {
                write!(f, "Option {a} is useless without options {b1} or {b2}")
            }
            Self::TreeAllAll => write!(f, "Option --tree is useless given --all --all"),
            Self::FailedParse(s, n, e) => write!(f, "Value {s:?} not valid for {n}: {e}"),
            Self::FailedGlobPattern(ref e) => write!(f, "Failed to parse glob pattern: {e}"),
        }
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
