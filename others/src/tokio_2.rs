use libhelper::*;
use libhelper::helper::type_of;
use tokio::time::sleep;
use tokio::time::{Duration};
use futures::future::select_all;
use futures::future::FutureExt;
use tokio::runtime::Runtime;


#[tokio::main]
pub async fn test() {
    print!("\n------------ {} ------------\n", function!());

/*
[others::tokio_2::test::{{closure}}::some_function::{{closure}}] start 1
[others::tokio_2::test::{{closure}}::some_function::{{closure}}] start 2
[others::tokio_2::test::{{closure}}::some_function::{{closure}}] start 3
[others::tokio_2::test::{{closure}}::some_function::{{closure}}] start 4
[others::tokio_2::test::{{closure}}::some_function::{{closure}}] end  1
[others::tokio_2::test::{{closure}}::some_function::{{closure}}] end  4
[others::tokio_2::test::{{closure}}::some_function::{{closure}}] end  3
[others::tokio_2::test::{{closure}}::some_function::{{closure}}] end  2
*/
    async fn some_function(i: u64) -> Result<(), ()> {
        log!("start {}\n", i);
        sleep(Duration::from_millis(10)).await;
        log!("end  {}\n", i);
        Ok(()) // sometimes succeeds, sometimes doesn't - here it succeeds every time
    }

    let mut fs = Vec::new();
    for i in 1..5 {
        fs.push(some_function(i).boxed());
    }
    loop {
        let (res, idx, remaining_futures) = select_all(fs).await;
        fs = remaining_futures;

        if fs.len() == 0 {
            break;
        }
    }

    print!("\n");

/*
[others::tokio_2::test::{{closure}}::some_function::{{closure}}] start 1
[others::tokio_2::test::{{closure}}::some_function::{{closure}}] start 2
[others::tokio_2::test::{{closure}}::some_function::{{closure}}] start 3
[others::tokio_2::test::{{closure}}::some_function::{{closure}}] start 4
[others::tokio_2::test::{{closure}}::some_function::{{closure}}] end  1
[others::tokio_2::test::{{closure}}::some_function::{{closure}}] end  4
[others::tokio_2::test::{{closure}}::some_function::{{closure}}] end  3
[others::tokio_2::test::{{closure}}::some_function::{{closure}}] end  2
*/
    struct Data {
    }

    impl Data {
        pub fn new() -> Self {
            Self {
            }
        }

        async fn some_function(&self, i: u64) -> Result<(), ()> {
            log!("start {}\n", i);
            sleep(Duration::from_millis(10)).await;
            log!("end  {}\n", i);
            Ok(()) // sometimes succeeds, sometimes doesn't - here it succeeds every time
        }
    }

    async fn some_function2(d: Data, i: u64) -> Result<(), ()> {
        d.some_function(i).await
    }

    let mut fs = Vec::new();
    for i in 1..5 {
        fs.push(some_function2(Data::new(), i).boxed());
    }
    loop {
        let (res, idx, remaining_futures) = select_all(fs).await;
        fs = remaining_futures;
        print!("res:{:?} idx:{:?}\n", res, idx);

        if fs.len() == 0 {
            break;
        }
    }

}
