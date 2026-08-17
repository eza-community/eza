// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use log::trace;
use nu_ansi_term::{Color as Colour, Style};
use palette::{FromColor, LinSrgb, Oklab, Srgb};

use crate::{
    fs::{
        File, dir_action::RecurseOptions, feature::git::GitCache, fields::Size, filter::FileFilter,
    },
    output::{table::TimeType, tree::TreeDepth},
};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct ColorScaleOptions {
    pub mode: ColorScaleMode,
    pub min_luminance: isize,
    pub size: bool,
    pub age: bool,
}

impl Default for ColorScaleOptions {
    fn default() -> Self {
        Self {
            mode: ColorScaleMode::Fixed,
            min_luminance: 50,
            size: false,
            age: false,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum ColorScaleMode {
    Fixed,
    Gradient,
}

#[derive(Copy, Clone, Debug)]
pub struct ColorScaleInformation {
    pub options: ColorScaleOptions,

    pub accessed: Option<Extremes>,
    pub changed: Option<Extremes>,
    pub created: Option<Extremes>,
    pub modified: Option<Extremes>,

    pub size: Option<Extremes>,
}

impl ColorScaleInformation {
    pub fn from_color_scale(
        color_scale: ColorScaleOptions,
        files: &[File<'_>],
        filter: &FileFilter,
        git: Option<&GitCache>,
        git_ignoring: bool,
        r: Option<RecurseOptions>,
    ) -> Option<Self> {
        if color_scale.mode == ColorScaleMode::Fixed {
            None
        } else {
            let mut information = Self {
                options: color_scale,
                accessed: None,
                changed: None,
                created: None,
                modified: None,
                size: None,
            };

            update_information_recursively(
                &mut information,
                files,
                filter,
                git,
                git_ignoring,
                TreeDepth::root(),
                r,
            );

            Some(information)
        }
    }

    #[must_use]
    pub fn adjust_style(&self, mut style: Style, value: f32, range: Option<Extremes>) -> Style {
        if let (Some(fg), Some(range)) = (style.foreground, range) {
            let mut ratio = ((value - range.min) / (range.max - range.min)).clamp(0.0, 1.0);
            if ratio.is_nan() {
                ratio = 1.0;
            }

            style.foreground = Some(adjust_luminance(
                fg,
                ratio,
                self.options.min_luminance as f32 / 100.0,
            ));
        }

        style
    }

    pub fn apply_time_gradient(&self, style: Style, file: &File<'_>, time_type: TimeType) -> Style {
        let range = match time_type {
            TimeType::Modified => self.modified,
            TimeType::Changed => self.changed,
            TimeType::Accessed => self.accessed,
            TimeType::Created => self.created,
        };

        if let Some(file_time) = time_type.get_corresponding_time(file) {
            self.adjust_style(style, file_time.and_utc().timestamp_millis() as f32, range)
        } else {
            style
        }
    }
}

fn update_information_recursively(
    information: &mut ColorScaleInformation,
    files: &[File<'_>],
    filter: &FileFilter,
    git: Option<&GitCache>,
    git_ignoring: bool,
    depth: TreeDepth,
    r: Option<RecurseOptions>,
) {
    for file in files {
        if information.options.age {
            Extremes::update(
                file.created_time()
                    .map(|x| x.and_utc().timestamp_millis() as f32),
                &mut information.created,
            );
            Extremes::update(
                file.modified_time()
                    .map(|x| x.and_utc().timestamp_millis() as f32),
                &mut information.modified,
            );
            Extremes::update(
                file.accessed_time()
                    .map(|x| x.and_utc().timestamp_millis() as f32),
                &mut information.accessed,
            );
            Extremes::update(
                file.changed_time()
                    .map(|x| x.and_utc().timestamp_millis() as f32),
                &mut information.changed,
            );
        }

        if information.options.size {
            let size = match file.size() {
                Size::Some(size) => Some(size as f32),
                _ => None,
            };
            Extremes::update(size, &mut information.size);
        }

        // We don't want to recurse into . and .., but still want to list them, therefore bypass
        // the dot_filter.
        if file.is_directory()
            && r.is_some_and(|x| !x.is_too_deep(depth.0))
            && file.name != "."
            && file.name != ".."
        {
            match file.read_dir() {
                Ok(dir) => {
                    let mut files: Vec<File<'_>> = dir
                        .files(filter.dot_filter, git, git_ignoring, false, false)
                        .collect();

                    // Files that will never be displayed must not contribute to
                    // the gradient extremes, or an ignored outlier would skew
                    // the colours of every row that *is* shown.
                    filter.filter_child_files(r.is_some(), &mut files);

                    update_information_recursively(
                        information,
                        &files,
                        filter,
                        git,
                        git_ignoring,
                        depth.deeper(),
                        r,
                    );
                }
                Err(e) => trace!("Unable to access directory {}: {}", file.name, e),
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Extremes {
    max: f32,
    min: f32,
}

impl Extremes {
    fn update(maybe_value: Option<f32>, maybe_range: &mut Option<Extremes>) {
        match (maybe_value, maybe_range) {
            (Some(value), Some(range)) => {
                if value > range.max {
                    range.max = value;
                } else if value < range.min {
                    range.min = value;
                }
            }
            (Some(value), rel) => {
                let _ = rel.insert({
                    Extremes {
                        max: value,
                        min: value,
                    }
                });
            }
            _ => (),
        }
    }
}

fn adjust_luminance(color: Colour, x: f32, min_l: f32) -> Colour {
    let rgb_color = match color {
        Colour::Rgb(r, g, b) => LinSrgb::new(
            f32::from(r) / 255.0,
            f32::from(g) / 255.0,
            f32::from(b) / 255.0,
        ),

        Colour::Black => LinSrgb::new(0.0, 0.0, 0.0),

        Colour::Green | Colour::LightGreen => LinSrgb::new(0.0, 1.0, 0.0),

        Colour::Yellow | Colour::LightYellow => LinSrgb::new(1.0, 1.0, 0.0),

        Colour::Blue | Colour::LightBlue => LinSrgb::new(0.0, 0.0, 1.0),

        Colour::Magenta | Colour::LightMagenta => LinSrgb::new(1.0, 0.0, 1.0),

        Colour::Cyan | Colour::LightCyan => LinSrgb::new(0.0, 1.0, 1.0),

        Colour::White => LinSrgb::new(1.0, 1.0, 1.0),

        Colour::LightGray => LinSrgb::new(0.5, 0.5, 0.5),

        Colour::LightRed | Colour::Red => LinSrgb::new(1.0, 0.0, 0.0),

        Colour::DarkGray => LinSrgb::new(0.25, 0.25, 0.25),

        Colour::LightPurple | Colour::Purple => LinSrgb::new(0.5, 0.0, 0.5),

        _ => LinSrgb::new(1.0, 1.0, 1.0),
    };

    let mut lab: Oklab = Oklab::from_color(rgb_color);
    lab.l = (min_l + (1.0 - min_l) * (-4.0 * (1.0 - x)).exp()).clamp(0.0, 1.0);

    let adjusted_rgb: Srgb<f32> = Srgb::from_color(lab);
    Colour::Rgb(
        (adjusted_rgb.red * 255.0).round() as u8,
        (adjusted_rgb.green * 255.0).round() as u8,
        (adjusted_rgb.blue * 255.0).round() as u8,
    )
}

#[cfg(test)]
mod tests {
    use super::{ColorScaleInformation, ColorScaleMode, ColorScaleOptions};
    use crate::fs::filter::{FileFilter, GitIgnore, IgnorePatterns};
    use crate::fs::{DotFilter, File, dir_action::RecurseOptions};
    use std::path::Path;
    use std::time::{Duration, SystemTime};

    fn touch(path: &Path, secs_since_epoch: u64) {
        let f = std::fs::File::create(path).unwrap();
        f.set_modified(SystemTime::UNIX_EPOCH + Duration::from_secs(secs_since_epoch))
            .unwrap();
    }

    fn filter_ignoring(patterns: Vec<&str>) -> FileFilter {
        let (ignore_patterns, errors) = IgnorePatterns::parse_from_iter(patterns);
        assert!(errors.is_empty());

        FileFilter {
            flags: Vec::new(),
            sort_field: crate::fs::filter::SortField::default(),
            dot_filter: DotFilter::JustFiles,
            ignore_patterns,
            git_ignore: GitIgnore::Off,
            no_symlinks: false,
            show_symlinks: false,
        }
    }

    fn modified_range(filter: &FileFilter, dir: &Path) -> (f32, f32) {
        let root = File::from_args(dir.to_path_buf(), None, None, false, false, None);
        let info = ColorScaleInformation::from_color_scale(
            ColorScaleOptions {
                mode: ColorScaleMode::Gradient,
                age: true,
                ..ColorScaleOptions::default()
            },
            &[root],
            filter,
            None,
            false,
            Some(RecurseOptions {
                tree: true,
                max_depth: None,
            }),
        )
        .expect("gradient mode should produce color scale information");

        let modified = info.modified.expect("mtimes should have been collected");
        (modified.min, modified.max)
    }

    /// Files removed by the filter (here: `--ignore-glob`) must not widen the
    /// age gradient, or an invisible outlier changes the colour of every
    /// visible row. Regression test: the walk used to bypass the filter.
    #[test]
    fn ignored_files_do_not_skew_the_age_range() {
        let dir = std::env::temp_dir().join(format!("eza-color-scale-{}", std::process::id()));
        let sub = dir.join("sub");
        std::fs::create_dir_all(&sub).unwrap();

        touch(&sub.join("visible-old.txt"), 1_000_000);
        touch(&sub.join("visible-new.txt"), 3_000_000);
        touch(&sub.join("ignored-ancient.log"), 1);

        let with_ignore = modified_range(&filter_ignoring(vec!["*.log"]), &dir);
        let without_ignore = modified_range(&filter_ignoring(vec![]), &dir);

        std::fs::remove_dir_all(&dir).unwrap();

        // The ignored `.log` is the oldest file, so it only shows up in the
        // unfiltered range.
        assert_eq!(1_000_000_000.0_f32, with_ignore.0);
        assert_eq!(1_000.0_f32, without_ignore.0);
    }
}
