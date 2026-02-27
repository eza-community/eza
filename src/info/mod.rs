// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
//! The “info” module contains routines that aren’t about probing the
//! filesystem nor displaying output to the user, but are internal “business
//! logic” routines that are performed on a file’s already-read metadata.
//! (This counts the file name as metadata.)

pub mod filetype;
mod sources;
