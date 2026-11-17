//! A safe, owning wrapper around the raw AVL tree.

use crate::c_api::dict_clear;
use crate::tree::{self, Node};
use std::ptr;

/// A dictionary from `u64` keys to strings, backed by the AVL tree.
///
/// It owns all of its nodes and frees them when it goes out of scope, so the
/// caller never touches raw pointers or `unsafe` code.
///
/// ```
/// use project_custom_avl_dictionary::Dictionary;
///
/// let mut dict = Dictionary::new();
/// dict.insert(2, "two");
/// dict.insert(1, "one");
/// assert_eq!(dict.get(2), Some("two"));
/// assert!(dict.remove(1));
/// assert_eq!(dict.len(), 1);
/// ```
pub struct Dictionary {
    root: *mut Node,
    len: usize,
}

impl Dictionary {
    /// Creates an empty dictionary.
    pub fn new() -> Self {
        Dictionary {
            root: ptr::null_mut(),
            len: 0,
        }
    }

    /// Inserts `value` under `key`, replacing the previous value if the key exists.
    pub fn insert(&mut self, key: u64, value: &str) {
        let existed = self.contains(key);
        let mut h = false;
        unsafe { tree::insert(&mut self.root, key, value, &mut h) };
        if !existed && self.contains(key) {
            self.len += 1;
        }
    }

    /// Returns the value stored under `key`, if any.
    pub fn get(&self, key: u64) -> Option<&str> {
        unsafe { tree::find(self.root, key).map(|value| (*value).as_str()) }
    }

    /// Returns `true` if the dictionary holds `key`.
    pub fn contains(&self, key: u64) -> bool {
        unsafe { tree::contains(self.root, key) }
    }

    /// Removes `key` and returns `true` if it was present.
    pub fn remove(&mut self, key: u64) -> bool {
        if !self.contains(key) {
            return false;
        }
        let mut h = false;
        unsafe { tree::delete(&mut self.root, key, &mut h) };
        self.len -= 1;
        true
    }

    /// Returns the number of keys in the dictionary.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the dictionary holds no keys.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Prints the tree sideways (right subtree on top) with balance factors.
    pub fn print(&self) {
        unsafe { tree::print_tree(self.root, 0) };
    }

    /// Returns the raw root pointer, e.g. to inspect the tree structure in tests.
    pub fn root(&self) -> *const Node {
        self.root
    }
}

impl Default for Dictionary {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Dictionary {
    fn drop(&mut self) {
        unsafe { dict_clear(self.root) };
    }
}

/// Builds a [`Dictionary`] from `key => value` pairs.
///
/// ```
/// use project_custom_avl_dictionary::dictionary;
///
/// let dict = dictionary! { 1 => "a", 2 => "b" };
/// assert_eq!(dict.get(1), Some("a"));
/// ```
#[macro_export]
macro_rules! dictionary {
    ( $( $key:expr => $val:expr ),* $(,)? ) => {
        {
            #[allow(unused_mut)]
            let mut dict = $crate::Dictionary::new();
            $(
                dict.insert($key, $val);
            )*
            dict
        }
    };
}
