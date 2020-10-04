use libhelper::*;
use libhelper::helper::type_of;
use crate::ErrStack;
use std::fs::File;
use std::io;
use std::io::{Error,ErrorKind};
use std::io::prelude::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

#[derive(Debug)]        
struct Data3 {
    a: i32,
    b: i32,
}

//Box borrow & deref can
//  1. replace the Data3 to new instance
//Box borrow also can
//  1. change value of the Data3 member variable 
fn box_deref(p: &mut Box<Data3>) {
    *p = Box::new(Data3{a:369, b:123});
}

//Box as_mut can
//  1. change value of the Data3 member variable
fn box_asmut(p: &mut Data3) {
    p.a = 333;
}

pub fn test() {
    let mut case_i = 0;
    print!("\n------------ {} ------------\n", function!());

    // References for Box
    // https://electronic.blue/blog/2017/04/09-rust-an-introduction-smart-pointer/
    
        {
            let mut a:Box<Data3> = Box::new(Data3{a:123, b:456});
            box_deref(&mut a);
            print!("a:{:?}  type_of:{}\n", a, type_of(&a));    //a1:Data { a: 123, b: 456 }  type_of:&alloc::boxed::Box<playground::main::Data>
            
            a.a = 999;
            print!("a:{:?}  type_of:{}\n", a, type_of(&a));    //a1:Data { a: 123, b: 456 }  type_of:&alloc::boxed::Box<playground::main::Data>
        }
        
        {
            let mut a:Box<Data3> = Box::new(Data3{a:123, b:456});
            box_asmut(a.as_mut());
            print!("\na:{:?}  type_of:{}\n", a, type_of(&a));    //a1:Data { a: 123, b: 456 }  type_of:&alloc::boxed::Box<playground::main::Data>
            
            a.a = 999;
            print!("a:{:?}  type_of:{}\n", a, type_of(&a));    //a1:Data { a: 123, b: 456 }  type_of:&alloc::boxed::Box<playground::main::Data>
        }
        
        //Copy: as_ref: Copy
        {
            #[derive(Debug)]        
            struct Data {
                a: i32,
                b: i32,
            }

            #[derive(Debug, Clone, Copy)]        
            struct DataG {
                a: i32,
                b: i32,
            }
            
            let mut g:Box<DataG> = Box::new(DataG{a:123, b:456});
            let mut g1 = *g;                //pass by value due to Copy already impled
            g.a = 456;  
            
            let mut a:Box<Data> = Box::new(Data{a:123, b:456});
            let a1 = &a;
            
            let mut aa:Box<Data> = Box::new(Data{a:123, b:456});
            let mut aa1 = &mut aa;
            aa1.a = 888;
            
            let mut b:Box<Data> = Box::new(Data{a:123, b:456});
            let b1 = b.as_ref();
            
            let mut bb:Box<Data> = Box::new(Data{a:123, b:456});
            let mut bb1 = bb.as_mut();
            bb1.a = 999;

            print!("\ng1:{:?}  type_of:{}\n", g1, type_of(&g1)); 
            print!("g:{:?}  type_of:{}\n", g, type_of(&g)); 
            print!("a1:{:?}  type_of:{}\n", a1, type_of(&a1)); 
            
            print!("aa1:{:?}  type_of:{}\n", aa1, type_of(&aa1));
            print!("aa:{:?}  type_of:{}\n", aa, type_of(&aa));

            print!("b1:{:?}  type_of:{}\n", b1, type_of(&b1));
            
            print!("bb1:{:?}  type_of:{}\n", bb1, type_of(&bb1));
            print!("bb:{:?}  type_of:{}\n", bb, type_of(&bb));
            bb.a = 333;
            print!("bb:{:?}  type_of:{}\n", bb, type_of(&bb));

            /*
            g1:DataG { a: 123, b: 456 }  type_of:others::box2::test::DataG
            g:DataG { a: 456, b: 456 }  type_of:alloc::boxed::Box<others::box2::test::DataG>
            a1:Data { a: 123, b: 456 }  type_of:&alloc::boxed::Box<others::box2::test::Data>
            aa1:Data { a: 888, b: 456 }  type_of:&mut alloc::boxed::Box<others::box2::test::Data>
            aa:Data { a: 888, b: 456 }  type_of:alloc::boxed::Box<others::box2::test::Data>
            b1:Data { a: 123, b: 456 }  type_of:&others::box2::test::Data
            bb1:Data { a: 999, b: 456 }  type_of:&mut others::box2::test::Data
            bb:Data { a: 999, b: 456 }  type_of:alloc::boxed::Box<others::box2::test::Data>
            bb:Data { a: 333, b: 456 }  type_of:alloc::boxed::Box<others::box2::test::Data>
            */
        }


    // Box can alloc a size of T from the heap memory.
    {
        {
            let mut p:Box<i32> = Box::new(123);
            *p = 456;
            print!("\n\n*p={:} p={} \n", *p, p);   //*p=456 p=456
        }

        // Move
        {
            let mut p:Box<i32> = Box::new(123);
            let mut p1 = p;
            
            //*p = 321;           //error[E0382]: use of moved value: `p`
            *p1 = 456;              //deref
            
            print!("\np1:{}  type_of:{}\n", p1, type_of(&p1)); //p1:123  type_of:alloc::boxed::Box<i32>
        }

        // Clone
        {
            let mut p:Box<i32> = Box::new(123);
            let mut p1 = p.clone();
            
            *p = 456;
            *p1 = 321;

            print!("\np:{}  type_of:{}\n", p, type_of(&p)); //p:456  type_of:alloc::boxed::Box<i32>
            print!("p1:{}  type_of:{}\n", p1, type_of(&p1)); //p1:321  type_of:alloc::boxed::Box<i32>
        }

        // Borrow & deref
        {
            let mut p:Box<i32> = Box::new(123);
            let p1 = &p;
            //let p2 = *p1;     //error[E0507]: cannot move out of `*p1` which is behind a shared reference
            print!("\n*p:{}  type_of:{}\n", *p, type_of(&*p)); //*p:123  type_of:i32
            print!("p:{}  type_of:{}\n", p, type_of(&p)); //p:123  type_of:alloc::boxed::Box<i32>
            print!("p1:{}  type_of:{}\n", p1, type_of(&p1)); //p1:123  type_of:&alloc::boxed::Box<i32>
        }


        //Move: Struct
        {
            #[derive(Debug)]      
            struct Data {
                a: String,
                b: i32,
            }

            let mut p:Box<Data> = Box::new(Data{a:"123".to_string(), b:456});
            let mut p1 = p;                                     //move
            
            //p.a = "321".to_string();                          //error[E0382]: assign to part of moved value: `*p`
            p1.a = "456".to_string();
            
            print!("\np1:{:?}  type_of:{}\n", p1, type_of(&p1)); //p1:Data { a: 456, b: 123 }  type_of:alloc::boxed::Box<others::box2::test::Data>
        }


        //Copy: as_ref: Copy
        {
            #[derive(Debug, Copy, Clone)]        
            struct Data {
                a: i32,
                b: i32,
            }
            
            let mut p:Box<Data> = Box::new(Data{a:123, b:456});
            let p1 = p.as_ref();                //this is borrow reference
            let p2 = *p1;                       //this is copy
            let p3 = p1.clone();
            print!("\np1:{:?}  type_of:{}\n", p1, type_of(&p1)); //p1:Data { a: 123, b: 456 }  type_of:&others::box2::test::Data
            print!("p2:{:?}  type_of:{}\n", p2, type_of(&p2));   //p2:Data { a: 123, b: 456 }  type_of:others::box2::test::Data
            print!("p3:{:?}  type_of:{}\n", p3, type_of(&p3));   //p3:Data { a: 123, b: 456 }  type_of:others::box2::test::Data
        }

        //Move: as_ref: Move
        {
            #[derive(Debug)]        
            struct Data {
                a: i32,
                b: i32,
            }
            
            let mut p:Box<Data> = Box::new(Data{a:123, b:456});
            let p1 = p.as_ref();                //this is borrow reference
            //let p2 = *p1;                     //error[E0507]: cannot move out of `*p1` which is behind a shared reference
                                                //move occurs because `*p1` has type `box2::test::Data`, which does not implement the `Copy` trait
        }

        //Copy: as_mut (mutable)
        {
            #[derive(Debug, Copy, Clone)]        
            struct Data {
                a: i32,
                b: i32,
            }
            
            let mut p:Box<Data> = Box::new(Data{a:123, b:456});
            let mut p1 = p.as_mut();                //this is borrow reference
            let mut p2 = *p1;                       //this is copy
            let mut p3 = p1.clone();

            p1.a = 456;
            p2.a = 789;
            p3.a = 012;

            print!("\np1:{:?}  type_of:{}\n", p1, type_of(&p1)); //p1:Data { a: 123, b: 456 }  type_of:&others::box2::test::Data
            print!("p2:{:?}  type_of:{}\n", p2, type_of(&p2));   //p2:Data { a: 123, b: 456 }  type_of:others::box2::test::Data
            print!("p3:{:?}  type_of:{}\n", p3, type_of(&p3));   //p3:Data { a: 123, b: 456 }  type_of:others::box2::test::Data
        }


        //Clone 
        {
            #[derive(Debug, Clone)]        
            struct Data {
                a: String,
                b: i32,
            }
            
            let mut p:Box<Data> = Box::new(Data{a:"123".to_string(), b:456});
            let mut p1 = p.clone();                //clone
            let mut p2 = p.as_ref().clone();
            
            p.a = "111".to_string();
            p1.a = "222".to_string();
            p2.a = "333".to_string();

            //p:Data { a: "111", b: 456 }  type_of:alloc::boxed::Box<others::box2::test::Data>
            print!("\np:{:?}  type_of:{}\n", p, type_of(&p));

            //p1:Data { a: "222", b: 456 }  type_of:alloc::boxed::Box<others::box2::test::Data>
            print!("p1:{:?}  type_of:{}\n", p1, type_of(&p1));

            //p2:Data { a: "333", b: 456 }  type_of:others::box2::test::Data
            print!("p2:{:?}  type_of:{}\n", p2, type_of(&p2));   //others::box2::test::Data
        }

        // borrow reference & deref
        {

            // Copy
            {
                #[derive(Debug, Copy, Clone)]        
                struct Data {
                    a: i32,
                    b: i32,
                }
                
                let mut p:Box<Data> = Box::new(Data{a:123, b:456});
                let mut p1 = &mut p;
                //let mut p2 = *p1;           //this is copy
                                              //error[E0507]: cannot move out of `*p1` which is behind a mutable reference
                                              //move occurs because `*p1` has type `std::boxed::Box<box2::test::Data>`, 
                                              //which does not implement the `Copy` trait
            }

            // No impl Copy 
            {
                #[derive(Debug, Clone)]        
                struct Data {
                    a: String,
                    b: i32,
                }

                let mut p:Box<Data> = Box::new(Data{a:"123".to_string(), b:456});
                let p1 = &p;
                //let p2 = *p1;           //this is copy
                                        //error[E0507]: cannot move out of `*p1` which is behind a shared reference
                                        //move occurs because `*p1` has type `std::boxed::Box<box2::test::Data>`, which does not implement the `Copy` trait
            }
        }
    }

    //Conclusion:
    //  Box means the thing inside is always heap allocated rather than potentially stack
    //  allocated.
    //  There are no null pointer in the rust lang. But can use Option<Box<Data>> instead. 
    //  Use Box::clone can 1. create new heap space for T 2. deep copy src T to dst T
}

