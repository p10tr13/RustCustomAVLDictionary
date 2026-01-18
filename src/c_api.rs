use crate::tree::{insert, delete, find, contains, Node};
use crate::mem::free;
use std::ffi::{c_char, CStr};
use std::ptr;

pub type Dict = Node;

/// # Safety
#[unsafe(no_mangle)]
pub extern "C" fn dict() -> *mut Dict {
    ptr::null_mut()
}

/// # Safety
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_insert(root: *mut *mut Dict, key: u64, value: *const c_char) {
    if root.is_null() || value.is_null() {
        return;
    }

    unsafe {
        let c_str = CStr::from_ptr(value);
        let str_slice = match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return,
        };

        let mut h = false;

        insert(&mut *root, key, str_slice, &mut h);
    }
}

/// # Safety
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_find(root: *mut Dict, key: u64) -> *const c_char {
    unsafe {
        match find(root, key) {
            Some(val_ptr) => {
                (*val_ptr).ptr as *const c_char
            },
            None => ptr::null(),
        }
    }
}

/// # Safety
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_contains(root: *mut Dict, key: u64) -> i32 {
    unsafe {
        if contains(root, key) {
            1
        } else {
            0
        }
    }
}

/// # Safety
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_delete(root_ptr: *mut *mut Dict, key: u64) {
    if root_ptr.is_null() {
        return;
    }

    let mut h = false;
    unsafe {
        delete(&mut *root_ptr, key, &mut h);
    }
}

/// # Safety
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dict_clear(root: *mut Dict) {
    if !root.is_null() {
        unsafe {
            dict_clear((*root).left);
            dict_clear((*root).right);
            ptr::drop_in_place(&mut (*root).value);
            free(root);
        }
    }
}