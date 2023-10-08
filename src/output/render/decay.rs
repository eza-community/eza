use ansiterm::{Colour, Style};
use chrono::{Months, NaiveDateTime, Utc};
use palette::{FromColor, Lab, Srgb};

use crate::{fs::File, output::table::TimeType};

#[derive(Debug, Copy, Clone, Default)]
pub struct RelativeTime {
    pub newest: NaiveDateTime,
    pub oldest: NaiveDateTime,
}

impl From<(NaiveDateTime, NaiveDateTime)> for RelativeTime {
    fn from((newest, oldest): (NaiveDateTime, NaiveDateTime)) -> Self {
        Self { newest, oldest }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RelativeDecay {
    pub accessed: Option<RelativeTime>,
    pub changed: Option<RelativeTime>,
    pub created: Option<RelativeTime>,
    pub modified: Option<RelativeTime>,
}

impl RelativeDecay {
    /// Returns [RelativeDecay] decay times for absolute mode
    pub fn absolute() -> Self {
        let newest = Utc::now().naive_utc();
        let oldest = newest
            .checked_sub_months(Months::new(12))
            .unwrap_or(NaiveDateTime::UNIX_EPOCH);
        let reltime = RelativeTime { newest, oldest };

        Self {
            accessed: Some(reltime),
            changed: Some(reltime),
            created: Some(reltime),
            modified: Some(reltime),
        }
    }

    /// Returns [RelativeDecay] containing newest and oldest file modified dates
    /// from a slice of [File] for use in relative mode
    pub fn new(files: &[File<'_>]) -> Self {
        let mut rel_decay = Self {
            accessed: None,
            changed: None,
            created: None,
            modified: None,
        };

        for file in files.iter() {
            let created_time = file.created_time();
            let modified_time = file.modified_time();
            let accessed_time = file.accessed_time();
            let changed_time = file.changed_time();

            match (created_time, &mut rel_decay.created) {
                (Some(t), Some(rel)) => {
                    if t > rel.newest {
                        rel.newest = t
                    } else if t < rel.oldest {
                        rel.oldest = t
                    };
                }
                (Some(t), rel) => {
                    let _ = rel.insert(RelativeTime::from((t, t)));
                }
                _ => (),
            }

            match (modified_time, &mut rel_decay.modified) {
                (Some(t), Some(rel)) => {
                    if t > rel.newest {
                        rel.newest = t
                    } else if t < rel.oldest {
                        rel.oldest = t
                    };
                }
                (Some(t), rel) => {
                    let _ = rel.insert(RelativeTime::from((t, t)));
                }
                _ => (),
            }

            match (accessed_time, &mut rel_decay.accessed) {
                (Some(t), Some(rel)) => {
                    if t > rel.newest {
                        rel.newest = t
                    } else if t < rel.oldest {
                        rel.oldest = t
                    };
                }
                (Some(t), rel) => {
                    let _ = rel.insert(RelativeTime::from((t, t)));
                }
                _ => (),
            }

            match (changed_time, &mut rel_decay.changed) {
                (Some(t), Some(rel)) => {
                    if t > rel.newest {
                        rel.newest = t
                    } else if t < rel.oldest {
                        rel.oldest = t
                    };
                }
                (Some(t), rel) => {
                    let _ = rel.insert(RelativeTime::from((t, t)));
                }
                _ => (),
            }
        }

        rel_decay
    }

    fn luminance_from_relative_time(relative_time: f32) -> f32 {
        (0.2 + 0.8 * (-5.0 * (1.0 - relative_time)).exp()) * 100.0
    }

    fn adjust_luminance(
        &self,
        relative_time: RelativeTime,
        file_time: NaiveDateTime,
        color: Colour,
    ) -> Colour {
        let relative_time = if relative_time.newest == relative_time.oldest {
            1.0
        } else {
            let time = file_time.timestamp_millis() as f32;
            let max = relative_time.newest.timestamp_millis() as f32;
            let min = relative_time.oldest.timestamp_millis() as f32;
            ((time - min) / (max - min)).clamp(0.0, 1.0)
        };

        let color = Srgb::from_components(color.into_rgb()).into_linear();

        let mut lab: Lab = Lab::from_color(color);
        lab.l = Self::luminance_from_relative_time(relative_time);

        let adjusted_rgb: Srgb<f32> = Srgb::from_color(lab);
        Colour::RGB(
            (adjusted_rgb.red * 255.0).round() as u8,
            (adjusted_rgb.green * 255.0).round() as u8,
            (adjusted_rgb.blue * 255.0).round() as u8,
        )
    }

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
            style.foreground = Some(self.adjust_luminance(rel_time, file_time, fg));
        }

        style
    }
}
