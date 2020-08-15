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
        println!("[{}] {}", crate::function!(), fmt);
        c
    }}
}
