mod mem;
pub mod mystring;
pub mod tree;

#[macro_export]
macro_rules! dict {
    ( $( $key:expr => $val:expr ),* $(,)? ) => {
        {
            let mut root: *mut $crate::tree::Node = std::ptr::null_mut();
            let mut h = false;
            unsafe {
                $(
                    $crate::tree::insert(&mut root, $key, $val, &mut h);
                )*
            }
            root
        }
    };
}

#[cfg(test)]
mod tests {
    use super::tree::*;
    use std::ptr;

    #[test]
    fn test_basic_insert_and_find() {
        unsafe {
            let mut root: *mut Node = ptr::null_mut();
            let mut h = false;

            insert(&mut root, 1, "a", &mut h);
            assert!(!root.is_null());
            assert_eq!((*root).key, 1);

            assert!(contains(root, 1));
            assert!(!contains(root, 2));

            let val_ptr = find(root, 1);
            assert!(val_ptr.is_some());
            let val_str = &*val_ptr.unwrap();

            let slice = std::slice::from_raw_parts(val_str.ptr, val_str.len);
            let s = std::str::from_utf8(slice).unwrap();
            assert_eq!(s, "a");
        }
    }

    #[test]
    fn test_update_value() {
        unsafe {
            let mut root: *mut Node = ptr::null_mut();
            let mut h = false;

            insert(&mut root, 5, "old", &mut h);
            insert(&mut root, 5, "new", &mut h);

            let val_ptr = find(root, 5).unwrap();
            let val_str = &*val_ptr;
            let slice = std::slice::from_raw_parts(val_str.ptr, val_str.len);
            let s = std::str::from_utf8(slice).unwrap();

            assert_eq!(s, "new");
        }
    }

    #[test]
    fn test_rotation_rr() {
        unsafe {
            let mut root: *mut Node = ptr::null_mut();
            let mut h = false;

            insert(&mut root, 4, "a", &mut h);
            insert(&mut root, 5, "b", &mut h);
            insert(&mut root, 7, "c", &mut h);

            assert_eq!((*root).key, 5, "5 should be a root");
            assert_eq!((*(*root).left).key, 4, "4 should be a left child of root");
            assert_eq!((*(*root).right).key, 7, "7 should be a left child of root");

            assert_eq!((*root).bl, 0, "Root should be balanced");
            assert_eq!((*(*root).left).bl, 0);
            assert_eq!((*(*root).right).bl, 0);
        }
    }

    #[test]
    fn test_rotation_ll() {
        unsafe {
            let mut root: *mut Node = ptr::null_mut();
            let mut h = false;

            insert(&mut root, 5, "b", &mut h);
            insert(&mut root, 4, "a", &mut h);
            insert(&mut root, 7, "c", &mut h);
            insert(&mut root, 2, "f", &mut h);
            insert(&mut root, 1, "g", &mut h);

            assert_eq!((*root).key, 5, "5 should be a root");
            assert_eq!((*(*root).left).key, 2, "2 should be a left child of root");
            assert_eq!((*(*root).right).key, 7, "7 should be a right child of root");
            assert_eq!((*(*(*root).left).left).key, 1, "1 should be a right child of root's left child");
            assert_eq!((*(*(*root).left).right).key, 4, "4 should be a right child of root's left child");
            assert_eq!((*root).bl, 1);
            assert_eq!((*(*root).left).bl, 0);
            assert_eq!((*(*root).right).bl, 0);
            assert_eq!((*(*(*root).left).right).bl, 0);
            assert_eq!((*(*(*root).left).left).bl, 0);
        }
    }

