use std::collections::HashMap;
use crate::log;

fn hashmap_test_1() {
    let mut color_hex_map: HashMap<String,u32> = HashMap::new();

    color_hex_map.insert(String::from("yellow"),    0x00FFFF00);
    color_hex_map.insert(String::from("red"),       0x00FF0000);
    color_hex_map.insert(String::from("blue"),      0x000000FF);
    color_hex_map.insert(String::from("green"),     0x00008000);
    color_hex_map.insert(String::from("black"),     0xFFFFFFFF);
    
    //update
    color_hex_map.insert(String::from("black"),     0x00000000);
    
    //insert if key has no value
    color_hex_map.entry(String::from("white")).or_insert(0x00FFFFFF);

    //get
    let yellow = color_hex_map.get(&String::from("yellow"));
    if yellow.is_none() == false {
        log!("found yellow {}\n", format!("0x{:08X}", yellow.unwrap()));
    }

    //remove
    color_hex_map.remove("yellow");

    //check only
    if color_hex_map.contains_key("yellow") {
        log!("found yellow \n");
    } else {
        log!("not found of yellow \n");
    }

    //dump key, val
    for (key, val) in &color_hex_map {
        log!("{} {}\n", key, format!("0x{:08X}", val));
    }
}

pub fn test() {
    log!("Hello {}\n", 1);
    hashmap_test_1();
}
