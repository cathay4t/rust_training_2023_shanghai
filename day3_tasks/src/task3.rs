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

fn is_subnet_nested(
    subset: &HashMap<String, HashMap<String, String>>,
    super_set: &HashMap<String, HashMap<String, String>>,
) -> bool {
    subset.iter().all(|(k, v)| {
        super_set
            .get(k)
            .map(|sup_v| is_subnet(v, sup_v))
            .unwrap_or_default()
    })
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

    let mut sub_src = HashMap::new();
    sub_src.insert("section_a".to_string(), src);
    let mut sub_dst = HashMap::new();
    sub_dst.insert("section_a".to_string(), dst);
    let mut sup_h = HashMap::new();
    sup_h.insert("section_a".to_string(), new_hash);

    println!(
        "src is subset of new: {}",
        is_subnet_nested(&sub_src, &sup_h)
    );
    println!(
        "dst is subset of new: {}",
        is_subnet_nested(&sub_dst, &sup_h)
    );
}
