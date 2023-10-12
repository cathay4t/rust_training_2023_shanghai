## Task 1: Sort a vector of IPv4 address.

Give vector of string with "127.0.0.2" and "127.0.0.1",
Expected output is sorted Vec<Ipv4Addr>

## Task 2: Implement sort and dedup on custom struct with IPv4 address

```rust
struct HostInfo {
    name: String,
    ip: Ipv4Addr,
}


HostInfo::new(name: String, ip: Ipv4Addr);

fn main() {
    let mut a = vec![HostInfo::new("hostb", Ipv4Addr::new(127, 0, 0, 1))
        HostInfo::new("hostb", Ipv4Addr::new(127, 0, 0, 1)),
        HostInfo::new("hosta", Ipv4Addr::new(127, 0, 0, 1)),
        HostInfo::new("hostc", Ipv4Addr::new(129, 0, 0, 1))];

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
    let mut a = vec![HostInfo::new("hostb", "::1".parse().unwrap()),
        HostInfo::new("hostb", "127.0.0.1".parse().unwrap()),
        HostInfo::new("hosta", "::2".parse().unwrap()),
        HostInfo::new("hostc", "127.0.0.2".parse().unwrap())];

    // Sort by address first(prefer IPv6), then name.
    a.dedup();
    a.sort_unstable();
    println!("HAHA {:?}", a);
}
```

## Task 4: Server Client TCP


