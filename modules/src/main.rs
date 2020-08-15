pub mod test;
pub mod server;

mod main {
    pub fn help() {
        crate::main2::help2();
        self::help2();
        println!("Hello from main::help");
    }

    pub fn help2() {
        println!("Hello from main::help2");
    }
}

mod main2 {
    pub fn help2() {
        println!("Hello from main2::help2");
    }
}


use crate::main::help;

fn main() {
    crate::main::help();
    self::main::help();
    main::help();
    help();
    test::help();

    modules::common::help();
    modules::common2::help();
    modules::client::help();
    modules::client::connection::help();
    server::help();
    println!("Hello main()");
}
