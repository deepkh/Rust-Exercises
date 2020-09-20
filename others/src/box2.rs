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

pub fn test() {
    let mut case_i = 0;
    print!("\n------------ {} ------------\n", function!());

    // References for Box
    // https://electronic.blue/blog/2017/04/09-rust-an-introduction-smart-pointer/

    // Box can alloc a size of T from the heap memory.
    {
        {
            let mut p:Box<i32> = Box::new(123);
            *p = 456;
            print!("*p={:} p={} \n", *p, p);   //*p=456 p=456
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

}

