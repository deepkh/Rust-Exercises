
extern { fn mylib_test_a(); }

fn main() {
    unsafe { mylib_test_a(); }
    println!("Hello, world!");
}
