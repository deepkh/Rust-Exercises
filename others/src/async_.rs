use libhelper::*;
//use libhelper::helper::type_of;
use futures::executor::block_on;


pub fn test()  {
    print!("\n------------ {} ------------\n", function!());

    /* case 1 & 2 bave same execution order
[others::async_::test::bc::{{closure}}] 1
[others::async_::test::b::{{closure}}]
[others::async_::test::c::{{closure}}]
[others::async_::test::bc::{{closure}}] 2
[others::async_::test::d::{{closure}}]
[others::async_::test::a::{{closure}}] done
    */
    print!("\n1\n");
    {
        async fn b() { 
            for _ in 1..10000000 {
            }
            log!("\n");
        }
        async fn c() { 
            for _ in 1..10 {
            }
            log!("\n");
        }
        async fn d() { 
            for _ in 1..1 {
            }
            log!("\n");
        }

        async fn bc() {
            log!("1\n");
            let f1 = b();
            let f2 = c();
            futures::join!(f1, f2);
            log!("2\n");
        }

        async fn a() {
            let f1 = bc();
            let f2 = d();
            futures::join!(f1, f2);
            log!("done\n");
        }

        block_on(a());
    }

    print!("\n2\n");
    {
        fn b() { 
            for _ in 1..10000000 {
            }
            log!("\n");
        }
        fn c() { 
            for _ in 1..10 {
            }
            log!("\n");
        }
        async fn d() { 
            for _ in 1..1 {
            }
            log!("\n");
        }

        async fn bc() {
            log!("1\n");
            b();
            c();
            log!("2\n");
        }

        async fn a() {
            let f1 = bc();
            let f2 = d();
            futures::join!(f1, f2);
            log!("done\n");
        }

        block_on(a());
    }



    /*
[others::async_::test::bc::{{closure}}] 1
[others::async_::test::bc::{{closure}}] 2
[others::async_::test::d::{{closure}}]
[others::async_::test::a::{{closure}}] done
[others::async_::test] done
    */
    print!("\n3\n");
    {
        async fn b() { 
            for _ in 1..10000000 {
            }
            log!("\n");
        }
        async fn c() { 
            for _ in 1..10 {
            }
            log!("\n");
        }
        async fn d() { 
            for _ in 1..1 {
            }
            log!("\n");
        }

        /*
         * if no await b() & c() inner bc(), it look like
         * b() & c() will never print its log after a() is done
         */
        async fn bc() {
            log!("1\n");
            b();
            c();
            log!("2\n");
        }

        async fn a() {
            let f1 = bc();
            let f2 = d();
            futures::join!(f1, f2);
            log!("done\n");
        }

        block_on(a());
    }



    log!("done");
}

