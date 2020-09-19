use libhelper::*;
use crate::ErrStack;
use std::fs::File;
use std::io;
use std::io::{Error,ErrorKind};
use std::io::prelude::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

/***********************************************************
 * iter_filter_test
 ***********************************************************/
pub fn iter_filter_test() {
    print!("\n------------ {} ------------\n", function!());
    
    let v1 = vec![1, 2, 3, 4, 5, 6, 8];
    let v1_iter = v1.iter();
    for val in v1_iter.filter(|x| *x % 2 == 0) {
        print!("{},", val);                   //output: 2, 4, 6, 8
    }
}


/***********************************************************
 * function_pointer_test
 ***********************************************************/
pub fn some_func(s: &str) -> String {
    format!("{} {}", s, "YesYesYes")
}

pub fn function_pointer_test(ofp: Option<&dyn Fn(&str) -> String>) {
    print!("\n------------ {} ------------\n", function!());
    if let Some(fp) = ofp {
        let s: String = fp("YOYOYO");
        log!("fp(\"YOYOYO\") return \"{}\"\n", s);
    }
}



/***********************************************************
 * closure_fnonce_test
 ***********************************************************/
//the default closure is implements fnOnce. it's means closure
//allow to call once.

/***********************************************************
 * closure_fnmut_test
 ***********************************************************/
fn call_multitimes_with_read_write<F>(mut func: F)
    where F: FnMut()
{
    func();
    func();
}

pub fn closure_fnmut_test() {
    print!("\n------------ {} ------------\n", function!());

    let mut x = 1;
    let f = || x += 1;
    call_multitimes_with_read_write(f);
    log!("x = {}\n", x);        // x = 3
}


/***********************************************************
 * closure_fn_test
 ***********************************************************/
fn call_multitimes_with_read_only<F>(func: F) -> usize
    where F: Fn(usize) -> usize
{
    func(1);
    func(1)
}

pub fn closure_fn_test() {
    print!("\n------------ {} ------------\n", function!());

    let f = |x| x + 1;
    let z = call_multitimes_with_read_only(f);
    log!("z = 1 + 1 = {}\n", z);
}


pub fn closure_test() {
    print!("\n------------ {} ------------\n", function!());

    // default behavior: borrow (reference)
    {
        //borrow
        {
            let mut a: i32 = 123;
            let f = |b| a + b;
            //a = 789;                  //error[E0506]: cannot assign to `a` because it is borrowed
            let f_result = f(456);
            a = 789;                    //assign value to a is ok when after f(456)
            print!("a:{} f_result:{}\n", a, f_result);
        }

        //borrow with mutable
        {
            #[derive(Debug)]        
            struct Data {
                a: String,
                b: i32,
            }

            let mut a: Data = Data {a:"AAA".to_string(), b:123};
            let mut f = |b| {
                a.a.push_str(b);
            };
            f("BBB");
            print!("a {:?}\n", a);
        }
    }

    // move behavior: try copy first. if copy is fails than try use move
    {
        //copy
        {
            let mut a: i32 = 123;
            let f = move |b| a + b;
            a = 789;                  //this is ok
            let f_result = f(456);
            print!("a:{} f_result:{}\n", a, f_result);
        }

        //move
        {
            #[derive(Debug)]        
            struct Data {
                a: String,
                b: i32,
            }

            let mut a: Data = Data {a:"AAA".to_string(), b:123};
            let mut f = move |b| {
                a.a.push_str(b);
            };
            f("BBB");
            //a.a = "CCC".to_string();    //error[E0382]: assign to part of moved value: `a`
        }

        //copy
        {
            #[derive(Debug,Clone,Copy)]        
            struct Data {
                a: i32,
                b: i32,
            }

            let mut a: Data = Data {a:123, b:456};
            let mut f = move |b| {
                a.a = b;
            };
            a.a = 123;                      //this is ok before f(789). because a is use Copy
            f(789);
        }
    }
}


pub fn test() {
    print!("\n------------ {} ------------\n", function!());
    closure_test();
    closure_fn_test();
    closure_fnmut_test();
    
    function_pointer_test(Some(&some_func));
    function_pointer_test(Some(&|s| format!("{} {}", s, "NoNoNo")));

    iter_filter_test();
}

