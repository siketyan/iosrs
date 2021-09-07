mod greeter;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::greeter::Greeter;

#[no_mangle]
pub extern "C" fn iosrs_greet(name: *const c_char) -> *mut c_char {
    let name = match get_cstr(name).to_str() {
        Ok(str) => str,
        _ => "error",
    };

    let greeter = Greeter::new(name);
    let greeting = greeter.greet();

    CString::new(greeting).unwrap().into_raw()
}

fn get_cstr(ptr: *const c_char) -> &'static CStr {
    unsafe { CStr::from_ptr(ptr) }
}
