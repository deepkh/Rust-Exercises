pub mod message_queue;
pub mod test;

#[no_mangle]
pub extern fn rust_function_b() {
    println!("Hello this is rust function a\n");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


