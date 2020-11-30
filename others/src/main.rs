use libhelper::*;
mod file;
mod file2;
mod match_handling;
mod traits;
mod box_;
mod rc_;
mod closure;
mod copy_move_clone;
mod box2;
mod rc2;
mod mutex_guard_;
mod channel_;
mod json;
mod tokio_;
mod tokio_2;
//use msgq::message_queue::*;
//use msgq::test::*;

use traits::*;

fn main() {
    print!("\n------------ {}1 ------------\n", function!());

    libhelper::rust_function_a();
    libhelper::log!("XXXX\n");
    
    //** match_handling test */
    crate::match_handling::test();

    //** file test */
    crate::file::test();
    
    //** file2 test */
    if let Err(e) = crate::file2::test() {
        print!("{}\n", ErrStack!(e, "failed to crate::file2::test()").to_string());
    };

    //** traits test */
    crate::traits::test().unwrap();

    print!("\n------------ {} use traits from main module ------------\n", function!());
    //use trait from another module
    let n: i32 = 43210;
    let pn: &dyn crate::traits::ToString = &n;
    crate::traits::dump_string(&n);
    crate::traits::dump_string(pn);

    //use trait PostInc for i32
    let mut n2: i32 = 543210;
    log!("n2.post_inc():{} n2:{} to_string_x:{}\n", n2.post_inc(), n2, n2.to_string_x());

    //use Rationl
    let r1 = mylib::Rational::new(123, 456);
    dump_string(&r1);

    //use Complex
    let c1 = Complex::new(99.2, 98.3);
    dump_string(&c1);
    
    //** box test */
    crate::box_::test();
    
    //** rc test */
    crate::rc_::test();
    
    //** closure test */
    crate::closure::test();
    
    //** copy_move_clone test */
    crate::copy_move_clone::test();
    
    //** box2 test */
    crate::box2::test();

    //** rc2 test */
    crate::rc2::test();

    //** mutex_guard_ test */
    crate::mutex_guard_::test();
    
    //** channel_ test */
    crate::channel_::test();

    //** msgq::test::TestMessageQueue() */
    msgq::test::test_message_queue();
   
    /*
    //** json::test() */
    crate::json::test();
    
    //** tokio_ */
    crate::tokio_::test();
    */
    
    //** tokio_2 */
    crate::tokio_2::test();
    print!("\n------------ {} done ------------\n", function!());
}
