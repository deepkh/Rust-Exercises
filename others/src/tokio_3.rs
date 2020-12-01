use libhelper::*;
//use libhelper::helper::type_of;
//use tokio::time::sleep;
//use tokio::time::{Duration};
use tokio::runtime::Runtime;
use tokio::runtime;

pub fn test() {
    print!("\n------------ {} ------------\n", function!());

    //single-threaded
    let rt = runtime::Builder::new_current_thread().build().unwrap();
    
    //rt-multi-thread selected by default
    //let rt  = Runtime::new().unwrap();

    //rt-multi-thread
    /*let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("my thread")
        .enable_io()
        .build()
        .unwrap();*/
/*
[others::tokio_3::test::bar::{{closure}}] start 0
[others::tokio_3::test::bar::{{closure}}] end  0
[others::tokio_3::test::bar::{{closure}}] start 1
[others::tokio_3::test::bar::{{closure}}] 1 0
[others::tokio_3::test::bar::{{closure}}] end  1
[others::tokio_3::test::bar::{{closure}}] start 2
[others::tokio_3::test::bar::{{closure}}] 2 0
[others::tokio_3::test::bar::{{closure}}] 2 1
[others::tokio_3::test::bar::{{closure}}] end  2
[others::tokio_3::test::bar::{{closure}}] start 3
[others::tokio_3::test::bar::{{closure}}] 3 0
[others::tokio_3::test::bar::{{closure}}] 3 1
[others::tokio_3::test::bar::{{closure}}] 3 2
[others::tokio_3::test::bar::{{closure}}] end  3
[others::tokio_3::test::foo::{{closure}}] 4
*/
    {
        async fn bar(n: i32) {
            log!("start {}\n", n);
            for i in 0..n {
                log!("{} {}\n", n, i);
                /* this would be */
                for _j in 0..1000000 {
                }
                
                /* equal with this */
                //sleep(Duration::from_millis(10)).await;
            }
            log!("end  {}\n", n);
        }

        /*
         * wrong
        async fn foo(n: i32) {
            for i in 0..n {
                tokio::spawn(async move {
                    bar(i).await;
                });
            }
            log!("{}\n", n);
        }
        */

        /* Yes */
        async fn foo(n: i32) {
            let mut handles = vec![];
            for i in 0..n {
                handles.push(tokio::spawn(bar(i)));
            }
            futures::future::join_all(handles).await;
            log!("{}\n", n);
        }

        rt.block_on(foo(4));
    }
}
