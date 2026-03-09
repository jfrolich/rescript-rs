#![allow(dead_code)]

use std::collections::{BTreeMap, HashMap, HashSet};

use rescript_rs::{Config, TS};

#[derive(TS)]
#[rescript(export, export_to = "hashmap/")]
struct Hashes {
    map: HashMap<String, String>,
    set: HashSet<String>,
}

#[test]
fn hashmap() {
    let cfg = Config::from_env();
    assert_eq!(
        Hashes::decl(&cfg),
        "type hashes = {\n  map: Dict.t<string>,\n  set: array<string>,\n}"
    )
}

struct CustomHasher {}

type CustomHashMap<K, V> = HashMap<K, V, CustomHasher>;
type CustomHashSet<K> = HashSet<K, CustomHasher>;

#[derive(TS)]
#[rescript(export, export_to = "hashmap/")]
struct HashesHasher {
    map: CustomHashMap<String, String>,
    set: CustomHashSet<String>,
}

#[test]
fn hashmap_with_custom_hasher() {
    let cfg = Config::from_env();
    assert_eq!(
        HashesHasher::decl(&cfg),
        "type hashesHasher = {\n  map: Dict.t<string>,\n  set: array<string>,\n}"
    )
}

#[derive(TS, Eq, PartialEq, Hash)]
#[rescript(export, export_to = "hashmap/")]
struct CustomKey(String);

#[derive(TS)]
#[rescript(export, export_to = "hashmap/")]
struct CustomValue;

#[derive(TS)]
#[rescript(export, export_to = "hashmap/")]
struct HashMapWithCustomTypes {
    map: HashMap<CustomKey, CustomValue>,
}

#[derive(TS)]
#[rescript(export, export_to = "hashmap/")]
struct BTreeMapWithCustomTypes {
    map: BTreeMap<CustomKey, CustomValue>,
}

#[derive(TS)]
#[rescript(export, export_to = "hashmap/")]
enum EnumKey {
    Foo,
    Bar,
}

#[test]
fn with_custom_types() {
    let cfg = Config::from_env();
    assert_eq!(
        HashMapWithCustomTypes::inline(&cfg),
        BTreeMapWithCustomTypes::inline(&cfg)
    );
    assert_eq!(
        HashMapWithCustomTypes::decl(&cfg),
        "type hashMapWithCustomTypes = {\n  map: Dict.t<customValue>,\n}"
    );
    assert_eq!(HashMap::<EnumKey, String>::name(&cfg), "Dict.t<string>");
    assert_eq!(HashMap::<EnumKey, String>::inline(&cfg), "Dict.t<string>");
}
