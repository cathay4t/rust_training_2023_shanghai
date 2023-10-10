// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

#[derive(Debug, Default)]
struct Abc {
    a: Option<Bcd>,
}

#[derive(Debug, Default)]
struct Bcd {
    b: Option<String>,
}

fn write_abc(abc: &mut Abc) {
    if let Some(b) = abc.a.as_mut() {
        b.b = Some("abc".into());
    } else {
        abc.a = Some(Bcd {
            b: Some("abc".into()),
        });
    }
}

fn main() {
    let mut abc = Abc::default();
    write_abc(&mut abc);
    println!("Updated {:?}", abc);

    let mut map: HashMap<&str, u32> = HashMap::new();

    if let Some(value) = match map.get_mut("abc") {
        Some(v) => Some(v),
        None => {
            map.insert("abc", 3);
            map.get_mut("abc")
        }
    } {
        *value *= 2;
    }

    // *map.entry("abc").or_insert(3) *= 2;
    assert_eq!(map["abc"], 6);

    println!("Map {:?}", map);

    for v in map.values_mut() {
        *v *= 2;
    }

    println!("Map {:?}", map);

    let opt_vec: Vec<Option<String>> = vec![Some("a".to_string()), None, None];

    println!("Old vec {:?}", opt_vec);

    let new_vec: Vec<&str> = opt_vec
        .iter()
        .filter_map(|i| i.as_ref().map(|i| i.as_str()))
        .collect();

    println!("New vec {:?}", new_vec);
}
