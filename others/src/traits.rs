use libhelper::*;
use crate::ErrStack;
use std::fs::File;
use std::io;
use std::io::{Error,ErrorKind};
use std::io::prelude::*;

/*
mod RationalA {
    struct Struct {
        num: i32,
        den: i32,
    }
    fn new_rational() -> Struct {
        return Struct { num: 3, den: 5 }; // OK
    }
}
*/

//due to Rust donen't support i++ or ++i, we can use trait to implements it.
pub trait PostInc {
    fn post_inc(&mut self) -> Self;
}

impl PostInc for i32 {
    fn post_inc(&mut self) -> Self {
        let x = *self;
        *self += 1;
        x
    }
}

pub trait ToString {
    fn to_string_x(&self) -> String;
}


pub mod mylib {

    //for import PostInc
    use crate::traits::*;

    fn rational_counter_inc() -> i32 {
        static mut rational_counter: i32 = 0;
        unsafe {
            rational_counter.post_inc()
        }
    }

    pub struct Rational {
        pub name: String,
        pub num: i32,
        pub den: i32,
    }

    //namespace function
    fn gcd(mut x: i32, mut y: i32) -> i32 {
        // 求取 x 與 y 的最大公因數，這裡省略實作
        // ...
        0
    }

    pub fn new_rational(num: i32, den: i32) -> Rational {
        return Rational {name: format!("{}", rational_counter_inc()), num, den }; // OK
    }

    //use impl to add member function for sturct Rational
    impl Rational {
        //static member function if the first arg is not &self
        //it would be usefull for constructor
        //but there no rules specified the constructor always be 'new'
        //Can use 'Self' (first char is S but not s) to represent struct of Rational
        pub fn new(n: i32, d: i32) -> Self {
            Rational { name: format!("{}", rational_counter_inc()), num: n, den: d }
        }
       
        //consturctor also can be 'new2'
        pub fn new2(n: i32, d: i32) -> Self {
            Rational { name: format!("{}", rational_counter_inc()), num: n, den: d }
        }

        //member function with immutable &self 
        pub fn is_integer(&mut self) -> bool {
            self.den == 1
        }
        
        //member function with mutable &self 
        pub fn reduce(&mut self) {
            let d = gcd(self.num, self.den);
            self.num /= d;
            self.den /= d;
        }
    }

    //Wrong: look like the implements of ToString are not public when inside of mylib
    //implements ToString interface
    /*
    impl ToString for Rational {
        fn to_string_x(&self) -> String {
            format!("num:{} den:{}", self.num, self.den)
        }
    }
    */
}


//Rust doesn't support inheritance. But can use 'composition' to instead of inheritance. 
//Correct. Can't impl interface inside of namespace ?
//implements ToString interface
impl ToString for mylib::Rational {
    fn to_string_x(&self) -> String {
        format!("num:{} den:{}", self.num, self.den)
    }
}

//Deconstructor: by impl Drop
impl Drop for mylib::Rational {
    fn drop(&mut self) {
       log!("~mylib::Rational() {}\n", self.name);
    }
}


//another type of Complex
pub struct Complex {
    name: String,
    real: f64,
    imaginary: f64,
}

fn complex_counter_inc() -> i32 {
    static mut complex_counter: i32 = 0;
    unsafe {
        complex_counter.post_inc()
    }
}

impl Complex {
    pub fn new(r: f64, i: f64) -> Complex {
        Complex {
            name: format!("{}", complex_counter_inc()),
            real: r,
            imaginary: i,
        }
    }
}

//Deconstructor: by impl Drop
impl Drop for Complex {
    fn drop(&mut self) {
       log!("~Complex() {}\n", self.name);
    }
}

//implements ToString interface
impl ToString for Complex {
    fn to_string_x(&self) -> String {
        format!("real:{} imaginary:{}", self.real, self.imaginary)
    }
}

//also can implements interface for basic type i32
impl ToString for i32 {
    fn to_string_x(&self) -> String {
        format!("i32:{}", self)
    }
}

//also can implements interface for exists struct String
impl ToString for String {
    fn to_string_x(&self) -> String {
        format!("String:{}", self)
    }
}

//also can implements interface for basic type f32
impl ToString for f32 {
    fn to_string_x(&self) -> String {
        format!("f32:{}", self)
    }
}

//only can use '&' pointer to pass interface  
pub fn dump_string(ts: &ToString) {
    log!("{}\n", ts.to_string_x());
}

pub fn test() -> io::Result<()> {
    print!("\n------------ {} ------------\n", function!());

    //let s = RationalA::new_rational();
    //                  ^^^^^^^^^^^^ private function

    //there are four way to contructe the mylib::Rational
    let r1 = mylib::Rational::new(3, 5);
    let r2 = mylib::Rational::new2(3, 5);
    let r3 = mylib::new_rational(3, 5);
    let r4 = mylib::Rational{ name:"-1".to_string(), num: 3, den: 5};    //but this need specified the variables num,den to pub 
    let c1 = Complex::new(99.0, 98.1);
    let c2 = Complex::new(99.1, 98.2);
    let c3 = Complex::new(99.2, 98.3);

    log!("r1:{}\n", r1.to_string_x());  
    
    dump_string(&r4);
    dump_string(&c1);
    dump_string(&32);
    dump_string(&32.1);
    dump_string(&String::from("3210"));

    
    Ok(())
}

