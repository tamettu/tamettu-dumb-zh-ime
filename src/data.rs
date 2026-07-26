use std::collections::{BTreeMap, HashMap};

// enum Strokes {
//     q, // 丶
//     w, // 一
//     e, // 丨
//     r, // 丿
//     t, // ㇏
//     y, // ㇀
//     s, // 亅
//     a, // ㇟
// }
fn import_map() -> HashMap<String, BTreeMap<char, i8>> {
    let json_data = include_str!("../assets/stroke_data.json");
    let stroke_map: HashMap<String, String> =
        serde_json::from_str(json_data).expect(".json err, fk go to check it");
    let mut stroke_map_zip: HashMap<String, BTreeMap<char, i8>> = HashMap::new();

    for (c, s) in stroke_map {
        stroke_map_zip.insert(c, to_count_map(&s));
    }
    stroke_map_zip
}
pub fn reverse_map() -> Vec<(BTreeMap<char, i8>, Vec<String>)> {
    let mut new_map: HashMap<BTreeMap<char, i8>, Vec<String>> = HashMap::new();
    for (k, v) in import_map() {
        new_map.entry(v).or_default().push(k);
    }
    let res: Vec<(BTreeMap<char, i8>, Vec<String>)> = new_map.into_iter().collect();
    res
}
fn to_count_map(s: &str) -> BTreeMap<char, i8> {
    let mut map = BTreeMap::new();
    for c in s.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    map
}
