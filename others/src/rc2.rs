use libhelper::*;
use libhelper::helper::type_of;
use std::rc::Rc;
use std::cell::RefCell;

pub fn test()  {
    print!("\n------------ {} ------------\n", function!());

        //Rc + RefCell for data that doesnt impl Copy
        {
            #[derive(Debug)]        
            struct Data {
                a: String,
                b: i32,
            }

            impl Drop for Data {
                fn drop(&mut self) {
                    log!("~Data() {} {}\n", self.a, self.b);
                }
            }

            let b: Rc<RefCell<Data>> = Rc::new(RefCell::new(Data {a:"AAAA".to_string(), b:333}));
            print!("b:{:?} type_of:{}\n", b, type_of(&b));
            //b:RefCell { value: Data { a: "AAAA", b: 333 } } type_of:alloc::rc::Rc<core::cell::RefCell<others::rc_arc::test::Data>>
           
            //write
            {
                let mut b1 = b.borrow_mut();
                b1.b = 456;
                print!("b1:{:?}  type_of:{}\n", b1, type_of(&b1));
                //b1:Data { a: "AAAA", b: 456 }  type_of:core::cell::RefMut<others::rc_arc::test::Data>
            }

            //read
            {
                let b2 = b.borrow();
                print!("b2:{:?} type_of:{}\n", b2, type_of(&b2));
                //b2:Data { a: "AAAA", b: 456 } type_of:core::cell::Ref<others::rc_arc::test::Data>
            }

            //clone, write 
            {
                let b3 = b.clone();
                print!("b3:{:?} type_of:{}\n", b3, type_of(&b3));
                //b3:RefCell { value: Data { a: "AAAA", b: 456 } } type_of:alloc::rc::Rc<core::cell::RefCell<others::rc_arc::test::Data>>
                let mut b4 = b3.borrow_mut();
                b4.a.push_str("GGGGGG");
                b4.b = 999;
            }

            //Drop
            //[<others::rc_arc::test::Data as core::ops::drop::Drop>::drop] ~Data() AAAAGGGGGG 999
        }

}

