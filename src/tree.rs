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
            (*ptr).value = val;
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