    #[test]
    fn test_rotation_lr() {
        unsafe {
            let mut root: *mut Node = ptr::null_mut();
            let mut h = false;

            insert(&mut root, 5, "b", &mut h);
            insert(&mut root, 4, "a", &mut h);
            insert(&mut root, 7, "c", &mut h);
            insert(&mut root, 2, "f", &mut h);
            insert(&mut root, 1, "g", &mut h);
            insert(&mut root, 3, "d", &mut h);

            assert_eq!((*root).key, 4, "4 should be a root");
            assert_eq!((*(*root).left).key, 2, "2 should be a left child of root");
            assert_eq!((*(*root).right).key, 5, "5 should be a right child of root");
            assert_eq!((*(*(*root).right).right).key, 7, "7 should be a right child of root's left child");
            assert_eq!((*(*(*root).left).left).key, 1, "1 should be a right child of root's left child");
            assert_eq!((*(*(*root).left).right).key, 3, "3 should be a right child of root's left child");

            assert_eq!((*root).bl, 0);
            assert_eq!((*(*root).left).bl, 0);
            assert_eq!((*(*root).right).bl, -1);
            assert_eq!((*(*(*root).left).right).bl, 0);
            assert_eq!((*(*(*root).left).left).bl, 0);
            assert_eq!((*(*(*root).right).right).bl, 0);
        }
    }

    #[test]
    fn test_rotation_rl() {
        unsafe {
            let mut root: *mut Node = ptr::null_mut();
            let mut h = false;

            insert(&mut root, 5, "b", &mut h);
            insert(&mut root, 4, "a", &mut h);
            insert(&mut root, 7, "c", &mut h);
            insert(&mut root, 2, "f", &mut h);
            insert(&mut root, 1, "g", &mut h);
            insert(&mut root, 3, "d", &mut h);
            insert(&mut root, 6, "e", &mut h);

            assert_eq!((*root).key, 4, "4 should be a root");
            assert_eq!((*(*root).left).key, 2, "2 should be a left child of root");
            assert_eq!((*(*root).right).key, 6, "6 should be a right child of root");
            assert_eq!((*(*(*root).right).right).key, 7, "7 should be a right child of root's left child");
            assert_eq!((*(*(*root).left).left).key, 1, "1 should be a right child of root's left child");
            assert_eq!((*(*(*root).left).right).key, 3, "3 should be a right child of root's left child");
            assert_eq!((*(*(*root).right).left).key, 5, "5 should be a right child of root's left child");

            assert_eq!((*root).bl, 0);
            assert_eq!((*(*root).left).bl, 0);
            assert_eq!((*(*root).right).bl, 0);
            assert_eq!((*(*(*root).left).right).bl, 0);
            assert_eq!((*(*(*root).left).left).bl, 0);
            assert_eq!((*(*(*root).right).right).bl, 0);
            assert_eq!((*(*(*root).right).left).bl, 0);
        }
    }

    #[test]
    fn test_contains() {
        unsafe {
            let mut root: *mut Node = ptr::null_mut();
            let mut h = false;

            insert(&mut root, 5, "b", &mut h);
            insert(&mut root, 4, "a", &mut h);
            insert(&mut root, 7, "c", &mut h);
            insert(&mut root, 2, "f", &mut h);
            insert(&mut root, 1, "g", &mut h);
            insert(&mut root, 3, "d", &mut h);
            insert(&mut root, 6, "e", &mut h);

            assert_eq!(contains(root, 1), true);
            assert_eq!(contains(root, 2), true);
            assert_eq!(contains(root, 3), true);
            assert_eq!(contains(root, 4), true);
            assert_eq!(contains(root, 5), true);
            assert_eq!(contains(root, 6), true);
            assert_eq!(contains(root, 7), true);
            assert_eq!(contains(root, 8), false);
        }
    }

    #[test]
    fn test_delete_simple_leaf() {
        unsafe {
            let mut root: *mut Node = ptr::null_mut();
            let mut h = false;
            insert(&mut root, 10, "a", &mut h);
            insert(&mut root, 5, "b", &mut h);

            delete(&mut root, 5, &mut h);

            assert!(contains(root, 10));
            assert!(!contains(root, 5));
            assert!((*root).left.is_null());
        }
    }

