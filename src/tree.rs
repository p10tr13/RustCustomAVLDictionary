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