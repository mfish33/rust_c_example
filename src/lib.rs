#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/lib_bindings.rs"));

use core::slice;
use std::{ops::{Deref, DerefMut}, os::raw::c_void};
use rgsl::statistics::skew;

struct CArr<T> where T:'static {
    arr: &'static mut[T]
}

impl<T> CArr<T> {
    fn new(ptr: *mut T, len: usize) -> Self {
        let arr = unsafe {slice::from_raw_parts_mut(ptr, len)};
        Self {
            arr
        }
    }
}

impl<T> Deref for CArr<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.arr    
    }
}

impl<T> DerefMut for CArr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
      &mut self.arr  
    }
}

impl<T> Drop for CArr<T> {
    fn drop(&mut self) {
       unsafe { libc::free(self.arr.as_mut_ptr() as *mut c_void) }
    }
}

#[no_mangle]
pub extern "C" fn rust_function() {
    println!("Hello from Rust!");

    let data_size = 1024;
    let ptr = unsafe{generate_data(data_size)};
    let arr = CArr::new(ptr, data_size as usize);

    let answer = skew(&arr, 1, arr.len());

    println!("Got {answer} from C")
}