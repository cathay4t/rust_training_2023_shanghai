// SPDX-License-Identifier: Apache-2.0

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

fn main() {
    let mut map_a: HashMap<String, String> = HashMap::new();
    let mut bmap_a: BTreeMap<String, String> = BTreeMap::new();
    let mut set_a: HashSet<String> = HashSet::new();
    let mut bset_a: BTreeSet<String> = BTreeSet::new();

    for i in 1..10 {
        map_a.insert(i.to_string(), i.to_string());
        bmap_a.insert(i.to_string(), i.to_string());
        set_a.insert(i.to_string());
        set_a.insert(i.to_string());
        bset_a.insert(i.to_string());
        bset_a.insert(i.to_string());
    }

    for (k, v) in map_a.iter() {
        println!("{k}:{v}");
    }

    let keys: Vec<String> = map_a.keys().cloned().collect();
    let keys_ref: Vec<&str> = map_a.keys().map(String::as_str).collect();

    println!("HashMap Keys {:?}", keys);
    println!("HashMap Keys by ref {:?}", keys_ref);

    let keys: Vec<String> = bmap_a.keys().cloned().collect();
    let keys_ref: Vec<&str> = bmap_a.keys().map(String::as_str).collect();

    println!("BTreeMap Keys {:?}", keys);
    println!("BTreeMap Keys by ref {:?}", keys_ref);

    let keys: Vec<String> = set_a.drain().collect();
    let keys_ref: Vec<&str> = set_a.iter().map(String::as_str).collect();

    println!("HashSet Keys {:?}", keys);
    println!("BTreeMap Keys by ref {:?}", keys_ref);

    let keys: Vec<String> = bset_a.iter().cloned().collect();
    bset_a.retain(|i| i == "1");
    let keys_ref: Vec<&str> = bset_a.iter().map(String::as_str).collect();

    println!("HashSet Keys {:?}", keys);
    println!("BTreeMap Keys by ref {:?}", keys_ref);
}
