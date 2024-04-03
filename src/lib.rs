#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod ella_metrics;
pub mod cli;
pub mod metric_client;
pub mod metric_endpoint;
pub mod metric_parser;
mod utils;
pub mod family;

use libc::ptrdiff_t;
use std::mem;
use std::os::raw::c_char;

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

pub mod prom_to_json {
    use super::{prom_to_json as low_prom_to_json, GoString};
    use std::ffi::CString;

    pub fn parse(raw: String) -> Result<String, Box<dyn std::error::Error>> {
        let raw_input: CString = CString::new(raw).unwrap();
        let raw_input_as_go_string = GoString {
            p: raw_input.as_ptr(),
            n: raw_input.as_bytes().len() as isize,
        };

        let result: (String, String) = unsafe {
            let result = low_prom_to_json(raw_input_as_go_string);
            (
                CString::from_raw(result.r0).into_string().unwrap(),
                CString::from_raw(result.r1).into_string().unwrap(),
            )
        };

        match result {
            (ok, some) if some.is_empty() => Ok(ok),
            (_, err) => Err(Box::from(err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::prom_to_json;
    use std::fs;

    #[test]
    fn test_prom_to_json() {
        let error_example = prom_to_json::parse("hello".to_string());
        let error = error_example.err().unwrap().to_string();
        assert!(!error.is_empty(), "not good. expecting error.");

        let prom_example = fs::read_to_string("./tests/data/example.prom").unwrap();
        let ok_example = prom_to_json::parse(prom_example).unwrap();

        assert!(!ok_example.is_empty(), "not good. expecting json.");
    }
}
