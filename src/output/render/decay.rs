use ansiterm::{Colour, Style};
use chrono::{Months, NaiveDateTime, Utc};
use palette::{FromColor, Oklab, Srgb};

use crate::{fs::File, output::table::TimeType};

#[derive(Debug, Copy, Clone, Default)]
pub struct FileModificationRange {
    pub newest: NaiveDateTime,
    pub oldest: NaiveDateTime,
}

fn luminance_from_relative_time(relative_time: f32) -> f32 {
    1.0 - 0.4 + 0.6 * (-7.0 * (1.0 - relative_time)).exp()
}

/// Update the `range` based on the given `time` value:
/// - If `time` is greater than `range.newest`, update `range.newest` to `time`.
/// - If `time` is less than `range.oldest`, update `range.oldest` to `time`.
/// - If `time` has a value and `range` doesn't, initialize `range` with {newest: time, oldest: time}.
fn update_range(
    maybe_time: Option<NaiveDateTime>,
    maybe_range: &mut Option<FileModificationRange>,
) {
    match (maybe_time, maybe_range) {
        (Some(time), Some(range)) => {
            if time > range.newest {
                range.newest = time
            } else if time < range.oldest {
                range.oldest = time
            };
        }
        (Some(t), rel) => {
            let _ = rel.insert({
                let (newest, oldest) = (t, t);
                FileModificationRange { newest, oldest }
            });
        }
        _ => (),
    };
}

#[derive(Debug, Copy, Clone)]
pub struct FileTimeRanges {
    pub accessed: Option<FileModificationRange>,
    pub changed: Option<FileModificationRange>,
    pub created: Option<FileModificationRange>,
    pub modified: Option<FileModificationRange>,
}

impl FileTimeRanges {
    pub fn absolute() -> Self {
        let newest = Utc::now().naive_utc();

        let oldest = newest
            .checked_sub_months(Months::new(12))
            .unwrap_or(NaiveDateTime::UNIX_EPOCH); // current_time - 12_months

        let reltime = FileModificationRange { newest, oldest };
        Self {
            accessed: Some(reltime),
            changed: Some(reltime),
            created: Some(reltime),
            modified: Some(reltime),
        }
    }

    /// Gets the oldest and newest timestamps from `&[File]`
    pub fn from_files(files: &[File<'_>]) -> Self {
        let mut time_ranges = Self {
            accessed: None,
            changed: None,
            created: None,
            modified: None,
        };

        for file in files.iter() {
            update_range(file.created_time(), &mut time_ranges.created);
            update_range(file.modified_time(), &mut time_ranges.modified);
            update_range(file.accessed_time(), &mut time_ranges.accessed);
            update_range(file.changed_time(), &mut time_ranges.changed);
        }

        time_ranges
    }

    /// Adjust the luminance for a given colour
    fn adjust_luminance(&self, color: Colour, luminance: f32) -> Colour {
        let color = Srgb::from_components(color.into_rgb()).into_linear();

        let mut lab: Oklab = Oklab::from_color(color);
        lab.l = luminance;

        let adjusted_rgb: Srgb<f32> = Srgb::from_color(lab);
        Colour::RGB(
            (adjusted_rgb.red * 255.0).round() as u8,
            (adjusted_rgb.green * 255.0).round() as u8,
            (adjusted_rgb.blue * 255.0).round() as u8,
        )
    }

    /// Returns the style after decay-related styles are added
    pub fn get_adjusted_style(
        &self,
        mut style: Style,
        file: &File<'_>,
        time_type: TimeType,
    ) -> Style {
        let maybe_rel_time = match time_type {
            TimeType::Modified => self.modified,
            TimeType::Changed => self.changed,
            TimeType::Accessed => self.accessed,
            TimeType::Created => self.created,
        };
        if let (Some(fg), Some(file_time), Some(rel_time)) = (
            style.foreground,
            time_type.get_corresponding_time(file),
            maybe_rel_time,
        ) {
            let file_time = file_time.timestamp_millis() as f32;
            let newest = rel_time.newest.timestamp_millis() as f32;
            let oldest = rel_time.oldest.timestamp_millis() as f32;

            let mut ratio = ((file_time - oldest) / (newest - oldest)).clamp(0.0, 1.0);
            if ratio.is_nan() {
                ratio = 1.0;
            }

            let luminance = luminance_from_relative_time(ratio);
            style.foreground = Some(self.adjust_luminance(fg, luminance));
        }

        style
    }
}
