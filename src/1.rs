use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    map.insert("key3", "value3");

    for (k, v) in &map {
        println!("{}: {}", k, v);
    }
}
