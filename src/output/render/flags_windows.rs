// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
use crate::fs::fields as f;
use crate::output::table::FlagsFormat;
use crate::output::TextCell;
use nu_ansi_term::Style;

// See https://learn.microsoft.com/en-us/windows/win32/fileio/file-attribute-constants
const FILE_ATTRIBUTE_READONLY: u32 = 0x0000_0001; // R
const FILE_ATTRIBUTE_HIDDEN: u32 = 0x0000_0002; // H
const FILE_ATTRIBUTE_SYSTEM: u32 = 0x0000_0004; // S
const FILE_ATTRIBUTE_ARCHIVE: u32 = 0x0000_0020; // A
const FILE_ATTRIBUTE_TEMPORARY: u32 = 0x0000_0100; // T
const FILE_ATTRIBUTE_COMPRESSED: u32 = 0x0000_0800; // C
const FILE_ATTRIBUTE_OFFLINE: u32 = 0x0000_1000; // O
const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: u32 = 0x0000_2000; // I
const FILE_ATTRIBUTE_ENCRYPTED: u32 = 0x0000_4000; // E
const FILE_ATTRIBUTE_NO_SCRUB_DATA: u32 = 0x0002_0000; // X
const FILE_ATTRIBUTE_PINNED: u32 = 0x0008_0000; // P
const FILE_ATTRIBUTE_UNPINNED: u32 = 0x0010_0000; // U
const FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS: u32 = 0x0040_0000; // M

struct Attribute {
    flag: u32,
    name: &'static str,
    abbr: char,
}

const ATTRIBUTES: [Attribute; 13] = [
    Attribute {
        flag: FILE_ATTRIBUTE_READONLY,
        name: "readonly",
        abbr: 'R',
    },
    Attribute {
        flag: FILE_ATTRIBUTE_HIDDEN,
        name: "hidden",
        abbr: 'H',
    },
    Attribute {
        flag: FILE_ATTRIBUTE_SYSTEM,
        name: "system",
        abbr: 'S',
    },
    Attribute {
        flag: FILE_ATTRIBUTE_ARCHIVE,
        name: "archive",
        abbr: 'A',
    },
    Attribute {
        flag: FILE_ATTRIBUTE_TEMPORARY,
        name: "temporary",
        abbr: 'T',
    },
    Attribute {
        flag: FILE_ATTRIBUTE_COMPRESSED,
        name: "compressed",
        abbr: 'C',
    },
    Attribute {
        flag: FILE_ATTRIBUTE_OFFLINE,
        name: "offline",
        abbr: 'O',
    },
    Attribute {
        flag: FILE_ATTRIBUTE_NOT_CONTENT_INDEXED,
        name: "not indexed",
        abbr: 'I',
    },
    Attribute {
        flag: FILE_ATTRIBUTE_ENCRYPTED,
        name: "encrypted",
        abbr: 'E',
    },
    Attribute {
        flag: FILE_ATTRIBUTE_NO_SCRUB_DATA,
        name: "no scrub",
        abbr: 'X',
    },
    Attribute {
        flag: FILE_ATTRIBUTE_UNPINNED,
        name: "unpinned",
        abbr: 'U',
    },
    Attribute {
        flag: FILE_ATTRIBUTE_PINNED,
        name: "pinned",
        abbr: 'P',
    },
    Attribute {
        flag: FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS,
        name: "recall on data access",
        abbr: 'M',
    },
];

fn flags_to_bsd_string(flags: f::flag_t) -> String {
    let mut result = Vec::new();

    for attribute in &ATTRIBUTES {
        if attribute.flag & flags != 0 {
            result.push(attribute.name);
        }
    }

    if result.is_empty() {
        "-".to_string()
    } else {
        result.join("-")
    }
}

fn flags_to_windows_string(flags: f::flag_t) -> String {
    let mut result = String::new();

    for attribute in &ATTRIBUTES {
        if attribute.flag & flags != 0 {
            result.push(attribute.abbr);
        }
    }

    if result.is_empty() {
        result.push('-');
    }

    result
}

impl f::Flags {
    pub fn render(self, style: Style, format: FlagsFormat) -> TextCell {
        TextCell::paint(
            style,
            if format == FlagsFormat::Short {
                flags_to_windows_string(self.0)
            } else {
                flags_to_bsd_string(self.0)
            },
        )
    }
}
