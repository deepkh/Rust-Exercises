use libhelper::*;
use crate::ErrStack;
use std::fs::File;
use std::io;
use std::io::{Error,ErrorKind};
use std::io::prelude::*;

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



/***********************************************************
 * closure_test
 ***********************************************************/
struct S1 {
    v: i32,
}

fn new_s1(v: i32) -> S1 {
    return S1 { v }; 
}

impl S1 {
    pub fn get(&self) -> i32 {
        self.v
    }
    
    pub fn set(&mut self, v: i32)  {
        self.v = v;
    }
}


//the Copy derive only the member all implements Copy derive
#[derive(Debug, Clone, Copy)]
pub struct S2 {
    pub v: i32,
}

fn new_s2(v: i32) -> S2 {
    return S2 { v }; 
}

impl S2 {
    pub fn get(&self) -> i32 {
        self.v
    }
    
    pub fn set(&mut self, v: i32)  {
        self.v = v;
    }
}

pub fn closure_test() {
    print!("\n------------ {} ------------\n", function!());
   
    //one line
    {
        let mut y = 10;
        let z = |x| x + y + 1;
        print!("z = {}\n", z(20));          // z = 20 + 10 + 1 = 31
    }

    //multi-line
    {
        let mut y = 10;
        let z = |x| {
            x + y + 1
        };
        print!("z = {}\n", z(20));          // z = 20 + 10 + 1 = 31
    }

    //y = 123 -> error[E0506]: cannot assign to `y` because it is borrowed
    //{
        //let mut y = 10;
        //let z = |x| x + y + 1;
        //y = 123;                          //error[E0506]: cannot assign to `y` because it is borrowed
        //print!("z = {}\n", z(20));        // z = 20 + 10 + 1 = 31
    //}

    //move1: use move key word to copy or move. eg., try copy first if ok. try move second if copy failed.
    {
        let mut y = 10;
        let z = move |x| x + y + 1;
        y = 123;
        print!("z = {}, y = {}\n", z(20), y);        // z = 20 + 10 + 1 = 31, y = 123
    }

    //move2: struct that implements copy. if struct doesn't implements copy it will use move
    //instead and will cause error[E0506] occured in this case.
    {
        let mut y = new_s2(10);
        let z = move |x| format!("{}", x + y.get()); 
        y.set(456); 
        print!("z = {}, y = {}\n", z(20), y.get());        // z = 30, y = 456
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

