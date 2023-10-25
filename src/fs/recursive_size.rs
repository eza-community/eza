/// Used to represent a the size of a recursive directory traversal.  `None`
/// should be used when the file does not represent a directory or the recursive
/// size should not be calculated.
#[derive(Copy, Clone, Debug)]
pub enum RecursiveSize {
    /// Size should not be computed
    None,
    /// Size should be computed but has not been computed yet
    Unknown,
    /// Size has been computed
    Some(u64),
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
    /// let x = RecursiveSize::Some(0);
    /// assert_eq!(x.is_none(), false);
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
    /// assert_eq!(RecursiveSize::None.unwrap_or(1), 1);
    /// assert_eq!(RecursiveSize::Unknown.unwrap_or(1), 1);
    /// assert_eq!(RecursiveSize::Some(2).unwrap_or(1), 2);
    /// ```
    #[inline]
    pub const fn unwrap_or(self, default: u64) -> u64 {
        match self {
            Self::Some(x) => x,
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
    /// assert_eq!(RecursiveSize::None.map_or(None, |s| Some(s * 2)), None);
    /// assert_eq!(RecursiveSize::Unknown.map_or(None, |s| Some(s * 2)), None);
    /// assert_eq!(RecursiveSize::Some(2).map_or(None, |s| Some(s * 2)), Some(4));
    #[inline]
    pub fn map_or<U, F>(self, default: U, f: F) -> U
    where
        F: FnOnce(u64) -> U,
    {
        match self {
            RecursiveSize::Some(x) => f(x),
            _ => default,
        }
    }
}
