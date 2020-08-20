use crate::log;

//https://doc.rust-lang.org/std/io/type.Result.html
//https://doc.rust-lang.org/std/result/enum.Result.html
//https://doc.rust-lang.org/std/result/index.html

//Ok, Err is Option type
fn get_string(i: u32) -> std::result::Result<&'static str, &'static str> 
{
    match i {
        0 => Ok("OK_0"),
        1 => Ok("OK_1"),
        2 => Err("Err_2"),
        _ => Ok("Err__"),
    }
}

fn match_handling_test_1() 
{
    {
        let r = match get_string(0) {
            Ok(s) => Ok(s),
            Err(s) => Err(s),
        };

        if r.ok().is_none() == false {
            log!("{}\n", r.ok().unwrap());
        }
    }

    {
        let r = match get_string(1) {
            Ok(s) => s,
            Err(e) => panic!("failed to get string 2 {:?}\n", e),
        };

        log!("{}\n", r);
    }
}

fn match_handling_test_2(i: u32) -> std::result::Result<String, String>
{
    let r = match get_string(i) {
        Ok(s) => s.to_string(),
        Err(s) => return Err(s.to_string()),
    };
    
    Ok(r)
}

pub fn test() {
    crate::helper::help();
    log!("Hello {}\n", 1);
    match_handling_test_1();

    match match_handling_test_2(0) {
        Ok(s) => log!("{}\n", s),
        Err(s) => panic!("{}\n", s),
    };
    
    match match_handling_test_2(2) {
        Ok(s) => log!("{}\n", s),
        Err(s) => log!("{}\n", s),
    };
}
