//! Timestamp formatting.

use chrono::prelude::*;
use clap::ValueEnum;
use core::cmp::max;
use once_cell::sync::Lazy;
use std::time::Duration;
use unicode_width::UnicodeWidthStr;

/// Every timestamp in exa needs to be rendered by a **time format**.
/// Formatting times is tricky, because how a timestamp is rendered can
/// depend on one or more of the following:
///
/// - The user’s locale, for printing the month name as “Feb”, or as “fév”,
///   or as “2月”;
/// - The current year, because certain formats will be less precise when
///   dealing with dates far in the past;
/// - The formatting style that the user asked for on the command-line.
///
/// Because not all formatting styles need the same data, they all have their
/// own enum variants. It’s not worth looking the locale up if the formatter
/// prints month names as numbers.
///
/// Also, eza supports *custom* styles, where the user enters a
/// format string in an environment variable or something. Just these four.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TimeFormat {
    /// The **default format** uses the user’s locale to print month names,
    /// and specifies the timestamp down to the minute for recent times, and
    /// day for older times.
    DefaultFormat,

    /// Use the **ISO format**, which specifies the timestamp down to the
    /// minute for recent times, and day for older times. It uses a number
    /// for the month so it doesn’t use the locale.
    ISOFormat,

    /// Use the **long ISO format**, which specifies the timestamp down to the
    /// minute using only numbers, without needing the locale or year.
    LongISO,

    /// Use the **full ISO format**, which specifies the timestamp down to the
    /// millisecond and includes its offset down to the minute. This too uses
    /// only numbers so doesn’t require any special consideration.
    FullISO,

    /// Use a relative but fixed width representation.
    Relative,

    /// Use custom formats, optionally a different custom format can be
    /// specified for recent times, otherwise the same custom format will be
    /// used for both recent and non-recent times.
    Custom {
        non_recent: Option<String>,
        recent: Option<String>,
    },
}
impl clap::ValueEnum for TimeFormat {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::DefaultFormat => Some(clap::builder::PossibleValue::new("default")),
            Self::ISOFormat => Some(clap::builder::PossibleValue::new("iso")),
            Self::LongISO => Some(clap::builder::PossibleValue::new("long-iso")),
            Self::FullISO => Some(clap::builder::PossibleValue::new("full-iso")),
            Self::Relative => Some(clap::builder::PossibleValue::new("relative")),
            Self::Custom { .. } => Some(clap::builder::PossibleValue::new("custom")),
        }
    }

    fn from_str(s: &str, _ignore_case: bool) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "default" => Ok(Self::DefaultFormat),
            "iso" => Ok(Self::ISOFormat),
            "long-iso" => Ok(Self::LongISO),
            "full-iso" => Ok(Self::FullISO),
            "relative" => Ok(Self::Relative),
            fmt => {
                if fmt.starts_with('+') {
                    let mut lines = fmt.strip_prefix('+').unwrap().lines();

                    // line 1 will be None when:
                    //   - there is nothing after `+`
                    // line 1 will be empty when:
                    //   - `+` is followed immediately by `\n`
                    let empty_non_recent_format_msg = "Custom timestamp format is empty, \
                    please supply a chrono format string after the plus sign.";
                    let non_recent = lines.next().expect(empty_non_recent_format_msg);
                    let non_recent = if non_recent.is_empty() {
                        panic!("{}", empty_non_recent_format_msg)
                    } else {
                        non_recent
                    };

                    // line 2 will be None when:
                    //   - there is not a single `\n`
                    //   - there is nothing after the first `\n`
                    // line 2 will be empty when:
                    //   - there exist at least 2 `\n`, and no content between the 1st and 2nd `\n`
                    let empty_recent_format_msg =
                        "Custom timestamp format for recent files is empty, \
                    please supply a chrono format string at the second line.";
                    let recent = lines.next().map(|rec| {
                        if rec.is_empty() {
                            panic!("{}", empty_recent_format_msg)
                        } else {
                            String::from(rec)
                        }
                    });
                    Ok(TimeFormat::Custom {
                        non_recent: Some(String::from(non_recent)),
                        recent,
                    })
                } else {
                    Err(format!(
                        "Invalid custom timestamp format: {fmt}.\n\
        Please start the format with a plus sign (+) to indicate a custom format.\n\
        For example: +\"%Y-%m-%d %H:%M:%S\"",
                    ))
                }
            }
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::DefaultFormat,
            Self::ISOFormat,
            Self::LongISO,
            Self::FullISO,
            Self::Relative,
            Self::Custom {
                recent: None,
                non_recent: None,
            },
        ]
    }
}
impl TimeFormat {
    pub fn as_str(&self) -> &str {
        match self {
            Self::DefaultFormat => "default",
            Self::ISOFormat => "iso",
            Self::LongISO => "long-iso",
            Self::FullISO => "full-iso",
            Self::Relative => "relative",
            Self::Custom { .. } => "custom",
        }
    }
}

impl From<clap::builder::OsStr> for TimeFormat {
    fn from(s: clap::builder::OsStr) -> Self {
        TimeFormat::from_str(s.to_str().unwrap(), false).unwrap()
    }
}

impl From<TimeFormat> for clap::builder::OsStr {
    fn from(value: TimeFormat) -> Self {
        match value {
            TimeFormat::DefaultFormat => clap::builder::OsStr::from("default"),
            TimeFormat::ISOFormat => clap::builder::OsStr::from("iso"),
            TimeFormat::LongISO => clap::builder::OsStr::from("long-iso"),
            TimeFormat::FullISO => clap::builder::OsStr::from("full-iso"),
            TimeFormat::Relative => clap::builder::OsStr::from("relative"),
            TimeFormat::Custom { .. } => clap::builder::OsStr::from("custom"),
        }
    }
}

