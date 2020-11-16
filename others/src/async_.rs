use libhelper::*;
use libhelper::helper::type_of;
use futures::executor::block_on;


pub fn test()  {
    print!("\n------------ {} ------------\n", function!());

    print!("\n1\n");
    {
        async fn learn_song() { 
            for _ in 1..10000000 {
            }
            log!("\n");
        }
        async fn sing_song() { 
            for _ in 1..10 {
            }
            log!("\n");
        }
        async fn dance() { 
            for _ in 1..1 {
            }
            log!("\n");
        }

        async fn learn_and_sing() {
            // Wait until the song has been learned before singing it.
            // We use `.await` here rather than `block_on` to prevent blocking the
            // thread, which makes it possible to `dance` at the same time.
            log!("1\n");
            let f1 = learn_song();
            let f2 = sing_song();
            //let r1 = f1.await;
            //let r2 = f2.await;
            futures::join!(f1, f2);
            log!("2\n");
        }

        async fn async_main() {
            let f1 = learn_and_sing();
            let f2 = dance();

            // `join!` is like `.await` but can wait for multiple futures concurrently.
            // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
            // future will take over the current thread. If `dance` becomes blocked,
            // `learn_and_sing` can take back over. If both futures are blocked, then
            // `async_main` is blocked and will yield to the executor.
            futures::join!(f1, f2);
            log!("done\n");
        }

        block_on(async_main());
    }

    log!("done");
}

