use libhelper::*;
use libhelper::helper::type_of;
use crate::ErrStack;
use std::fs::File;
use std::io;
use std::io::{Error,ErrorKind};
use std::io::prelude::*;
use std::rc::Rc;
use std::cell::Cell;
use std::cell::RefCell;
use std::sync::{Mutex, Arc};
use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::time::Duration;
use std::collections::HashMap;

pub fn test()  {
    print!("\n------------ {} ------------\n", function!());

    //https://doc.rust-lang.org/book/ch16-02-message-passing.html
    //mpsc stands for ***multiple producer(sender), single consumer(receiver)***
    {
        let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

        let s0 = "AAAA".to_string();
        
        //Sending is a non-blocking operation, the thread will continue
        //immediately after sending its message
        tx.send(s0).unwrap();

        //let s01 = s0; //error[E0382]: use of moved value: `s0`

        //recv is blocking operation
        let s = rx.recv();
        print!("s:{:?} type_of:{}\n", s, type_of(&s));
        //s:Ok("AAAA") type_of:core::result::Result<alloc::string::String, std::sync::mpsc::RecvError>
        
        let s1 = s.unwrap(); 
        print!("s1:{:?} type_of:{}\n", s1, type_of(&s1));
        //s1:"AAAA" type_of:alloc::string::String
    }

    //send, recv shared pointer object through Rc + RefCell
    {
        #[derive(Debug)]
        struct Data {
            a: String,
            b: i32,
            c: Vec<String>,
        }

        impl Drop for Data {
            fn drop(&mut self) {
                print!("~Data() {:?}\n", self);
            }
        }
        
        let b: Rc<RefCell<Data>> = Rc::new(RefCell::new(Data {a:"AAAA".to_string(), b:333, c: Vec::new()}));
        print!("b:{:?} type_of:{}\n", b, type_of(&b));
        //b:RefCell { value: Data { a: "AAAA", b: 333, c: [] } } type_of:alloc::rc::Rc<core::cell::RefCell<others::channel_::test::Data>>
       
        let b1 = b.clone();

        let (tx, rx): (Sender<Rc<RefCell<Data>>>, Receiver<Rc<RefCell<Data>>>) = mpsc::channel();
        
        //Sending is a non-blocking operation, the thread will continue immediately after sending its message
        tx.send(b1).unwrap();

        //recv is blocking operation
        let b2 = rx.recv().unwrap();

        //modifiy 
        b2.borrow_mut().a.push_str("XXXXXXXXXXX");
        b2.borrow_mut().b += 1000;
        b2.borrow_mut().c.push("A1".to_string());
        b2.borrow_mut().c.push("B1".to_string());
        b2.borrow_mut().c.push("C1".to_string());
        print!("b2:{:?} type_of:{}\n", b2, type_of(&b2));
        //b2:RefCell { value: Data { a: "AAAAXXXXXXXXXXX", b: 1333, c: ["A1", "B1", "C1"] } } type_of:alloc::rc::Rc<core::cell::RefCell<others::channel_::test::Data>>

        //print original b again
        print!("b:{:?} type_of:{}\n", b, type_of(&b));
        //b:RefCell { value: Data { a: "AAAAXXXXXXXXXXX", b: 1333, c: ["A1", "B1", "C1"] } } type_of:alloc::rc::Rc<core::cell::RefCell<others::channel_::test::Data>>

        //Drop  
        //~Data() Data { a: "AAAAXXXXXXXXXXX", b: 1333, c: ["A1", "B1", "C1"] }

        //Conclusion
        //look like channel send, recv reference instead of deep copy. So they all operate same object via Rc shared pointer.
    }

    //Thread use channel 1
    {
        static NUM_THREADS: i32 = 10;
        let (tx, rx): (Sender<usize>, Receiver<usize>) = mpsc::channel();
        let mut threads: Vec<Option<thread::JoinHandle<()>>> = Vec::new();

        for i in 0..NUM_THREADS {
            let tx_clone = tx.clone();
            let thread = thread::spawn(move || {
                //thread::sleep(Duration::from_secs(1));
                tx_clone.send(i as usize).unwrap();
            });
            //print!("thread:{:?} type_of:{}\n", thread, type_of(&thread));
            //thread:JoinHandle { .. } type_of:std::thread::JoinHandle<()>
            threads.push(Some(thread));
        }

        //why need Option for JoinHandle ?
        //Cause 'fn join(self) -> Result<T>' but not 'fn join(&self) -> Result<T>'
        //https://stackoverflow.com/a/41331922/11474144
        //https://stackoverflow.com/questions/42790156/how-to-join-thread-in-drop-function?noredirect=1&lq=1
        //https://learnku.com/docs/rust-lang/2018/ch20-03-graceful-shutdown-and-cleanup/4583
        for _ in 0..NUM_THREADS {
            let i = rx.recv().unwrap();

            //use option take to get Some and leave None
            if let Some(thread) = threads[i].take() {
                print!(" thread {} done.\n", i);
                thread.join().unwrap();
            }
        }

        print!("DONE\n\n");
    }

    //Thread use channel 2 (Better impl by use Drop for join)
    {
        #[derive(Debug)]
        struct Worker {
            index: i32,
            handle: Option<thread::JoinHandle<()>>, 
        };
    
        impl Drop for Worker {
            fn drop(&mut self) {
                if let Some(handle) = self.handle.take() {
                    handle.join().unwrap();
                    print!("~Worker() {} done\n", self.index);
                }
            }
        };

        static NUM_THREADS: i32 = 10;
        let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
        let mut workers: HashMap<i32,Worker> = HashMap::new();
        
        for i in 0..NUM_THREADS {
            let tx_clone = tx.clone();
            let thread = thread::spawn(move || {
                //thread::sleep(Duration::from_secs(1));
                tx_clone.send(i).unwrap();
            });
            workers.insert(i, Worker{index: i, handle: Some(thread)});
        }

        for _ in 0..NUM_THREADS {
            let i = rx.recv().unwrap();
            workers.remove(&i);
        }
        print!("DONE\n");
    }

    //(main)send req -> (thd) recv req -> (thd) send rep -> (main)recv rep
    {
        #[derive(Debug)]
        struct MessageData {
            rep_send: Sender<i32>,
            data: i32,
        };

        impl MessageData {
            pub fn new(rep_send: Sender<i32>, data: i32) -> Self {
                MessageData {
                    rep_send, data
                }
            }
        };

        #[derive(Debug)]
        struct MessageQueue {
            req_send: Sender<MessageData>,
            thread: Option<thread::JoinHandle<()>>, 
        };

        impl MessageQueue {
            pub fn new() -> Self {

                let (req_send, req_recv): (Sender<MessageData>, Receiver<MessageData>) = mpsc::channel();

                let thread = thread::spawn(move || {
                    for x in 0..100 {
                        let msg = req_recv.recv().unwrap();
                        print!("recv req {}\n", msg.data);
                        print!("send rep {}\n", msg.data);
                        msg.rep_send.send(msg.data).unwrap();
                    }
                });

                MessageQueue {
                    req_send:  req_send.clone(),
                    thread: Some(thread),
                }
            }

            pub fn PostMessage(&self, msg: MessageData) {
                self.req_send.send(msg);
            }

            pub fn PostMessageAndReply(&self, msg: i32) -> i32 {
                let (rep_send, rep_recv): (Sender<i32>, Receiver<i32>) = mpsc::channel();
                self.PostMessage(MessageData::new(rep_send, msg));
                rep_recv.recv().unwrap()
            }
        }

        impl Drop for MessageQueue {
            fn drop(&mut self) {
                if let Some(thread) = self.thread.take() {
                    thread.join().unwrap();
                    print!("~MessageQueue()  done\n");
                }
            }
        };

        let mq: MessageQueue = MessageQueue::new();
        
        for x in 0..100 {
            print!("send req {}\n", x);
            let n = mq.PostMessageAndReply(x);
            print!("recv resp {}\n", n);
        }
        /*
        let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
        
        print!("send req 123\n");
        mq.tx.send((tx.clone(), 123)).unwrap();

        let n = rx.recv().unwrap();
        print!("recv resp {}\n", n);
        */
        //mq.PostMessage(1233333);
        //mq.PostMessage(1233333);
        //thread::sleep(Duration::from_secs(1));
    }


}

