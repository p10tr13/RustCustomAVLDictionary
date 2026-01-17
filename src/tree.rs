use crate::mem::{alloc, free};
use crate::mystring::MyString;
use std::ptr;

pub struct Node {
    pub key: u64,
    pub value: MyString,
    pub left: *mut Node,
    pub right: *mut Node,
    pub bl: i8,
}

impl Node {
    pub fn new(key: u64, value: &str) -> *mut Node {
        unsafe {
            let ptr = alloc::<Node>();
            if ptr.is_null() {
                return ptr::null_mut();
            }

            let val = match MyString::from_str(value) {
                Some(v) => v,
                None => {
                    free(ptr);
                    return ptr::null_mut();
                }
            };

            (*ptr).key = key;
            ptr::write(&mut (*ptr).value, val);
            (*ptr).bl = 0;
            (*ptr).left = ptr::null_mut();
            (*ptr).right = ptr::null_mut();

            ptr
        }
    }
}

unsafe fn rotate_ll(root: &mut *mut Node) {
    unsafe {
        let p = *root;
        let x = (*p).left;

        (*p).left = (*x).right;
        (*x).right = p;

        if (*x).bl == 1 {
            (*p).bl = 0;
        } else {
            (*p).bl = 1;
        }

        if (*x).bl == 0 {
            (*x).bl = -1;
        } else {
            (*x).bl = 0;
        }

        *root = x;
    }
}

unsafe fn rotate_rr(root: &mut *mut Node) {
    unsafe {
        let p = *root;
        let x = (*p).right;

        (*p).right = (*x).left;
        (*x).left = p;

        if (*x).bl == -1 {
            (*p).bl = 0;
        } else {
            (*p).bl = -1;
        }

        if (*x).bl == 0 {
            (*x).bl = 1;
        } else {
            (*x).bl = 0;
        }

        *root = x;
    }
}

unsafe fn rotate_lr(root: &mut *mut Node) {
    unsafe {
        let p = *root;
        let x = (*p).left;
        let y = (*x).right;

        (*x).right = (*y).left;
        (*y).left = x;
        (*p).left = (*y).right;
        (*y).right = p;

        if (*y).bl == 1 {
            (*p).bl = -1;
        } else {
            (*p).bl = 0;
        }

        if (*y).bl == -1 {
            (*x).bl = 1;
        } else {
            (*x).bl = 0;
        }

        (*y).bl = 0;

        *root = y;
    }
}

unsafe fn rotate_rl(root: &mut *mut Node) {
    unsafe {
        let p = *root;
        let x = (*p).right;
        let y = (*x).left;

        (*x).left = (*y).right;
        (*y).right = x;
        (*p).right = (*y).left;
        (*y).left = p;

        if (*y).bl == -1 {
            (*p).bl = 1;
        } else {
            (*p).bl = 0;
        }

        if (*y).bl == 1 {
            (*x).bl = -1;
        } else {
            (*x).bl = 0;
        }

        (*y).bl = 0;

        *root = y;
    }
}

unsafe fn balance_after_insert_left(root: &mut *mut Node, h: &mut bool) {
    unsafe {
        match (*(*root)).bl {
            1 => {
                let ptr = (*(*root)).left;
                if (*ptr).bl == 1 || (*ptr).bl == 0 {
                    rotate_ll(root);
                } else {
                    rotate_lr(root);
                }
                *h = false;
            },
            0 => {
                (*(*root)).bl = 1;
            },
            -1 => {
                (*(*root)).bl = 0;
                *h = false;
            },
            _ => {}
        }
    }
}

unsafe fn balance_after_insert_right(root: &mut *mut Node, h: &mut bool) {
    unsafe {
        match (*(*root)).bl {
            -1 => {
                let ptr = (*(*root)).right;
                if (*ptr).bl == -1 || (*ptr).bl == 0 {
                    rotate_rr(root);
                } else {
                    rotate_rl(root);
                }
                *h = false;
            },
            0 => {
                (*(*root)).bl = -1;
            },
            1 => {
                (*(*root)).bl = 0;
                *h = false;
            },
            _ => {}
        }
    }
}

