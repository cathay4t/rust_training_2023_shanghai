// SPDX-License-Identifier: Apache-2.0

mod do_something;

use crate::do_something::DoSomething;

fn main() {
    //init_env_logger();
    init_syslog_logger();
    DoSomething::run();
}

fn init_env_logger() {
    let mut log_builder = env_logger::Builder::new();
    log_builder.filter(None, log::LevelFilter::Debug);
    log_builder.init();
}

fn init_syslog_logger() {
    let formatter = syslog::Formatter3164 {
        facility: syslog::Facility::LOG_USER,
        hostname: None,
        process: "mzc".into(),
        pid: 0,
    };

    let logger = syslog::unix(formatter).expect("could not connect to syslog");
    log::set_boxed_logger(Box::new(syslog::BasicLogger::new(logger)))
        .map(|()| log::set_max_level(log::LevelFilter::Debug))
        .expect("Failed to setup syslog logger");
}
