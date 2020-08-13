pub mod test;
pub mod server;

fn main() {
    test::help();
    modules::common::help();
    modules::common2::help();
    modules::client::help();
    modules::client::connection::help();
    server::help();
    println!("Hello main()");
}
