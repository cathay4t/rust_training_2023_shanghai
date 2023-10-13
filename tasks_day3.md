## Task 1: Merged two HashMap

 * Given two `HashMap<String, String>`,
 * When invoke `merge_hash(src, dst)`,
 * Then return __new__ `HashMap` with data found in src or dst, prefer src.


## Task 2: Subset check of HashMap

 * Give two `HashMap<String, String>`,
 * When invoke `is_subset(hashmap_a, hashmap_b)`,
 * Then return true if `hashmap_a` is subset of `hashmap_b` and false
   otherwise.

## Task 3: Subset check of Nested HashMap

 * Give two `HashMap<String, HashMap<String, String>`,
 * When invoke `is_subset(hashmap_a, hashmap_b)`,
 * Then return true if `hashmap_a` is subset of `hashmap_b` and false
   otherwise.

## Task 4: Deserialize HashMap with sorted order

 * Given a custom struct holding HashMap as a property,
 * When invoke `serde_yaml::to_string(&obj)`,
 * Then the keys in output YAML is sorted without changing struct member type.
