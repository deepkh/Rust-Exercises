use libhelper::*;
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

    // References for Copy, Move, Clone
    // 0. https://doc.rust-lang.org/std/marker/trait.Copy.html
    // 1. https://www.chainnews.com/zh-hant/articles/734292099188.htm
    // 2. https://zhuanlan.zhihu.com/p/21730929
    // 3. https://juejin.im/post/6862339363229925390

    // Difference between Copy and Clone
    //1. Copies happen implicitly: eg., y = x
    //2. Cloning is an explicit: eg., x.clone()
    //3. Clone is a supertrait of Copy, so everything which is Copy must also implement Clone
    //4. If a type is Copy then its Clone implementation only needs to return *self (see the example above).

    //implicitly: Copy, Move
    {
        // Copy
        {
            let mut a: i32 = 123;
            let b = a;
            a = 456;
        }

        // Move 
        {
            let mut a: String = "AAA".to_string();
            let b = a;
            //a.push_str("BBB"); //error[E0382]: borrow of moved value: `a`
        }

        // Move
        {
            #[derive(Debug)]        
            struct Data {
                a: i32,
                b: i32,
            }
            
            let mut a: Data = Data{a:123, b:456};
            let b = a;                              //this is Move
            //a.a = 456;                              //error[E0382]: assign to part of moved value: `a`
            //let c = a.clone();                        //method not found in `copy_move_clone::test::Data`
        }

        // Copy: implement Copy, Clone manually
        {
            #[derive(Debug)]        
            struct Data {
                a: i32,
                b: i32,
            }
            
            impl Copy for Data {}

            //Clone is a supertrait of Copy, so everything member variables which is Copy must also implement Clone
            //in this case the copy behavior is same as clone
            impl Clone for Data {
                fn clone(&self) -> Data {
                    *self                   //does this means copy ?
                }
            }
            
            let mut a: Data = Data{a:123, b:456};
            let b = a;                              //this is Copy
            a.a = 456;
            a.b = 123;
            let c = a.clone();
        }

        // Copy: implement Copy, Clone  by use derive
        {
            //Clone is a supertrait of Copy, so everything member variables which is Copy must also implement Clone
            //in this case the copy behavior is same as clone
            #[derive(Debug, Copy, Clone)]        
            struct Data {
                a: i32,
                b: i32,
            }
            
            let mut a: Data = Data{a:123, b:456};
            let b = a;
            a.a = 456;
            a.b = 123;
            let c = a.clone();
        }
    }

    // explicit: Clone
    {
        //Clone:
        {
            let mut a: String = "AAA".to_string();
            let b = a.clone();
            a.push_str("BBB");
        }

        //Clone: implement Clone manually
        {
            #[derive(Debug)]        
            struct Data {
                a: String,
                b: i32,
            }
            
            //impl Copy for Data {}             //can't implemnt Copy due to 'a: String' are not implement Copy

            impl Clone for Data {
                fn clone(&self) -> Data {
                    print!("clone occured ");
                    //*self                       //error[E0507]: cannot move out of `*self` which is behind a shared reference
                    Data {
                        a: self.a.clone(),
                        b: self.b,
                    }
                }
            }

            let mut a: Data = Data{a: "AAA".to_string(), b: 123};
            //let b = a;
            //a.b = 456;                      //error[E0382]: assign to part of moved value: `a
            let mut c = a.clone();
            a.a = "BBB".to_string();
            c.a = "CCC".to_string();
            print!("a {:?} c {:?}\n", a, c);    //clone occured a Data { a: "BBB", b: 123 } c Data { a: "CCC", b: 123 }
        }

        //Clone: implement Clone use derive
        {
            #[derive(Debug, Clone)]        
            struct Data {
                a: String,
                b: i32,
            }
            
            let mut a: Data = Data{a: "AAA".to_string(), b: 123};
            //let b = a;
            //a.b = 456;                      //error[E0382]: assign to part of moved value: `a
            let mut c = a.clone();
            a.a = "BBB".to_string();
            c.a = "CCC".to_string();
            print!("a {:?} c {:?}\n", a, c); //a Data { a: "BBB", b: 123 } c Data { a: "CCC", b: 123 }
        }
    }
}