    #[test]
    fn test_delete_simple_with_one_child() {
        unsafe {
            let mut root: *mut Node = ptr::null_mut();
            let mut h = false;

            insert(&mut root, 10, "a", &mut h);
            insert(&mut root, 5, "b", &mut h);

            delete(&mut root, 10, &mut h);

            assert!(!contains(root, 10));
            assert!(contains(root, 5));
            assert_eq!((*root).key, 5);
        }
    }

    #[test]
    fn test_delete_simple_with_two_children_and_balance() {
        unsafe {
            let mut root: *mut Node = ptr::null_mut();
            let mut h = false;

            insert(&mut root, 2, "b", &mut h);
            insert(&mut root, 1, "a", &mut h);
            insert(&mut root, 3, "c", &mut h);
            insert(&mut root, 5, "e", &mut h);
            insert(&mut root, 4, "d", &mut h);

            delete(&mut root, 2, &mut h);

            assert!(!contains(root, 2));
            assert!(contains(root, 3));
            assert!(contains(root, 1));
            assert_eq!((*root).key, 3);
        }
    }

    #[test]
    fn test_ppt_delete() {
        unsafe {
            let mut root: *mut Node = ptr::null_mut();
            let mut h = false;

            insert(&mut root, 5, "b", &mut h);
            insert(&mut root, 3, "d", &mut h);
            insert(&mut root, 8, "h", &mut h);
            insert(&mut root, 2, "f", &mut h);
            insert(&mut root, 4, "i", &mut h);
            insert(&mut root, 7, "c", &mut h);
            insert(&mut root, 10, "j", &mut h);
            insert(&mut root, 9, "k", &mut h);
            insert(&mut root, 11, "l", &mut h);
            insert(&mut root, 1, "g", &mut h);

            assert_eq!((*root).key, 5);
            assert_eq!((*(*root).left).key, 3);
            assert_eq!((*(*root).right).key, 8);
            assert_eq!((*(*(*root).right).right).key, 10);
            assert_eq!((*(*(*root).left).left).key, 2);
            assert_eq!((*(*(*(*root).left).left).left).key, 1);
            assert_eq!((*(*(*root).left).right).key, 4);
            assert_eq!((*(*(*root).right).left).key, 7);
            assert_eq!((*(*(*(*root).right).right).left).key, 9);
            assert_eq!((*(*(*(*root).right).right).right).key, 11);

            h = false;
            delete(&mut root, 4, &mut h);

            assert_eq!((*root).key, 5);
            assert_eq!((*(*root).left).key, 2);
            assert_eq!((*(*(*root).left).left).key, 1);
            assert_eq!((*(*(*root).left).right).key, 3);

            h = false;
            delete(&mut root, 8, &mut h);

            assert_eq!((*root).key, 5);
            assert_eq!((*(*root).right).key, 9);
            assert_eq!((*(*(*root).right).left).key, 7);
            assert_eq!((*(*(*root).right).right).key, 10);
            assert_eq!((*(*(*(*root).right).right).right).key, 11);

            h = false;
            delete(&mut root, 5, &mut h);

            assert_eq!((*root).key, 7);
            assert_eq!((*(*root).right).key, 10);
            assert_eq!((*(*(*root).right).left).key, 9);
            assert_eq!((*(*(*root).right).right).key, 11);
            assert_eq!((*(*root).left).key, 2);
            assert_eq!((*(*(*root).left).left).key, 1);
            assert_eq!((*(*(*root).left).right).key, 3);
        }
    }

    #[test]
    fn test_macro_creates_dictionary() {
        let root = dict! {
                1 => "a",
                2 => "b",
                3  => "c"
            };

        unsafe {
            assert!(!root.is_null());

            assert!(contains(root, 1));
            assert!(contains(root, 2));
            assert!(contains(root, 3));
            assert!(!contains(root, 4));

            let val_ptr = find(root, 1).unwrap();
            assert_eq!((*val_ptr).as_str(), "a");
        }
    }
}
