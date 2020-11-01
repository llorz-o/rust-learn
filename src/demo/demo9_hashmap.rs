use std::collections::HashMap;

pub fn run() {
    let mut map = HashMap::new();
    map.insert(String::from("key"), "value"); // 所有的键必须是相同类型，值也必须都是相同类型。

    // 元组转 HashMap
    let keys = vec![String::from("id"), String::from("name")];
    let values = vec![String::from("01"), String::from("jojo")];

    // 下划线占位 HashMap 根据元组的数据类型推断
    let mut map: HashMap<_, _> = keys.iter().zip(values.iter()).collect(); // 元组对转HashMap

    // HashMap 将获得引用类型的所有权
    let map_key = String::from("id");

    if let Option::Some(key) = map.get(&map_key) {
        println!("if let value:{}", key)
    }
    match map.get(&map_key) {
        Option::Some(key) => println!("match value:{}", key),
        _ => (),
    }
    let key = String::from("id");
    let value = String::from("02");

    map.insert(&key, &value); // 更新值
    map.entry(&key).or_insert(&value); // 当前键对应的值不存在则插入

    // 顺序将是随机的
    for (key, value) in &map {
        println!("key:{}  value:{}", key, value);
    }
}
