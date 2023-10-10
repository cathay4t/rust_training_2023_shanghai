// SPDX-License-Identifier: Apache-2.0

use std::ffi::CString;
use std::io::Write;
use std::net::TcpStream;

use crate::error::CsYamlError;
use crate::info::CliInfo;

fn get_hostname() -> String {
    match nix::unistd::gethostname() {
        Ok(hostname_cstr) => match hostname_cstr.to_str() {
            Some(h) => h.to_string(),
            None => String::new(),
        },
        Err(_) => String::new(),
    }
}

pub(crate) fn report_time() -> Result<(), CsYamlError> {
    let mut stream = TcpStream::connect("127.0.0.1:8766")?;
    log::info!("TCP session connected {:?}", stream);
    let time = chrono::Local::now();
    let cli_info = CliInfo {
        time,
        hostname: get_hostname(),
        ..Default::default()
    };
    log::debug!("CliInfo prepared {:?}", cli_info);
    let cli_info_yml_cs =
        CString::new(serde_yaml::to_string(&cli_info)?.as_str())?;

    stream.write(&cli_info_yml_cs.as_bytes_with_nul().len().to_be_bytes())?;
    stream.write(cli_info_yml_cs.as_bytes_with_nul())?;
    log::info!("CliInfo sent");
    Ok(())
}
