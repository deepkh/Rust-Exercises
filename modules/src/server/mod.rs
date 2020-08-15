pub fn help() {
    crate::main::help();
    super::main::help();
    modules::client::help2();
    println!("hello server::help()");
}
