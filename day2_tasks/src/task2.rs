// SPDX-License-Identifier: Apache-2.0

use std::net::Ipv4Addr;

#[derive(Debug, PartialEq, Eq)]
struct HostInfo {
    name: String,
    ip: Ipv4Addr,
}

impl HostInfo {
    fn new(name: String, ip: Ipv4Addr) -> Self {
        Self { name, ip }
    }

    fn sort_key(&self) -> (&Ipv4Addr, &str) {
        (&self.ip, self.name.as_str())
    }
}

impl PartialOrd for HostInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HostInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.sort_key().cmp(&other.sort_key())
    }
}

fn main() {
    let mut a = vec![
        HostInfo::new("hostb".into(), Ipv4Addr::new(127, 0, 0, 1)),
        HostInfo::new("hostb".into(), Ipv4Addr::new(127, 0, 0, 1)),
        HostInfo::new("hosta".into(), Ipv4Addr::new(127, 0, 0, 2)),
        HostInfo::new("hostc".into(), Ipv4Addr::new(129, 0, 0, 1)),
    ];

    // Sort by IP address first, then name.
    a.sort_unstable();
    a.dedup();
    println!("HAHA {:?}", a);
}