impl TimeFormat {
    pub fn format(self, time: &DateTime<FixedOffset>) -> String {
        #[rustfmt::skip]
        return match self {
            Self::DefaultFormat                 => default(time),
            Self::ISOFormat                     => iso(time),
            Self::LongISO                       => long(time),
            Self::FullISO                       => full(time),
            Self::Relative                      => relative(time),
            Self::Custom{non_recent, recent}    => custom(
                time, non_recent.unwrap_or(String::new()).as_str(), recent.as_deref()
            ),
        };
    }
}

fn default(time: &DateTime<FixedOffset>) -> String {
    let month = &*LOCALE.short_month_name(time.month0() as usize);
    let month_width = short_month_padding(*MAX_MONTH_WIDTH, month);
    let format = if time.year() == *CURRENT_YEAR {
        format!("%_d {month:<month_width$} %H:%M")
    } else {
        format!("%_d {month:<month_width$}  %Y")
    };
    time.format(format.as_str()).to_string()
}

/// Convert between Unicode width and width in chars to use in format!.
/// ex: in Japanese, 月 is one character, but it has the width of two.
/// For alignment purposes, we take the real display width into account.
/// So, `MAXIMUM_MONTH_WIDTH` (“12月”) = 4, but if we use `{:4}` in format!,
/// it will add a space (“ 12月”) because format! counts characters.
/// Conversely, a char can have a width of zero (like combining diacritics)
fn short_month_padding(max_month_width: usize, month: &str) -> usize {
    let shift = month.chars().count() as isize - UnicodeWidthStr::width(month) as isize;
    (max_month_width as isize + shift) as usize
}

fn iso(time: &DateTime<FixedOffset>) -> String {
    if time.year() == *CURRENT_YEAR {
        time.format("%m-%d %H:%M").to_string()
    } else {
        time.format("%Y-%m-%d").to_string()
    }
}

fn long(time: &DateTime<FixedOffset>) -> String {
    time.format("%Y-%m-%d %H:%M").to_string()
}

// #[allow(trivial_numeric_casts)]
fn relative(time: &DateTime<FixedOffset>) -> String {
    timeago::Formatter::new()
        .ago("")
        .convert(Duration::from_secs(
            max(0, Local::now().timestamp() - time.timestamp())
                // this .unwrap is safe since the call above can never result in a
                // value < 0
                .try_into()
                .unwrap(),
        ))
}

fn full(time: &DateTime<FixedOffset>) -> String {
    time.format("%Y-%m-%d %H:%M:%S.%f %z").to_string()
}

fn custom(time: &DateTime<FixedOffset>, non_recent_fmt: &str, recent_fmt: Option<&str>) -> String {
    if let Some(recent_fmt) = recent_fmt {
        if time.year() == *CURRENT_YEAR {
            time.format(recent_fmt).to_string()
        } else {
            time.format(non_recent_fmt).to_string()
        }
    } else {
        time.format(non_recent_fmt).to_string()
    }
}

static CURRENT_YEAR: Lazy<i32> = Lazy::new(|| Local::now().year());

static LOCALE: Lazy<locale::Time> =
    Lazy::new(|| locale::Time::load_user_locale().unwrap_or_else(|_| locale::Time::english()));

static MAX_MONTH_WIDTH: Lazy<usize> = Lazy::new(|| {
    // Some locales use a three-character wide month name (Jan to Dec);
    // others vary between three to four (1月 to 12月, juil.). We check each month width
    // to detect the longest and set the output format accordingly.
    (0..11)
        .map(|i| UnicodeWidthStr::width(&*LOCALE.short_month_name(i)))
        .max()
        .unwrap()
});

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn short_month_width_japanese() {
        let max_month_width = 4;
        let month = "1\u{2F49}"; // 1月
        let padding = short_month_padding(max_month_width, month);
        let final_str = format!("{:<width$}", month, width = padding);
        assert_eq!(max_month_width, UnicodeWidthStr::width(final_str.as_str()));
    }

    #[test]
    fn short_month_width_hindi() {
        let max_month_width = 4;
        assert!([
            "\u{091C}\u{0928}\u{0970}",                         // जन॰
            "\u{092B}\u{093C}\u{0930}\u{0970}",                 // फ़र॰
            "\u{092E}\u{093E}\u{0930}\u{094D}\u{091A}",         // मार्च
            "\u{0905}\u{092A}\u{094D}\u{0930}\u{0948}\u{0932}", // अप्रैल
            "\u{092E}\u{0908}",                                 // मई
            "\u{091C}\u{0942}\u{0928}",                         // जून
            "\u{091C}\u{0941}\u{0932}\u{0970}",                 // जुल॰
            "\u{0905}\u{0917}\u{0970}",                         // अग॰
            "\u{0938}\u{093F}\u{0924}\u{0970}",                 // सित॰
            "\u{0905}\u{0915}\u{094D}\u{0924}\u{0942}\u{0970}", // अक्तू॰
            "\u{0928}\u{0935}\u{0970}",                         // नव॰
            "\u{0926}\u{093F}\u{0938}\u{0970}",                 // दिस॰
        ]
        .iter()
        .map(|month| format!(
            "{:<width$}",
            month,
            width = short_month_padding(max_month_width, month)
        ))
        .all(|string| UnicodeWidthStr::width(string.as_str()) == max_month_width));
    }
}
