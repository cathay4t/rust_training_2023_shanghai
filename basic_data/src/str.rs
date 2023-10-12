// SPDX-License-Identifier: Apache-2.0

fn main() {
    let str_a = "abc";
    let part_a = &str_a[1..];
    println!("HAHA {:?}", part_a);
    println!("HAHA {:?}", part_a.len());
    let mut string_a = "ab".to_string();
    string_a.push('c');

    let str_b: &str = string_a.as_str();

    let opt_a = Some(string_a.clone());

    assert_eq!(str_a, str_b);
    assert_eq!(str_a, string_a);
    assert_eq!(Some("abc"), opt_a.as_deref());
    println!("PASS");
}
