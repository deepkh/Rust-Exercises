#[macro_export]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }}
}

#[macro_export]
macro_rules! log {
    ($($args:expr),*) => {{
        let mut c: i32 = 0;
        let mut fmt: String = String::from("");
        $(
            if c == 0 {
                fmt = $args.to_string();
                c+=1;
            } else {
                fmt = fmt.replacen("{}", &$args.to_string(), 1);
            }
        )*
        print!("[{}] {}", crate::function!(), fmt);
        c
    }}
}

#[macro_export]
macro_rules! ErrStack {
    ($e:expr, $($args:expr),*) => {{
        let mut c: i32 = 0;
        let mut fmt: String = String::from("");
        $(
            if c == 0 {
                fmt = $args.to_string();
                c+=1;
            } else {
                fmt = fmt.replacen("{}", &$args.to_string(), 1);
            }
        )*
        std::io::Error::new($e.kind(), format!("\t{} \n\t({}:{}) {}", $e.to_string(),  crate::function!(), line!(), fmt))
    }};
}

pub fn type_of<T>(_: &T) -> String {
    return  String::from(std::any::type_name::<T>());
}


pub fn help() {
    crate::log!("Hello\n");
}
