// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
pub mod xattr;

#[cfg(feature = "git")]
pub mod git;

#[cfg(not(feature = "git"))]
pub mod git {
    use std::iter::FromIterator;
    use std::path::{Path, PathBuf};

    use crate::fs::fields as f;

    pub struct GitCache;

    impl FromIterator<PathBuf> for GitCache {
        fn from_iter<I>(_iter: I) -> Self
        where
            I: IntoIterator<Item = PathBuf>,
        {
            Self
        }
    }

    impl GitCache {
        pub fn has_anything_for(&self, _index: &Path) -> bool {
            false
        }

        pub fn get(&self, _index: &Path, _prefix_lookup: bool) -> f::Git {
            unreachable!();
        }
    }

    impl f::SubdirGitRepo {
        pub fn from_path(_dir: &Path, _status: bool) -> Self {
            panic!("Tried to get subdir Git status, but Git support is disabled")
        }
    }
}
