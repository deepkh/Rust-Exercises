use libhelper::*;
use libhelper::helper::type_of;
use std::sync::{Mutex, Arc};

pub fn test()  {
    print!("\n------------ {} ------------\n", function!());

        {
            #[derive(Debug)]        
            struct Data {
                a: String,
                b: i32,
                c: Mutex<String>
            }

            impl Drop for Data {
                fn drop(&mut self) {
                    log!("~Data() {} {} {}\n", self.a, self.b, self.c.lock().unwrap());
                }
            }

            let b: Arc<Data> = Arc::new(Data {a:"AAAA".to_string(), b:333, c: Mutex::new("GGGG".to_string())});
            print!("b:{:?} type_of:{}\n", b, type_of(&b));
            //b:RefCell { value: Data { a: "AAAA", b: 333 } } type_of:alloc::rc::Rc<core::cell::RefCell<others::rc_arc::test::Data>>

            //let mut c  = b.as_mut();
            //method not found in `std::sync::Arc<thread_::test::Data>`

            //b.a = "AAAA".to_string();
            //error[E0594]: cannot assign to data in an `Arc`

            //b.a.push_str("AAAA")
            //error[E0596]: cannot borrow data in an `Arc` as mutable

            //s == MutexGuard<String>
            {
                let mut s = b.c.lock().unwrap();
                s.push_str("GGGGGGG");             //this is ok, can modified string
                print!("s:{:?} type_of:{}\n", s, type_of(&s)); 
                //s:"GGGGGGGGGGG" type_of:std::sync::mutex::MutexGuard<alloc::string::String>
            }
           
            //https://stackoverflow.com/a/51336145/11474144
            {
                let mut s = b.c.lock().unwrap();
                (*s).push_str("XXXXXXXX");
                print!("*s:{:?} type_of:{}\n", *s, type_of(&*s)); 
                //s:"GGGGGGGGGGG" type_of:std::sync::mutex::MutexGuard<alloc::string::String>
            }

            {
                print!("b.c:{:?} \n", b.c.lock().unwrap()); 
                //b.c:"GGGGGGGGGGGXXXXXXXX"
            }
           
            //look like can assign new String instance via MutexGuard<T>'s DerefMut 
            {
                let mut s = b.c.lock().unwrap();
                (*s) = "RESET".to_string();
            }

            //Drop
            //[<others::thread_::test::Data as core::ops::drop::Drop>::drop] ~Data() AAAA 333 RESET
        }

}

