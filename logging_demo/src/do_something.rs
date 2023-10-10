// SPDX-License-Identifier: Apache-2.0

pub(crate) struct DoSomething {}

impl DoSomething {
    pub(crate) fn run() {
        log::debug!("Debug 1");
        log::info!("Info 1");
        log::warn!("Warn 1");
        log::error!("Error 1");
    }
}
