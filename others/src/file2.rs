use libhelper::*;
use std::fs::File;
use std::io;
use std::io::{Error,ErrorKind};
use std::io::prelude::*;

fn create_and_write(file_name: &str) -> io::Result<()>
{
    log!("\n");

    let mut fp = match File::create(file_name) {
        Ok(f) => f,
        Err(e) => return Err(ErrStack!(e, "failed to open {}", file_name)),
    };

    if let Err(e) = fp.write_all(b"some bytes\n") {
        return Err(ErrStack!(e, "failed to write 'some bytes'"));
    };

    if let Err(e) = fp.write_all(format!("name: {}\n", "ABCDEFG").as_bytes()) {
        return Err(ErrStack!(e, "failed to write 'some bytes'"));
    }

    Ok(())
}

fn open_and_read(file_name: &str) -> io::Result<String>
{
    let mut str = String::new();
    log!("\n");

    let mut fp = match File::open(file_name) {
        Ok(f) => f,
        Err(e) => return Err(ErrStack!(e,"failed to open {}", file_name)),
    };

    if let Err(e) = fp.read_to_string(&mut str) {
        return Err(ErrStack!(e, "failed to read {}", file_name));
    }

    Ok(str)
}

fn do_fake_error() -> io::Result<()> 
{
    if 0 != 1 {
        return Err(ErrStack!(
            Error::new(ErrorKind::Other, "FAKE ERR: No such file or directory (os error 2)")
            , "failed to xxxx"
        ));
    }

    Ok(())
}

pub fn test() -> io::Result<()> {
    print!("\n------------ {} ------------\n", function!());

    let file_name = "file_test2.txt";


    if let Err(e) = create_and_write(file_name) {
        return Err(ErrStack!(e, "failed to create_and_write"));
    }

    let s = match open_and_read(file_name) {
        Ok(s) => s,
        Err(e) => return Err(ErrStack!(e, "failed to open_and_read")),
    };

    log!("read: '{}'\n", s);

    if let Err(e) = do_fake_error() {
        return Err(ErrStack!(e, "failed to do_fake_error"));
    }

    Ok(())
}
