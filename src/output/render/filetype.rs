use nu_ansi_term::{AnsiString, Style};

use crate::fs::fields as f;


impl f::Type {
    pub fn render<C: Colors>(self, colors: &C) -> AnsiString<'static> {
        match self {
            Self::File         => colors.normal().paint("."),
            Self::Directory    => colors.directory().paint("d"),
            Self::Pipe         => colors.pipe().paint("|"),
            Self::Link         => colors.symlink().paint("l"),
            Self::BlockDevice  => colors.block_device().paint("b"),
            Self::CharDevice   => colors.char_device().paint("c"),
            Self::Socket       => colors.socket().paint("s"),
            Self::Special      => colors.special().paint("?"),
        }
    }
}


pub trait Colors {
    fn normal(&self) -> Style;
    fn directory(&self) -> Style;
    fn pipe(&self) -> Style;
    fn symlink(&self) -> Style;
    fn block_device(&self) -> Style;
    fn char_device(&self) -> Style;
    fn socket(&self) -> Style;
    fn special(&self) -> Style;
}
