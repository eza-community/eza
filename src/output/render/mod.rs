// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
#[cfg(unix)]
mod blocks;
#[cfg(unix)]
pub use self::blocks::Colours as BlocksColours;

mod filetype;
pub use self::filetype::Colours as FiletypeColours;

mod git;
pub use self::git::Colours as GitColours;
pub use self::git::RepoColours as GitRepoColours;

#[cfg(unix)]
mod groups;
#[cfg(unix)]
pub use self::groups::{Colours as GroupColours, Render as GroupRender};

#[cfg(unix)]
mod inode;
// inode uses just one colour

mod links;
pub use self::links::Colours as LinksColours;

mod permissions;
pub use self::permissions::{Colours as PermissionsColours, PermissionsPlusRender};

mod size;
pub use self::size::Colours as SizeColours;

mod times;
pub use self::times::Render as TimeRender;
// times does too

#[cfg(unix)]
mod users;
#[cfg(unix)]
pub use self::users::Colours as UserColours;
#[cfg(unix)]
pub use self::users::Render as UserRender;

mod octal;
#[cfg(unix)]
pub use self::octal::Render as OctalPermissionsRender;
// octal uses just one colour

mod securityctx;
pub use self::securityctx::Colours as SecurityCtxColours;

#[cfg(any(
    target_os = "macos",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "dragonfly"
))]
mod flags_bsd;

#[cfg(windows)]
mod flags_windows;

#[cfg(not(any(
    target_os = "macos",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "dragonfly",
    target_os = "windows"
)))]
mod flags;
