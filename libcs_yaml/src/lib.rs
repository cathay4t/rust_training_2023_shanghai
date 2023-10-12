// SPDX-License-Identifier: Apache-2.0

mod cli;
mod error;
mod info;
mod logger;

use std::ffi::{c_char, CString};
use std::time::SystemTime;

use once_cell::sync::OnceCell;

use crate::cli::report_time;
use crate::error::{CsYamlError, ErrorKind};
use crate::logger::MemoryLogger;

const CS_YAML_PASS: u32 = 0;
const CS_YAML_FAIL: u32 = 1;
const CS_YAML_FAIL_NULL_POINTER: u32 = 2;

static MEM_LOGGER: OnceCell<MemoryLogger> = OnceCell::new();

#[no_mangle]
pub extern "C" fn cs_yaml_report_time(
    log: *mut *mut c_char,
    err_msg: *mut *mut c_char,
) -> u32 {
    if log.is_null() || err_msg.is_null() {
        return CS_YAML_FAIL_NULL_POINTER;
    }

    unsafe {
        let _ = Box::into_raw(Box::new([0u8; 7]));
    }

    unsafe {
        *log = std::ptr::null_mut();
        *err_msg = std::ptr::null_mut();
    }

    let logger = match init_logger() {
        Ok(l) => l,
        Err(e) => {
            unsafe {
                *err_msg =
                    CString::new(format!("Failed to setup logger: {}", e))
                        .unwrap()
                        .into_raw();
            }
            return CS_YAML_PASS;
        }
    };
    let now = SystemTime::now();

    let result = report_time();

    unsafe {
        *log = CString::new(logger.drain(now)).unwrap().into_raw();
    }

    match result {
        Ok(()) => CS_YAML_PASS,
        Err(e) => {
            unsafe {
                *err_msg = CString::new(e.msg).unwrap().into_raw();
            }
            CS_YAML_FAIL
        }
    }
}

#[no_mangle]
pub extern "C" fn cs_yaml_cstring_free(cstring: *mut c_char) {
    unsafe {
        if !cstring.is_null() {
            let _ = CString::from_raw(cstring);
        }
    }
}

fn init_logger() -> Result<&'static MemoryLogger, CsYamlError> {
    match MEM_LOGGER.get() {
        Some(l) => {
            l.add_consumer();
            Ok(l)
        }
        None => {
            if MEM_LOGGER.set(MemoryLogger::new()).is_err() {
                return Err(CsYamlError::new(
                    ErrorKind::Bug,
                    "Failed to set once_sync for logger".to_string(),
                ));
            }
            if let Some(l) = MEM_LOGGER.get() {
                if let Err(e) = log::set_logger(l) {
                    Err(CsYamlError::new(
                        ErrorKind::Bug,
                        format!("Failed to log::set_logger: {}", e),
                    ))
                } else {
                    l.add_consumer();
                    log::set_max_level(log::LevelFilter::Debug);
                    Ok(l)
                }
            } else {
                Err(CsYamlError::new(
                    ErrorKind::Bug,
                    "Failed to get logger from once_sync".to_string(),
                ))
            }
        }
    }
}
