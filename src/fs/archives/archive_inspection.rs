// SPDX-FileCopyrightText: 2025 eza contributors
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2025 eza contributors
// SPDX-License-Identifier: MIT
#[derive(Debug, Clone, PartialEq)]
pub enum ArchiveInspection {
    Always,
    Never,
    // TODO: option to limit file size (especially for compressed archives)
}
