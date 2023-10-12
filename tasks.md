## Task 1: Sort a vector of IPv4 address.

Give vector of string with "127.0.0.2", "127.0.0.1" and "127.0.0.1",
Expected output is sorted Vec<Ipv4Addr>

## Task 2: Implement sort and dedup on custom struct with IPv4 address

```rust
struct HostInfo {
    name: String,
    ip: Ipv4Addr,
}


HostInfo::new(name: String, ip: Ipv4Addr);

fn main() {
    let mut a = vec![HostInfo::new("hostb".into(), Ipv4Addr::new(127, 0, 0, 1)),
        HostInfo::new("hostb".into(), Ipv4Addr::new(127, 0, 0, 1)),
        HostInfo::new("hosta".into(), Ipv4Addr::new(127, 0, 0, 1)),
        HostInfo::new("hostc".into(), Ipv4Addr::new(129, 0, 0, 1))];

    // Sort by IP address first, then name.
    a.dedup();
    a.sort_unstable();
    println!("HAHA {:?}", a);
}
```

## Task 3: Implement sort and dedup on custom struct with IPv6 or IPv4 address.

```rust
struct HostInfo {
    name: String,
    ip: IpAddr,
}
impl HostInfo {
    fn new(name: String, ip: IpAddr) -> Self ;
}

fn main() {
    let mut a = vec![HostInfo::new("hostb".into(), "::1".parse().unwrap()),
        HostInfo::new("hostb".into(), "127.0.0.1".parse().unwrap()),
        HostInfo::new("hosta".into(), "::2".parse().unwrap()),
        HostInfo::new("hostc".into(), "127.0.0.2".parse().unwrap())];

    // Sort by address first(prefer IPv6), then name.
    a.dedup();
    a.sort_unstable();
    println!("HAHA {:?}", a);
}
```

## Task 4: Server Client TCP


