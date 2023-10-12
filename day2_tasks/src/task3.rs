// SPDX-License-Identifier: Apache-2.0

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug, PartialEq, Eq)]
struct HostInfo {
    name: String,
    ip: IpAddr,
}

impl HostInfo {
    fn new(name: String, ip: IpAddr) -> Self {
        Self { name, ip }
    }

    fn sort_key(&self) -> (bool, &IpAddr, &str) {
        (self.ip.is_ipv4(), &self.ip, self.name.as_str())
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
        HostInfo::new("hostb".into(), "::1".parse().unwrap()),
        HostInfo::new("hostb".into(), "127.0.0.1".parse().unwrap()),
        HostInfo::new("hosta".into(), "::2".parse().unwrap()),
        HostInfo::new("hostc".into(), "127.0.0.2".parse().unwrap()),
    ];

    // Sort by IP address first, then name.
    a.sort_unstable();
    a.dedup();
    println!("HAHA {:?}", a);
}
