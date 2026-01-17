use crate::mem::free;
use std::ptr;

pub struct MyString {
    pub ptr: *mut u8,
    pub len: usize,
}

impl MyString {
    pub fn from_str(str: &str) -> Option<MyString> {
        let len = str.len();

        unsafe {
            let ptr = libc::malloc(len + 1) as *mut u8;

            if ptr.is_null() {
                return None;
            }

            ptr::copy_nonoverlapping(str.as_ptr(), ptr, len);

            *ptr.offset(len as isize) = 0;

            Some(MyString { ptr, len })
        }
    }

    pub fn deep_clone(&self) -> Option<MyString> {
        unsafe {
            let new_ptr = libc::malloc(self.len + 1) as *mut u8;

            if new_ptr.is_null() {
                return None;
            }

            ptr::copy_nonoverlapping(self.ptr, new_ptr, self.len + 1);

            Some(MyString { ptr: self.ptr, len: self.len })
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.ptr, self.len)
        }
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            let slice = std::slice::from_raw_parts(self.ptr, self.len);
            match std::str::from_utf8(slice) {
                Ok(s) => s,
                Err(_) => "<invalid utf8>",
            }
        }
    }
}

impl Drop for MyString {
    fn drop(&mut self) {
        unsafe {
            free(self.ptr);
        }
    }
}