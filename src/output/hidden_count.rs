use ansiterm::Colour;

#[derive(Debug)]
pub struct HiddenCount {
    /// Whether to show all counts regardless of how many hidden and/or ignored items there are
    always_print: bool,
    hidden: usize,
    ignored: usize,
}

impl HiddenCount {
    pub fn new(mode: WarnHiddenMode) -> Option<HiddenCount> {
        let always_print = match mode {
            WarnHiddenMode::Never => return None,
            WarnHiddenMode::Auto => false,
            WarnHiddenMode::Always => true,
        };

        Some(HiddenCount {
            always_print,
            hidden: 0,
            ignored: 0,
        })
    }

    pub fn inc_hidden(&mut self) {
        self.hidden += 1;
    }

    pub fn inc_ignored(&mut self) {
        self.ignored += 1;
    }

    pub fn render(&self) -> Option<String> {
        let warn_string = match (self.always_print, self.hidden, self.ignored) {
            (false, 0, 0) => None,
            (false, hidden, 0) => Some(format!("...and {hidden} hidden items")),
            (false, 0, ignored) => Some(format!("...and {ignored} ignored items")),
            (false, hidden, ignored) => {
                Some(format!("...and {hidden} hidden, {ignored} ignored items"))
            }
            (true, hidden, ignored) => Some(format!("{hidden} hidden and {ignored} ignored items")),
        };
        warn_string.map(|s| Colour::BrightRed.paint(s).to_string())
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum WarnHiddenMode {
    Never,
    Auto,
    Always,
}
