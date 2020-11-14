use libhelper::*;
//use libhelper::helper::type_of;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
//use serde_json::{Result, Value};


pub fn test()  {
    print!("\n------------ {} ------------\n", function!());

    print!("\n1\n");
    {
        #[derive(Debug, Serialize, Deserialize)]
        struct Subobj {
            name: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Person {
            name: String,
            age: u8,
            married: bool,
            subobj: Subobj,
            subobj_map: HashMap<String, Subobj>,
            subobj_vec: Vec<Subobj>,
        }

        let data = r#"
            {
                "name": "John Doe",
                "age": 43,
                "married": false,
                "subobj": {
                    "name": "Test"
                },
                "subobj_map": {
                    "A": {"name": "A"},
                    "B": {"name": "B"}
                },
                "subobj_vec": [
                   {"name": "A"},
                   {"name": "B"}
                ]
            }"#;

        let p: Person = serde_json::from_str(&data).unwrap();
        print!("Convert json string to struct\n\t{:?}\n", p);
        
       // let p2: serde_json::Value = serde_json::from_str(&data).unwrap();
       // print!("Convert json string to Value type_of:{}\n\t{:?}\n", type_of(&p2), p2);
        
        //let s: String = serde_json::to_string(&p).unwrap();
        let s: String = serde_json::to_string_pretty(&p).unwrap();
        println!("Convert struct to json string:\n{}\n", s);


        //error: give wrong name1 -> Panic
        /*
        let data = r#"
            {
                "name1": "John Doe",
                "age": 43,
                "married": false,
                "subobj": {
                    "name": "Test"
                },
                "subobj_map": {
                    "A": {"name": "A"},
                    "B": {"name": "B"}
                },
                "subobj_vec": [
                   {"name": "A"},
                   {"name": "B"}
                ]
            }"#;
        */
        //let p: Person = serde_json::from_str(&data).unwrap();
        //print!("Convert json string to struct\n\t{:?}\n", p);

        //error: leak name filed -> Panic
        /*
        let data = r#"
            {
                "age": 43,
                "married": false,
                "subobj": {
                    "name": "Test"
                },
                "subobj_map": {
                    "A": {"name": "A"},
                    "B": {"name": "B"}
                },
                "subobj_vec": [
                   {"name": "A"},
                   {"name": "B"}
                ]
            }"#;
        */
        //let p: Person = serde_json::from_str(&data).unwrap();
        //print!("Convert json string to struct\n\t{:?}\n", p);

        //error: upper cast for name filed -> Panic
        /*
        let data = r#"
            {
                "Name": "John Doe",
                "age": 43,
                "married": false,
                "subobj": {
                    "name": "Test"
                },
                "subobj_map": {
                    "A": {"name": "A"},
                    "B": {"name": "B"}
                },
                "subobj_vec": [
                   {"name": "A"},
                   {"name": "B"}
                ]
            }"#;
        */
        //let p: Person = serde_json::from_str(&data).unwrap();
        //print!("Convert json string to struct\n\t{:?}\n", p);
        

        //empty subobj_map and subobj_vec
        let data = r#"
            {
                "name": "John Doe",
                "age": 43,
                "married": false,
                "subobj": {
                    "name": "Test"
                },
                "subobj_map": {},
                "subobj_vec": []
            }"#;
        let p: Person = serde_json::from_str(&data).unwrap();
        print!("Convert json string to struct\n\t{:?}\n", p);

    }
    

    log!("done");
}

