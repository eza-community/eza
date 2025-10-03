// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use std::ffi::OsString;
use std::fmt;
use std::num::ParseIntError;

use crate::options::flags;
use crate::options::parser::{Arg, Flag, ParseError};

/// Something wrong with the combination of options the user has picked.
#[derive(PartialEq, Eq, Debug)]
pub enum OptionsError {
    /// There was an error (from `getopts`) parsing the arguments.
    Parse(ParseError),

    /// The user supplied an illegal choice to an Argument.
    BadArgument(&'static Arg, OsString),

    /// The user supplied a set of options that are unsupported
    Unsupported(String),

    /// An option was given twice or more in strict mode.
    Duplicate(Flag, Flag),

    /// Two options were given that conflict with one another.
    Conflict(&'static Arg, &'static Arg),

    /// An option was given that does nothing when another one either is or
    /// isn’t present.
    Useless(&'static Arg, bool, &'static Arg),

    /// An option was given that does nothing when either of two other options
    /// are not present.
    Useless2(&'static Arg, &'static Arg, &'static Arg),

    /// A very specific edge case where --tree can’t be used with --all twice.
    TreeAllAll,

    /// A numeric option was given that failed to be parsed as a number.
    FailedParse(String, NumberSource, ParseIntError),

    /// A glob ignore was given that failed to be parsed as a pattern.
    FailedGlobPattern(String),

    /// A numeric option was given that contained a negative number.
    NegativeNumber(&'static Arg, String),
}

/// The source of a string that failed to be parsed as a number.
#[derive(PartialEq, Eq, Debug)]
pub enum NumberSource {
    /// It came... from a command-line argument!
    Arg(&'static Arg),

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
        use crate::options::parser::TakesValue;

        #[rustfmt::skip]
        return match self {
            Self::BadArgument(arg, attempt) => {
                if let TakesValue::Necessary(Some(values)) = arg.takes_value {
                    write!(
                        f,
                        "Option {} has no {:?} setting ({})",
                        arg,
                        attempt,
                        Choices(values)
                    )
                } else {
                    write!(f, "Option {arg} has no {attempt:?} setting")
                }
            }
            Self::Parse(e)                   => write!(f, "{e}"),
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
            Self::NegativeNumber(a, n)       => write!(f, "Value {n:?} not valid for {a}: Negative numbers are not allowed"),
        };
    }
}

impl OptionsError {
    /// Try to second-guess what the user was trying to do, depending on what
    /// went wrong.
    #[must_use]
    pub fn suggestion(&self) -> Option<&'static str> {
        // ‘ls -lt’ and ‘ls -ltr’ are common combinations
        match self {
            Self::BadArgument(time, r) if *time == &flags::TIME && r == "r" => {
                Some("To sort oldest files last, try \"--sort oldest\", or just \"-sold\"")
            }
            Self::Parse(ParseError::NeedsValue { ref flag, .. }) if *flag == Flag::Short(b't') => {
                Some("To sort newest files last, try \"--sort newest\", or just \"-snew\"")
            }
            _ => None,
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
