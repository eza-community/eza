// SPDX-FileCopyrightText: 2024 Christina Sørensen
// SPDX-License-Identifier: EUPL-1.2
//
// SPDX-FileCopyrightText: 2023-2024 Christina Sørensen, eza contributors
// SPDX-FileCopyrightText: 2014 Benjamin Sago
// SPDX-License-Identifier: MIT
/// Used to represent a the size of a recursive directory traversal.  `None`
/// should be used when the file does not represent a directory or the recursive
/// size should not be calculated.
#[derive(Copy, Clone, Debug)]
pub enum RecursiveSize {
    /// Size should not be computed
    None,
    /// Size should be computed but has not been computed yet
    Unknown,
    /// Size has been computed.  First field is size in bytes and second field
    /// is size in blocks
    #[cfg_attr(target_family = "windows", allow(dead_code))]
    Some(u64, u64),
}

impl RecursiveSize {
    /// Returns `true` if `None`
    ///
    /// # Examples
    ///
    /// ```
    /// use eza::fs::recursive_size::RecursiveSize;
    ///
    /// let x = RecursiveSize::None;
    /// assert_eq!(x.is_none(), true);
    ///
    /// let x = RecursiveSize::Unknown;
    /// assert_eq!(x.is_none(), false);
    ///
    /// let x = RecursiveSize::Some(0, 0);
    /// assert_eq!(x.is_none(), false);
    /// ```
    #[inline]
    pub const fn is_none(&self) -> bool {
        matches!(*self, Self::None)
    }

    /// Returns the contained [`Some`] value or a provided default.
    ///
    /// # Examples
    ///
    /// ```
    /// use eza::fs::recursive_size::RecursiveSize;
    ///
    /// assert_eq!(RecursiveSize::None.unwrap_bytes_or(1), 1);
    /// assert_eq!(RecursiveSize::Unknown.unwrap_bytes_or(1), 1);
    /// assert_eq!(RecursiveSize::Some(2, 3).unwrap_bytes_or(1), 2);
    /// ```
    #[inline]
    pub const fn unwrap_bytes_or(self, default: u64) -> u64 {
        match self {
            Self::Some(bytes, _blocks) => bytes,
            _ => default,
        }
    }

    /// Returns the provided default result (if None or Unknown),
    /// or applies a function to the contained value (if Some).
    ///
    /// # Examples
    ///
    /// ```
    /// use eza::fs::recursive_size::RecursiveSize;
    ///
    /// assert_eq!(RecursiveSize::None.map_or(None, |s, _| Some(s * 2)), None);
    /// assert_eq!(RecursiveSize::Unknown.map_or(None, |s, _| Some(s * 2)), None);
    /// assert_eq!(RecursiveSize::Some(2, 3).map_or(None, |s, _| Some(s * 2)), Some(4));
    /// ```
    #[inline]
    #[cfg_attr(target_family = "windows", allow(dead_code))]
    pub fn map_or<U, F>(self, default: U, f: F) -> U
    where
        F: FnOnce(u64, u64) -> U,
    {
        match self {
            RecursiveSize::Some(bytes, blocks) => f(bytes, blocks),
            _ => default,
        }
    }
}
