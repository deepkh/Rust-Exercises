use libhelper::*;
mod file;
mod file2;
mod match_handling;
mod traits;

use traits::*;

fn main() {
    print!("\n------------ {}1 ------------\n", function!());


    libhelper::rust_function_a();
    libhelper::log!("XXXX\n");
    crate::match_handling::test();
    crate::file::test();
    if let Err(e) = crate::file2::test() {
        print!("{}\n", ErrStack!(e, "failed to crate::file2::test()").to_string());
    };

    crate::traits::test();

    print!("\n------------ {} use traits from main module ------------\n", function!());
    //use trait from another module
    let n: i32 = 43210;
    let pn: &crate::traits::ToString = &n;
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

}
