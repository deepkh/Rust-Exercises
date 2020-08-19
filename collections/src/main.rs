mod vector;
mod hashmap;
mod helper;

fn main() {
    crate::vector::test();
    crate::hashmap::test();
   log!("Hello main! {} {} {}\n", 1, 2, 3);
}
