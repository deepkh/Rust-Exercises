use libhelper::*;
//use libhelper::helper::type_of;
use tokio::time::sleep;
use tokio::time::{Duration};
use tokio::runtime::Runtime;


pub fn test() {
    print!("\n------------ {} ------------xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\n", function!());
    let runtime = Runtime::new().unwrap();
    
    /*
1
[others::tokio_::test::b::{{closure}}] start/end
[others::tokio_::test::c::{{closure}}] start
[others::tokio_::test::d::{{closure}}] start
[others::tokio_::test::c::{{closure}}] end
[others::tokio_::test::d::{{closure}}] end
[others::tokio_::test::dispatch_1::{{closure}}] done
---
[others::tokio_::test::d::{{closure}}] start
[others::tokio_::test::d::{{closure}}] end
[others::tokio_::test::c::{{closure}}] start
[others::tokio_::test::c::{{closure}}] end
[others::tokio_::test::b::{{closure}}] start/end
[others::tokio_::test::dispatch_2::{{closure}}] done
---
[others::tokio_::test::b::{{closure}}] start/end
[others::tokio_::test::c::{{closure}}] start
[others::tokio_::test::d::{{closure}}] start
[others::tokio_::test::c::{{closure}}] end
[others::tokio_::test::d::{{closure}}] end
[others::tokio_::test::Data::func1::{{closure}}] done
[others::tokio_::test::dispatch_3::{{closure}}] YO
---
[others::tokio_::test::b::{{closure}}] start/end
[others::tokio_::test::c::{{closure}}] start
[others::tokio_::test::d::{{closure}}] start
[others::tokio_::test::c::{{closure}}] end
[others::tokio_::test::d::{{closure}}] end
[others::tokio_::test::Data::func1::{{closure}}] done
[others::tokio_::test::dispatch_4] YO
[others::tokio_::test] done
    */
    print!("\n1\n");
    {
        async fn b() { 
            log!("start/end  \n");
        }
        async fn c() { 
            log!("start\n");
            sleep(Duration::from_millis(10)).await;
            log!("end  \n");
        }
        async fn d() { 
            log!("start\n");
            sleep(Duration::from_millis(30)).await;
            log!("end  \n");
        }

        async fn dispatch_1() {
            let f3 = d();
            let f2 = c();
            let f1 = b();
            futures::join!(f1, f2, f3);
            log!("done\n");
        }

        async fn dispatch_2() {
            print!("---\n");
            d().await;
            c().await;
            b().await;
            log!("done\n");
        }

        struct Data {
        }

        impl Data {
            pub fn new() -> Self {
                Self {
                }
            }

            pub async fn func1(&self) {
                let f3 = d();
                let f2 = c();
                let f1 = b();
                futures::join!(f1, f2, f3);
                log!("done\n");
            }
        }

        async fn dispatch_3() {
            print!("---\n");
            let d = Data::new();
            d.func1().await;
            log!("YO\n");
        }

        fn dispatch_4(runtime: &Runtime) {
            print!("---\n");
            let d = Data::new();
            runtime.block_on(d.func1());
            log!("YO\n");
        }

        runtime.block_on(dispatch_1());
        runtime.block_on(dispatch_2());
        runtime.block_on(dispatch_3());
        dispatch_4(&runtime);
    }

    log!("done");
}

