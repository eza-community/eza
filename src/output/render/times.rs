use crate::output::cell::TextCell;
use crate::output::time::TimeFormat;

use ansiterm::Style;
use chrono::prelude::*;

pub trait Render {
    fn render(self, style: Style, time_format: TimeFormat) -> TextCell;
}

impl Render for Option<NaiveDateTime> {
    fn render(self, style: Style, time_format: TimeFormat) -> TextCell {
        let timezone_str = iana_time_zone::get_timezone().unwrap();
        let timezone: chrono_tz::Tz = timezone_str.parse().unwrap();
        let datestamp = if let Some(time) = self {
            let time_offset = timezone.offset_from_utc_datetime(&time).fix();
            time_format.format(&DateTime::<FixedOffset>::from_naive_utc_and_offset(
                time,
                time_offset,
            ))
        } else {
            String::from("-")
        };

        TextCell::paint(style, datestamp)
    }
}
