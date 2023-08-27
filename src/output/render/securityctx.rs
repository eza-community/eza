use nu_ansi_term::Style;

use crate::fs::fields as f;
use crate::output::cell::{TextCell, DisplayWidth};


impl f::SecurityContext<'_> {
    pub fn render<C: Colors>(&self, colors: &C) -> TextCell {
        match &self.context {
            f::SecurityContextType::None => {
                TextCell::paint_str(colors.none(), "?")
            }
            f::SecurityContextType::SELinux(context) => {
                let mut chars = Vec::with_capacity(7);

                for (i, part) in context.split(':').enumerate() {
                    let part_color = match i {
                        0 => colors.selinux_user(),
                        1 => colors.selinux_role(),
                        2 => colors.selinux_type(),
                        _ => colors.selinux_range()
                    };
                    if i > 0 {
                        chars.push(colors.selinux_colon().paint(":"));
                    }
                    chars.push(part_color.paint(String::from(part)));
                }

                TextCell {
                    contents: chars.into(),
                    width: DisplayWidth::from(context.len())
                }
            }
        }
    }
}

pub trait Colors {
    fn none(&self)          -> Style;
    fn selinux_colon(&self) -> Style;
    fn selinux_user(&self)  -> Style;
    fn selinux_role(&self)  -> Style;
    fn selinux_type(&self)  -> Style;
    fn selinux_range(&self) -> Style;
}
