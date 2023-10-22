mod dir;
pub use self::dir::{Dir, DotFilter};

mod file;
pub use self::file::{File, FileTarget};

pub mod dir_action;
pub mod feature;
pub mod fields;
pub mod filter;
pub mod mounts;

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
lazy_static! {
    static ref RECURSIVE_SIZE_HASHMAP: Mutex<HashMap<(u64, u64), u64>> = Mutex::new(HashMap::new());
}
