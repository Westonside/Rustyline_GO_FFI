use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::io::{self, Read};
// use crate::error::AppError;
use base64::encode;
use rustyline::KeyCode::F;

extern "C" {
    fn testing(something: GoString) -> *const c_char;
}

#[repr(C)]
struct GoString {
    a: *const c_char,
    b: i64,
}



pub fn make_sqip(path: &str) -> Result<String, bool> {
    println!("starting the run");
    let c_path = CString::new(path).expect("CString::new failed");
    let ptr = c_path.as_ptr();
    let go_string = GoString {
        a: ptr,
        b: c_path.as_bytes().len() as i64,
    };
    let result = unsafe { testing(go_string) };
    let c_str = unsafe { CStr::from_ptr(result) };
    let string = c_str.to_str().expect("Error translating SQIP from library");
    match string.is_empty() || string.starts_with("Error") {
        true => Err(false),
        false => Ok(string.parse().unwrap()),
    }
}