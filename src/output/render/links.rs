use ansiterm::Style;
#[cfg(unix)]
use locale::Numeric as NumericLocale;

#[cfg(unix)]
use crate::fs::fields as f;
#[cfg(unix)]
use crate::output::cell::TextCell;

#[cfg(unix)]
impl f::Links {
    pub fn render<C: Colours>(&self, colours: &C, numeric: &NumericLocale) -> TextCell {
        let style = if self.multiple { colours.multi_link_file() }
                                else { colours.normal() };

        TextCell::paint(style, numeric.format_int(self.count))
    }
}


pub trait Colours {
    fn normal(&self) -> Style;
    fn multi_link_file(&self) -> Style;
}


#[cfg(test)]
pub mod test {
    use super::Colours;
    #[cfg(unix)]   
    use crate::output::cell::{TextCell, DisplayWidth};
    #[cfg(unix)]
    use crate::fs::fields as f;

    use ansiterm::Colour::*;
    use ansiterm::Style;
    #[cfg(unix)]
    use locale;


    struct TestColours;

    impl Colours for TestColours {
        fn normal(&self)           -> Style { Blue.normal() }
        fn multi_link_file(&self)  -> Style { Blue.on(Red) }
    }


    #[test]
    #[cfg(unix)]
    fn regular_file() {
        let stati = f::Links {
            count:    1,
            multiple: false,
        };

        let expected = TextCell {
            width: DisplayWidth::from(1),
            contents: vec![ Blue.paint("1") ].into(),
        };

        assert_eq!(expected, stati.render(&TestColours, &locale::Numeric::english()));
    }

    #[test]
    #[cfg(unix)]
    fn regular_directory() {
        let stati = f::Links {
            count:    3005,
            multiple: false,
        };

        let expected = TextCell {
            width: DisplayWidth::from(5),
            contents: vec![ Blue.paint("3,005") ].into(),
        };

        assert_eq!(expected, stati.render(&TestColours, &locale::Numeric::english()));
    }

    #[test]
    #[cfg(unix)]
    fn popular_file() {
        let stati = f::Links {
            count:    3005,
            multiple: true,
        };

        let expected = TextCell {
            width: DisplayWidth::from(5),
            contents: vec![ Blue.on(Red).paint("3,005") ].into(),
        };

        assert_eq!(expected, stati.render(&TestColours, &locale::Numeric::english()));
    }
}
