// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, Default)]
struct AbcClient {
    _data: [u8; 7],
}

fn invoke_c_func(ptr: *mut AbcClient) {
//    drop_client(ptr);
}

fn main() {
    let cli = AbcClient::default();

    let ptr = unsafe { Box::into_raw(Box::new(cli)) };

    invoke_c_func(ptr);

}

fn drop_client(ptr: *mut AbcClient) {
    unsafe {
        let _ = Box::from_raw(ptr);
    }
}
