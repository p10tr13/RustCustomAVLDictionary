use std::ffi::c_void;
use std::mem::size_of;

pub unsafe fn alloc<T>() -> *mut T {
    unsafe {
        let size = size_of::<T>();
        libc::malloc(size) as *mut T
    }
}

pub unsafe fn free<T>(ptr: *mut T) {
    if !ptr.is_null() {
        unsafe { libc::free(ptr as *mut c_void); }
    }
}