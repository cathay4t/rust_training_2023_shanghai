// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

fn merged_hash(
    src: &HashMap<String, String>,
    dst: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut ret = dst.clone();
    src.iter().for_each(|(k, v)| {
        ret.insert(k.clone(), v.clone());
    });
    ret
}

fn main() {
    let mut src = HashMap::new();
    src.insert("A".to_string(), "src".to_string());
    src.insert("B".to_string(), "src".to_string());
    let mut dst = HashMap::new();
    dst.insert("A".to_string(), "dst".to_string());
    dst.insert("C".to_string(), "dst".to_string());

    println!("HAHA {:?}", merged_hash(&src, &dst));
}
