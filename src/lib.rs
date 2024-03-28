#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::mem;
use std::os::raw::c_char;

use libc::ptrdiff_t;

include!(concat!("../libprom2json/", "bindings.rs"));
const DUMMY_SAFE_PTR: &[u8] = &[0u8; 1024];

pub fn get_allocation_pointer<T>(data: &[T]) -> *const T {
    if data.is_empty() {
        // We can't provide NULL as sometimes Go will panic on finding one, even with length 0.
        // DUMMY_SAFE_PTR points to a static array of 1024 bytes, zeroed out and safe to read.
        DUMMY_SAFE_PTR.as_ptr() as *const T
    } else {
        data.as_ptr()
    }
}

impl GoString {
    pub fn from_string(mut str: String) -> GoString {
        str.shrink_to_fit();
        let ptr = get_allocation_pointer(str.as_bytes());
        let len = str.len();
        mem::forget(str);
        GoString {
            p: ptr as *const c_char,
            n: len as ptrdiff_t,
        }
    }

    // WARNING: The string must live for the lifetime of GoString.
    pub unsafe fn from_bytes_unmanaged(str: &[u8]) -> GoString {
        let ptr = get_allocation_pointer(str);
        let len = str.len();
        GoString {
            p: ptr as *const c_char,
            n: len as ptrdiff_t,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::{CStr, CString};

    use super::{add_numbers, prom_to_json, GoInt, GoString};

    #[test]
    fn test_addition() {
        unsafe {
            let x: GoInt = 10;
            let y: GoInt = 20;
            let r: GoInt = add_numbers(x, y);
            assert_eq!(r, 30);
        }
    }

    // https://dev.to/socrateslee/convert-string-to-cstr-and-back-in-rust-1617
    #[test]
    fn test_prom_to_json() {
        let x: GoString = unsafe { GoString::from_bytes_unmanaged("Helloo".as_bytes()) };
        let result_one: CString = unsafe {
            let result = prom_to_json(x);
            CString::from_raw(result)
        };
        println!("{:?}", result_one);

        let my_name = CString::new("oto brglez").unwrap();
        let go_str_ref = GoString {
            p: my_name.as_ptr(),
            n: my_name.as_bytes().len() as isize,
        };
        let result_two = unsafe { CString::from_raw(prom_to_json(go_str_ref)) };
        println!("{:?}", result_two);

        /*
        let s:String = "Hello World!".to_string();
        let c_string: CString = CString::new(s.as_str()).unwrap();
        let c_str: &CStr = c_string.as_c_str();
        let result_three = unsafe { CString::from_raw(prom_to_json(c_string)) };
        println!("{:?}", result_three);
         */
    }
}
