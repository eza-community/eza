use ansi_term::Style;
use locale::Numeric as NumericLocale;
use number_prefix::Prefix;

use crate::fs::fields as f;
use crate::output::cell::{TextCell, DisplayWidth};
use crate::output::table::SizeFormat;


impl f::Blocksize {
    pub fn render<C: Colours>(self, colours: &C, size_format: SizeFormat, numerics: &NumericLocale) -> TextCell {
        use number_prefix::NumberPrefix;

        let size = match self {
            Self::Some(s)             => s,
            Self::None                => return TextCell::blank(colours.no_blocksize()),
        };

        let result = match size_format {
            SizeFormat::DecimalBytes  => NumberPrefix::decimal(size as f64),
            SizeFormat::BinaryBytes   => NumberPrefix::binary(size as f64),
            SizeFormat::JustBytes     => {

                // Use the binary prefix to select a style.
                let prefix = match NumberPrefix::binary(size as f64) {
                    NumberPrefix::Standalone(_)   => None,
                    NumberPrefix::Prefixed(p, _)  => Some(p),
                };

                // But format the number directly using the locale.
                let string = numerics.format_int(size);

                return TextCell::paint(colours.blocksize(prefix), string);
            }
        };

        let (prefix, n) = match result {
            NumberPrefix::Standalone(b)   => return TextCell::paint(colours.blocksize(None), numerics.format_int(b)),
            NumberPrefix::Prefixed(p, n)  => (p, n),
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
            ].into(),
        }
    }
}


pub trait Colours {
    fn blocksize(&self, prefix: Option<Prefix>) -> Style;
    fn unit(&self, prefix: Option<Prefix>)      -> Style;
    fn no_blocksize(&self)                      -> Style;
}


#[cfg(test)]
pub mod test {
    use ansi_term::Style;
    use ansi_term::Colour::*;

    use super::Colours;
    use crate::output::cell::TextCell;
    use crate::fs::fields as f;


    struct TestColours;

    impl Colours for TestColours {
        fn block_count(&self) -> Style { Red.blink() }
        fn no_blocks(&self)   -> Style { Green.italic() }
    }


    #[test]
    fn blocklessness() {
        let blox = f::Blocks::None;
        let expected = TextCell::blank(Green.italic());

        assert_eq!(expected, blox.render(&TestColours));
    }


    #[test]
    fn blockfulity() {
        let blox = f::Blocks::Some(3005);
        let expected = TextCell::paint_str(Red.blink(), "3005");

        assert_eq!(expected, blox.render(&TestColours));
    }
}
