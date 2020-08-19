use crate::log;

fn vector_test_1() {
    let mut str_vec: Vec<String> = Vec::new();
    let str1 = String::from("CBA");

    str_vec.push(str1);
    //error[E0382]: borrow of moved value: `str1`
    //log!("{}\n", str1);
    str_vec.pop();

    for i in 0..10 {
        str_vec.push(String::from("ABC_") + &i.to_string());
    }

    //use i .. len 
    print!("\n#### for in i ####\n");
    for i in 0..str_vec.len() {
        log!("{}\n", str_vec[i]);
    }

    //use iter
    print!("\n#### for in iter ####\n");
    for i in str_vec.iter() {
        log!("{}\n", *i);
    }
    
    //use iter_mut if need modification
    print!("\n#### for in iter_mut ####\n");
    for i in str_vec.iter_mut() {
        *i = (*i).clone() + &String::from("_ABC");
        log!("{}\n", *i);
    }

    print!("\n#### move ownership from str_vec to str_vec. it would be like std::move in C++11 ####\n");
    let mut str_vec2 = str_vec;
    //error[E0382]: borrow of moved value: `str_vec`
    //for i in str_vec.iter() {
    //    log!("{}\n", *i);
    //}

    for i in str_vec2.iter() {
        log!("{}\n", *i);
    }
    
    print!("\n#### remove iteam that index of 0,2,4 ####\n");
    str_vec2.remove(0);
    str_vec2.remove(1);
    str_vec2.remove(2);
    
    for i in str_vec2.iter() {
        log!("{}\n", *i);
    }

    print!("\n#### pop iteam of 9 ####\n");
    let v:Option<String> = str_vec2.pop();
  
    if v.is_none() == false {
        log!("Poped value is {}\n", v.unwrap());
    }
    for i in str_vec2.iter() {
        log!("{}\n", *i);
    }

    log!("len of str_vec2 {}\n", str_vec2.len());
    
    
    print!("\n#### inital vec<&str> by using macro vec! ####\n");
    let s = vec!["ABC", "DEFG", "HIJK"];
    for i in s.iter() {
        log!("{} {}\n", *i, crate::helper::type_of(&(*i)));
    }
}

pub fn test() {
    crate::helper::help();
    log!("Hello {}\n", 1);
    vector_test_1();
}
