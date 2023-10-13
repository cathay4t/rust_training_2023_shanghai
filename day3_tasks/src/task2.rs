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

fn is_subnet(
    subset: &HashMap<String, String>,
    super_set: &HashMap<String, String>,
) -> bool {
    subset.iter().all(|(k, v)| super_set.get(k) == Some(v))
}

fn main() {
    let mut src = HashMap::new();
    src.insert("A".to_string(), "src".to_string());
    src.insert("B".to_string(), "src".to_string());
    let mut dst = HashMap::new();
    dst.insert("A".to_string(), "dst".to_string());
    dst.insert("C".to_string(), "dst".to_string());

    let new_hash = merged_hash(&src, &dst);
    println!("HAHA {:?}", new_hash);

    println!("src is subset of new: {}", is_subnet(&src, &new_hash));
    println!("dst is subset of new: {}", is_subnet(&dst, &new_hash));
}
