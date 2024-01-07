use crate::fs::fields as f;
use crate::fs::fields::MercurialStatus;
use crate::output::{DisplayWidth, TextCell};
use ansiterm::Style;

impl f::Mercurial {
    pub fn render(self, colours: &dyn MercurialColours) -> TextCell {
        let status = match self.status {
            MercurialStatus::Modified => colours.modified().paint("M"),
            MercurialStatus::Added => colours.added().paint("A"),
            MercurialStatus::Removed => colours.removed().paint("R"),
            MercurialStatus::Clean => colours.clean().paint("C"),
            MercurialStatus::Missing => colours.missing().paint("!"),
            MercurialStatus::NotTracked => colours.not_tracked().paint("?"),
            MercurialStatus::Ignored => colours.ignored().paint("I"),
            MercurialStatus::Directory => colours.ignored().paint("-"),
        };

        TextCell {
            width: DisplayWidth::from(1),
            contents: vec![status].into(),
        }
    }
}

pub trait MercurialColours {
    fn modified(&self) -> Style;
    fn added(&self) -> Style;
    fn removed(&self) -> Style;
    fn clean(&self) -> Style;
    fn missing(&self) -> Style;
    fn not_tracked(&self) -> Style;
    fn ignored(&self) -> Style;
}
