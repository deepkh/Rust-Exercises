use libhelper::*;
use libhelper::helper::type_of;



/****************************
 * SomeData
 ****************************/
struct SomeData {
    name: String,
    age: i32,
}

impl SomeData {
    pub fn new(name: &str, age: i32) -> SomeData {
        SomeData {
            name: String::from(name),
            age,
        }
    }
    
    pub fn dump(&self) {
        log!("{} {}\n", self.name, self.age);
    }
}

/****************************
 * SomeData2
 ****************************/
#[derive(Debug)]
struct SomeData2 {
    name: String,
    age: i32,
}

impl SomeData2 {
    pub fn new(name: &str, age: i32) -> SomeData2 {
        SomeData2 {
            name: String::from(name),
            age,
        }
    }
    
}

impl Clone for SomeData2 {
    fn clone(&self) -> Self {
        SomeData2 {
            name: self.name.clone(),
            age: self.age,
        }
    }
}

/****************************
 * SomeData3
 ****************************/
#[derive(Debug, Clone)]
struct SomeData3 {
    name: String,
    age: i32,
}

impl SomeData3 {
    pub fn new(name: &str, age: i32) -> SomeData3 {
        SomeData3 {
            name: String::from(name),
            age,
        }
    }
    
    pub fn dump(&self) {
        log!("{} {}\n", self.name, self.age);
    }
}



fn box_test() {
    print!("\n------------ {} ------------\n", function!());
    {
        let p = Box::new(10); 
        let mut p2 = p;
        
        log!("type_of: {}\n", type_of(&p2));
        *p2 = 999;
        log!("*p2:{} p2:{}\n", *p2, p2);
        
        let p3 = p2;
        //log!("{}\n", p2);                 //error[E0382]: borrow of moved value: `p2`
        
        let p4 = p3.clone();
        log!("p3:{} p4:{}\n", p3, p4);      //deep clone p3 data into p4
    }
    
    {
        let p = Box::new(SomeData::new("Marty Beard", 32)); 
        log!("{}\n", type_of(&p));          //alloc::boxed::Box<playground::SomeData>
        p.dump();
        //let p2 = p.clone();                 //error[E0599]: no method named `clone` found for struct `std::boxed::Box<SomeData>` in the current scope
    }
    
    {//clone test by manually implement
        let p = Box::new(SomeData2::new("Marty Beard2", 33)); 
        let p2 = p.clone();
        log!("{} {}\n", type_of(&p), type_of(&p2));          //alloc::boxed::Box<playground::SomeData2> alloc::boxed::Box<playground::SomeData2>
        print!("{:?} {:?}\n", p, p2);
    }
    
    {//clone test by #[derive(Clone)]
        let p = Box::new(SomeData3::new("Marty Beard3", 34)); 
        let p2 = p.clone();
        log!("{} {}\n", type_of(&p), type_of(&p2));          //alloc::boxed::Box<playground::SomeData2> alloc::boxed::Box<playground::SomeData2>
        print!("{:?} {:?}\n", p, p2);
    }
    
    {//clone test by #[derive(Clone)]
        let p = SomeData3::new("Marty Beard4", 35); 
        let p2 = p.clone();
        log!("{} {}\n", type_of(&p), type_of(&p2));          //playground::SomeData3 playground::SomeData3
        print!("{:?} {:?}\n", p, p2);
    }
}

fn option_box_test()
{
    print!("\n------------ {} ------------\n", function!());
    {
        let p: Option<Box<SomeData3>> = Some(Box::new(SomeData3::new("Super Marty Beard", 36)));
        let mut p2: Option<Box<SomeData3>> = None;
        let mut p3: Option<Box<SomeData3>> = None;
        let mut p4: Option<Box<SomeData3>> = None;
        
        log!("{} \n", type_of(&p));                 //core::option::Option<alloc::boxed::Box<playground::SomeData3>>
        
        if p.is_some() {
            p2 = p.clone();
            p3 = p.clone();
            p4 = p.clone();
            log!("{}\n", type_of(&(p.as_ref())));   //core::option::Option<&alloc::boxed::Box<others::box_::SomeData3>>
                                                    //                     ^ as_ref would be something different
            p.as_ref().unwrap().dump();              
            p.unwrap().dump();              
            //p.unwrap().dump();             //error[E0382]: use of moved value: `p`  
        }

        if let Some(v) = p2 {
            v.dump();
        }

        let vv = match p3 {
            Some(v) => v,
            None => return,
        };

        log!("{} \n", type_of(&vv));          //alloc::boxed::Box<others::box_::SomeData3>
        vv.dump();

        match p4 {
            Some(v) => v.dump(),//log!("have value\n"),
            None => (print!("is none\n")),
        };
    }
    
    {
        let p: Option<Box<SomeData3>> = None;
        if p.is_none() {
            log!("p is null\n");
        }
        //p.unwrap().dump();          //this will cause panic       
    }

    /*
    {
        let p: Option<Box<SomeData3>> = None;
        //p.expect("GG 15FF").dump();          //this will cause panic with specified 'GG 15FF' message 
    }
    */
}

//option with as_mut for write purpose
fn change_name(p: Option<&mut Box<SomeData3>>) -> std::result::Result<(), String>
{
    if p.is_none() {
        return Err("p is null".to_string());
    }

    p.unwrap().name = format!("Marty Marty super Beard");
    Ok(())
}

//option with as_ref for read only purpose
fn print_name(p: Option<&Box<SomeData3>>) -> std::result::Result<(), String>
{
    if p.is_none() {
        return Err("p is null".to_string());
    }

    log!("new name is '{}'\n", p.unwrap().name);
    Ok(())
}

/**
 * due to Rust doesnt have null pointer. 
 * So need use Option with Box to check if there have pointer address or not
 * The box would very similar C++'s
    let p = Box::new(10); // 即 C++14 的 auto p = make_unique<int>(10);
                          // 或 C++11 的 auto p = unique_ptr<int>(new int(10));
 */
pub fn test()  {
    box_test();
    option_box_test();

    {
        let p: Box<SomeData3> = Box::new(SomeData3::new("Nice Marty Beard", 40));
        let mut p2 = p.clone();

        p2.name = p2.name.chars().rev().collect::<String>();
        print!("{:?} {:?}\n", p, p2);
    }

    {
        let mut p: Box<Vec<String>> = Box::new(Vec::new());
        p.as_mut().push("A1".to_string());
        p.as_mut().push("A2".to_string());
        p.as_mut().push("A3".to_string());

        let p2 = p.clone();         //deep clone data to a new address
        p.as_mut()[0] = "B1".to_string();
        
        print!("{:?} \n", p);           //["B1", "A2", "A3"]
        print!("{:?} \n", p2);          //["A1", "A2", "A3"]

    }

    {
        let mut p:Option<Box<SomeData3>> = Some(Box::new(SomeData3::new("Super Super Marty Beard", 37)));

        if let Err(s) = change_name(p.as_mut()) {
            log!("{}\n", s);
            return;
        }

       print_name(p.as_ref()).unwrap(); 
    }
}

