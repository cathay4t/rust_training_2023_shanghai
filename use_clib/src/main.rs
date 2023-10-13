// SPDX-License-Identifier: Apache-2.0


fn main() {
    let cs = unsafe { std::ffi::CStr::from_ptr(libz_sys::zlibVersion())};

    println!("Version: {}", cs.to_str().unwrap());
}
