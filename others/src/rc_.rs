use libhelper::*;
use libhelper::helper::type_of;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;

//BorrowMut can cause confusion with Rc<RefCell<X>>
//https://github.com/rust-lang/rust/issues/39232
//use std::borrow::BorrowMut;

/*******************************************
 * MydataRc
 ******************************************/
struct MydataRc {
    name: String,
    age: i32,
}

impl Drop for MydataRc {
    fn drop(&mut self) {
        log!("~mydata() {} {}\n", self.name, self.age);
    }
}

/*******************************************
 * MydataBox
 ******************************************/
#[derive(Debug, Clone)]
struct MydataBox {
    name: String,
    age: i32,
}

impl Drop for MydataBox {
    fn drop(&mut self) {
        log!("~MydataBox() {} {}\n", self.name, self.age);
    }
}


/*******************************************
 * rc_test
 ******************************************/
/**
 * Rc is something like share_ptr as C++11. but Rc can't change the content even with mut flag.
 * Need use Rc<RefMut<T>> to change content 
 *
 * Rc is single threaded. Need use Arc instead for thread environment.
 */
fn rc_test()
{
    print!("\n------------ {} ------------\n", function!());
    /*
    {
        let x = Rc::new(MydataRc {
            name: "XXX".to_string(),
            age: 18,
        });
        //clone for rc is mean add a reference count for x and return ths same address
        let y = x.clone();                          
        let z = y.clone();
        let z = x.clone();

        log!("{}\n", x.name);

        //error[E0594]: cannot assign to data in an `Rc` even the y is mut. Need use Rc<RefMut<T>>
        //instead. 
        //y.name = "YYY".to_string();               
        
        //error[E0594]: cannot assign to data in an `Rc` even the y is mut. Need use Rc<RefMut<T>>
        //let g = x.borrow_mut();
        //g.name = "YYY".to_string();               

        //although clone three times from x, but only execute once of Drop for MydataRc.
    }
    */

    {
        let x = Box::new(MydataBox {
            name: "XXX".to_string(),
            age: 18,
        });

        //clone for box is mean create a new heap space and deep clone data from x to y 
        let mut y = x.clone();                      
        let mut z = y.clone();

        y.name = "YYY".to_string();
        z.name = "ZZZ".to_string();
        
        //clone three times from x. Drop three times for MydataBox.
    }
}

/*******************************************
 * rc_cell_test
 ******************************************/
/**
 * cell can work on some type that implements Copy
 */
fn rc_cell_test()
{
    print!("\n------------ {} ------------\n", function!());
    {
        let p = Rc::new(Cell::new(10));
        println!("{}", p.get()); // 10
        p.set(20);
        println!("{}", p.get()); // 20
    }
}

/*******************************************
 * rc_refcell_test
 ******************************************/
struct MydataRefcell {
    pub name: String,
    pub age: i32,
}

impl Drop for MydataRefcell {
    fn drop(&mut self) {
        log!("~MydataRefcell() {} {}\n", self.name, self.age);
    }
}

fn rc_refcell_change_age(p : Rc<RefCell<MydataRefcell>>) 
{
    let mut p2 = p.borrow_mut();
    p2.age = 200;
}

fn rc_refcell_change_reference(p : Rc<RefCell<MydataRefcell>>) 
{
    let mut p2 = p.borrow_mut();
    *p2 = MydataRefcell {
        name: "XXXXX".to_string(),
        age: 100,
     };
}


/**
 * RefCell can work on some type that doesn't implement Copy
 */
fn rc_refcell_test()
{
    print!("\n------------ {}1 ------------\n", function!());
    {
        let p = Rc::new(RefCell::new(
            MydataRefcell {
                name: "ABCDEFG".to_string(),
                age: 99,
        }));

        //alloc::rc::Rc<core::cell::RefCell<others::rc_::MydataRefcell>>
        log!("p type_of: {}\n", type_of(&p));

        //try to change name
        {
            //core::cell::RefMut<others::rc_::MydataRefcell>
            let mut p2 = p.borrow_mut();
            log!("p2 type_of: {} \n", type_of(&p2));
            p2.name = "GDEFGH".to_string();

            //runtime error: already mutably borrowed: BorrowError
            //let p3 = p.clone();
            //let p4 = p3.borrow();
        }

        //try to change age by use function 
        {
            rc_refcell_change_age(p.clone());
        }

        log!("p.age:{}\n", p.borrow().age);

        //try to replace new MydataRefcell instance. 
        //the previous 'MydataRefcell() GDEFGH 200' will done before new MydataRefcell instance created.
        {
            rc_refcell_change_reference(p.clone());
        }

        log!("p.name:{}\n", p.borrow().name);
    }
}


pub fn test()  {
    rc_test();
    rc_cell_test();
    rc_refcell_test();
}

