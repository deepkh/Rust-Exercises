use libhelper::*;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn create_and_write_1(file_name: &str) -> std::result::Result<(), String>
{
    let mut fp = match File::create(file_name) {
        Ok(f) => f,
        Err(e) => return Err(format!("failed to open {} {:?}", file_name, e)),
    };

    match fp.write_all(b"some bytes\n") {
        Ok(_) => {},
        Err(e) => return Err(e.to_string()),
    };

    if let Err(e) = fp.write_all(format!("name: {}\n", "ABCDEFG").as_bytes()) {
        return Err(format!("failed to write 2 {:?}", e));
    }

    log!("Hello\n");
    Ok(())
}

fn create_and_write_2(file_name: &str) -> std::result::Result<(), std::io::Error>
{
    let mut fp = match File::create(file_name) {
        Ok(f) => f,
        Err(e) => return Err(std::io::Error::new(e.kind(), format!("failed to open {}", file_name))),
    };

    match fp.write_all(b"some bytes\n") {
        Ok(_) => {},
        Err(e) => return Err(e),
    };

    if let Err(e) = fp.write_all(format!("name: {}\n", "ABCDEFG").as_bytes()) {
        return Err(std::io::Error::new(e.kind(), format!("failed to write to {}", file_name)));
    }

    log!("\n");
    Ok(())
}

fn open_test(file_name: &str) -> std::io::Result<()>
{
    let fp = File::open(file_name)?;
    log!("\n");
    Ok(())
}

pub fn test() {
    let file_name = "file_test.txt";
    //crate::helper::help();
    log!("\n");
    //match open_test(file_name) {
      //  Ok(_) => {},
      //  Err(e) => panic!("XXXXX {:?}", e),
    //};
    let e = open_test(file_name);
    if e.is_err() {
        log!("failed to open {}\n", format!("{:?}", e.err()));
    }

    if let Err(e) = open_test(file_name) {
        log!("failed to open {}\n", format!("{:?}", e));
    }

    match create_and_write_1(file_name) {
        Err(e) => panic!(e),
        Ok(_) => {},
    };

    if let Err(e) = create_and_write_2(file_name) {
        print!("failed to create_and_write_2 {:?}\n", e);
    };

}