pub unsafe fn insert(root: &mut *mut Node, key: u64, value: &str, h: &mut bool) {
    if (*root).is_null() {
        *root = Node::new(key, value);
        *h = !(*root).is_null();
        return;
    }

    unsafe {
        if key < (*(*root)).key {
            insert(&mut (*(*root)).left, key, value, h);
            if *h {
                balance_after_insert_left(root, h);
            }
        } else if key > (*(*root)).key {
            insert(&mut (*(*root)).right, key, value, h);
            if *h {
                balance_after_insert_right(root, h);
            }
        } else {
            if let Some(new_val) = MyString::from_str(value) {
                (*(*root)).value = new_val;
            }
            *h = false;
        }
    }
}

pub unsafe fn contains(mut root: *mut Node, key: u64) -> bool {
    unsafe {
        while !root.is_null() {
            let cur_key = (*root).key;

            if cur_key < key {
                root = (*root).right;
            } else if cur_key > key {
                root = (*root).left;
            }
            else {
                return true;
            }
        }
    }
    false
}

pub unsafe fn find(mut root: *mut Node, key: u64) -> Option<*mut MyString> {
    unsafe {
        while !root.is_null() {
            let current_key = (*root).key;

            if key == current_key {
                return Some(&mut (*root).value as *mut MyString);
            } else if key < current_key {
                root = (*root).left;
            } else {
                root = (*root).right;
            }
        }
    }

    None
}

unsafe fn get_min(mut root: *mut Node) -> *mut Node {
    unsafe {
        while !(*root).left.is_null() {
            root = (*root).left;
        }
    }
    root
}

unsafe fn balance_after_delete_left(root: &mut *mut Node, h: &mut bool) {
    unsafe {
        match (*(*root)).bl {
            1 => {
                (*(*root)).bl = 0;
                *h = true;
            },
            0 => {
                (*(*root)).bl = -1;
                *h = false;
            },
            -1 => {
                let right_child = (*(*root)).right;

                if (*right_child).bl <= 0 {
                    rotate_rr(root);
                } else {
                    rotate_rl(root);
                }

                if (*right_child).bl == 0 {
                    *h = false;
                } else {
                    *h = true;
                }
            },
            _ => {}
        }
    }
}

unsafe fn balance_after_delete_right(root: &mut *mut Node, h: &mut bool) {
    unsafe {
        match (*(*root)).bl {
            -1 => {
                (*(*root)).bl = 0;
                *h = true;
            },
            0 => {
                (*(*root)).bl = 1;
                *h = false;
            },
            1 => {
                let left_child = (*(*root)).left;

                if (*left_child).bl >= 0 {
                    rotate_ll(root);
                } else {
                    rotate_lr(root);
                }

                if (*left_child).bl == 0 {
                    *h = false;
                } else {
                    *h = true;
                }
            },
            _ => {}
        }
    }
}

pub unsafe fn delete(root: &mut *mut Node, key: u64, h: &mut bool) {
    if (*root).is_null() {
        *h = false;
        return;
    }

    unsafe {
        if key < (*(*root)).key {
            delete(&mut (*(*root)).left, key, h);
            if *h {
                balance_after_delete_left(root, h);
            }
        } else if key > (*(*root)).key {
            delete(&mut (*(*root)).right, key, h);
            if *h {
                balance_after_delete_right(root, h);
            }
        } else {
            if (*(*root)).left.is_null() || (*(*root)).right.is_null() {
                let temp = *root;
                if (*(*root)).left.is_null() {
                    *root = (*(*root)).right;
                } else {
                    *root = (*(*root)).left;
                }

                ptr::drop_in_place(&mut (*temp).value);
                free(temp);
                *h = true;
            } else {
                let successor = get_min((*(*root)).right);

                (*(*root)).key = (*successor).key;

                ptr::drop_in_place(&mut (*(*root)).value);

                if let Some(new_val) = (*successor).value.deep_clone() {
                    ptr::write(&mut (*(*root)).value, new_val);
                }

                delete(&mut (*(*root)).right, (*successor).key, h);

                if *h {
                    balance_after_delete_right(root, h);
                }
            }
        }
    }
}

pub unsafe fn print_tree(root: *mut Node, level: usize) {
    if root.is_null() {
        return;
    }
    unsafe {
        print_tree((*root).right, level + 1);

        for _ in 0..level {
            print!("    ");
        }

        println!("{} (bf={}) : {}", (*root).key, (*root).bl, (*root).value.as_str());

        print_tree((*root).left, level + 1);
    }
}