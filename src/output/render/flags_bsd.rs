use ansiterm::Style;
use std::ffi::CStr;

use crate::fs::fields as f;
use crate::output::cell::TextCell;
use crate::output::table::FlagsFormat;

extern "C" {
    fn fflagstostr(flags: libc::c_ulong) -> *const libc::c_char;
}

/// Wrapper around the C library call fflagstostr.  If returned string is NULL
/// or empty a "-" is returned
fn flags_to_string(flags: f::flag_t) -> String {
    // SAFETY: Calling external "C" function
    let flags_c_str = unsafe { fflagstostr(libc::c_ulong::from(flags)) };

    if flags_c_str.is_null() {
        "-".to_string()
    } else {
        let flags_str = unsafe { CStr::from_ptr(flags_c_str) };
        let flags = flags_str
            .to_str()
            .map_or("-", |s| if s.is_empty() { "-" } else { s })
            .to_string();

        // SAFETY: Calling external "C" function to free memory allocated by fflagstostr
        unsafe {
            libc::free(flags_c_str.cast_mut().cast());
        }

        flags
    }
}

impl f::Flags {
    pub fn render(self, style: Style, _format: FlagsFormat) -> TextCell {
        TextCell::paint(style, flags_to_string(self.0))
    }
}
