use crate::output::hidden_count::WarnHiddenMode;

use crate::options::parser::MatchedFlags;
use crate::options::{flags, OptionsError};

impl WarnHiddenMode {
    pub fn deduce(matches: &MatchedFlags<'_>) -> Result<Self, OptionsError> {
        match (matches.count(&flags::WARN_HIDDEN), matches.is_strict()) {
            (0, _) => Ok(WarnHiddenMode::Never),
            (1, _) => Ok(WarnHiddenMode::Auto),
            (2, _) | (_, false) => Ok(WarnHiddenMode::Always),
            (_, true) => Err(OptionsError::Conflict(
                &flags::WARN_HIDDEN,
                &flags::WARN_HIDDEN,
            )),
        }
    }
}
