// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use locale::Numeric as NumericLocale;
use nu_ansi_term::Style;
use number_prefix::Prefix;

use crate::fs::fields as f;
use crate::output::cell::{DisplayWidth, TextCell};
use crate::output::table::{SizeFormat, AllocatedSizeMode};

impl f::AllocatedSizeAvailability {
    pub fn render<C: Colours>(
        self,
        colours: &C,
        allocated_size_mode: AllocatedSizeMode,
        size_format: SizeFormat,
        numerics: &NumericLocale,
    ) -> TextCell {
        use number_prefix::NumberPrefix;

        let allocated_size = match self {
            Self::Some(s) => s,
            Self::None => return TextCell::blank(colours.no_blocksize()),
        };

        // Number of file system blocks instead of size
        if let AllocatedSizeMode::Blocks = allocated_size_mode {
              // Divide the actual file size by the file system block size to
              // get the number of allocated blocks.
              let blocks: u64 = allocated_size.total_size.div_ceil(allocated_size.block_size);
              let string = numerics.format_int(blocks);
              return TextCell::paint(colours.blocksize(None), string);
        }

        let result = match size_format {
            SizeFormat::DecimalBytes => NumberPrefix::decimal(allocated_size.total_size as f64),
            SizeFormat::BinaryBytes => NumberPrefix::binary(allocated_size.total_size as f64),
            SizeFormat::JustBytes => {
                // Use the binary prefix to select a style.
                let prefix = match NumberPrefix::binary(allocated_size.total_size as f64) {
                    NumberPrefix::Standalone(_) => None,
                    NumberPrefix::Prefixed(p, _) => Some(p),
                };

                // But format the number directly using the locale.
                let string = numerics.format_int(allocated_size.total_size);

                return TextCell::paint(colours.blocksize(prefix), string);
            }
        };

        let (prefix, n) = match result {
            NumberPrefix::Standalone(b) => {
                return TextCell::paint(colours.blocksize(None), numerics.format_int(b));
            }
            NumberPrefix::Prefixed(p, n) => (p, n),
        };

        let symbol = prefix.symbol();
        let number = if n < 10_f64 {
            numerics.format_float(n, 1)
        } else {
            numerics.format_int(n.round() as isize)
        };

        TextCell {
            // symbol is guaranteed to be ASCII since unit prefixes are hardcoded.
            width: DisplayWidth::from(&*number) + symbol.len(),
            contents: vec![
                colours.blocksize(Some(prefix)).paint(number),
                colours.unit(Some(prefix)).paint(symbol),
            ]
            .into(),
        }
    }
}

#[rustfmt::skip]
pub trait Colours {
    fn blocksize(&self, prefix: Option<Prefix>) -> Style;
    fn unit(&self, prefix: Option<Prefix>)      -> Style;
    fn no_blocksize(&self)                      -> Style;
}

#[cfg(test)]
pub mod test {
    use nu_ansi_term::Color::*;
    use nu_ansi_term::Style;

    use super::Colours;
    use crate::fs::fields as f;
    use crate::output::cell::{DisplayWidth, TextCell};
    use crate::output::table::{SizeFormat, AllocatedSizeMode};

    use locale::Numeric as NumericLocale;
    use number_prefix::Prefix;

    struct TestColours;

    #[rustfmt::skip]
    impl Colours for TestColours {
        fn blocksize(&self, _prefix: Option<Prefix>) -> Style { Fixed(66).normal() }
        fn unit(&self, _prefix: Option<Prefix>)      -> Style { Fixed(77).bold() }
        fn no_blocksize(&self)                       -> Style { Black.italic() }
    }

    #[test]
    fn directory() {
        let directory = f::AllocatedSizeAvailability::None;
        let expected = TextCell::blank(Black.italic());
        assert_eq!(
            expected,
            directory.render(
                &TestColours,
                AllocatedSizeMode::Bytes,
                SizeFormat::JustBytes,
                &NumericLocale::english()
            )
        );
    }

    #[test]
    fn file_decimal() {
        let directory = f::AllocatedSizeAvailability::Some(f::AllocatedSize {
            total_size: 2_100_000,
            block_size: 4096,
        });
        let expected = TextCell {
            width: DisplayWidth::from(4),
            contents: vec![Fixed(66).paint("2.1"), Fixed(77).bold().paint("M")].into(),
        };

        assert_eq!(
            expected,
            directory.render(
                &TestColours,
                AllocatedSizeMode::Bytes,
                SizeFormat::DecimalBytes,
                &NumericLocale::english()
            )
        );
    }

    #[test]
    fn file_binary() {
        let directory = f::AllocatedSizeAvailability::Some(f::AllocatedSize {
            total_size: 1_048_576,
            block_size: 4096,
        });
        let expected = TextCell {
            width: DisplayWidth::from(5),
            contents: vec![Fixed(66).paint("1.0"), Fixed(77).bold().paint("Mi")].into(),
        };

        assert_eq!(
            expected,
            directory.render(
                &TestColours,
                AllocatedSizeMode::Bytes,
                SizeFormat::BinaryBytes,
                &NumericLocale::english()
            )
        );
    }

    #[test]
    fn file_bytes() {
        let directory = f::AllocatedSizeAvailability::Some(f::AllocatedSize {
            total_size: 1_048_576,
            block_size: 4096,
        });
        let expected = TextCell {
            width: DisplayWidth::from(9),
            contents: vec![Fixed(66).paint("1,048,576")].into(),
        };

        assert_eq!(
            expected,
            directory.render(
                &TestColours,
                AllocatedSizeMode::Bytes,
                SizeFormat::JustBytes,
                &NumericLocale::english()
            )
        );
    }

    #[test]
    fn file_n_blocks() {
        let directory = f::AllocatedSizeAvailability::Some(f::AllocatedSize {
            total_size: 1_048_576,
            block_size: 4096,
        });
        let expected = TextCell {
            width: DisplayWidth::from(3),
            contents: vec![Fixed(66).paint("256")].into(),
        };

        assert_eq!(
            expected,
            directory.render(
                &TestColours,
                AllocatedSizeMode::Blocks,
                SizeFormat::JustBytes,
                &NumericLocale::english()
            )
        );
    }
}
