// SPDX-License-Identifier: Apache-2.0

use std::ffi::{c_char, CStr, CString};

#[derive(Debug, Clone)]
pub struct RabcClient {
    name: String,
}

const RABC_PASS: u32 = 0;
const RABC_FAIL: u32 = 1;
const RABC_FAIL_NULL_POINTER: u32 = 2;

#[no_mangle]
pub extern "C" fn rabc_client_new(
    client: *mut *mut RabcClient,
    name: *mut c_char,
    err_msg: *mut *mut c_char,
) -> u32 {
    if client.is_null() || name.is_null() || err_msg.is_null() {
        return RABC_FAIL_NULL_POINTER;
    }

    unsafe {
        *client = std::ptr::null_mut();
        *err_msg = std::ptr::null_mut();
    }

    let name_cstr = unsafe { CStr::from_ptr(name) };
    let name_str = match name_cstr.to_str() {
        Ok(s) => s,
        Err(e) => {
            unsafe {
                *err_msg = CString::new(format!(
                    "Error on converting `name` C char to rust str: {e}"
                ))
                .unwrap()
                .into_raw();
            }
            return RABC_FAIL;
        }
    };
    let mut name_string = name_str.to_string();
    name_string.make_ascii_lowercase();

    let cli = RabcClient { name: name_string };

    unsafe {
        *client = Box::into_raw(Box::new(cli));
        RABC_PASS
    }
}

#[no_mangle]
pub extern "C" fn rabc_client_get_name(client: *mut RabcClient) -> *mut c_char {
    if client.is_null() {
        return std::ptr::null_mut();
    }

    let client: &RabcClient = unsafe { &mut *client };

    let name_cstr = match CString::new(client.name.as_str()) {
        Ok(c) => c,
        Err(_) => {
            return std::ptr::null_mut();
        }
    };

    name_cstr.into_raw()
}

#[no_mangle]
pub extern "C" fn rabc_cstring_free(cstring: *mut c_char) {
    unsafe {
        if !cstring.is_null() {
            let _ = CString::from_raw(cstring);
        }
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn rabc_client_free(client: *mut RabcClient) {
    if !client.is_null() {
        unsafe {
            drop(Box::from_raw(client));
        }
    }
}
