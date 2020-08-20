mod helper;
mod file;
mod file2;
mod match_handling;

fn main() {
    crate::match_handling::test();
    crate::file::test();
    if let Err(e) = crate::file2::test() {
        print!("{}\n", ErrStack!(e, "failed to crate::file2::test()").to_string());
    };

   log!("Hello main! {} {} {}\n", 1, 2, 3);
}
