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
use std::sync::{Mutex, Arc, Condvar, MutexGuard, LockResult, WaitTimeoutResult};
use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::time::Duration;
use std::collections::HashMap;
use std::any::Any;

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
                    for x in 0..10 {
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
                    print!("~MessageQueue()  done {}\n", self.thread.is_none());
                }
            }
        };

        let mq: MessageQueue = MessageQueue::new();
        
        for x in 0..10 {
            print!("send req {}\n", x);
            let n = mq.PostMessageAndReply(x);
            print!("recv resp {}\n", n);
        }
    }

    {
        #[derive(Debug)]
        struct B {
            data: i32,
        };

        impl B {
            pub fn new(data: i32) -> Self {
                B {
                    data
                }
            }

            pub fn DoB(&mut self) {
                self.data = self.data + 100;
                print!("DoB() {}\n", self.data);
            }
        };

        #[derive(Debug)]
        struct A {
            b: Option<B>
        };

        impl A {
            pub fn new(data: i32) -> Self {
                A {
                    b: Some(B::new(data))
                }
            }

            pub fn DoA(&mut self) {
                if self.b.is_some() {
                    self.b.as_mut().unwrap().DoB();
                }
            }
        };

        let mut a: A = A::new(123);
        a.DoA();
        a.DoA();
        a.DoA();
        a.DoA();
    }

    print!("\n");

    //https://www.philipdaniels.com/blog/2020/self-cloning-for-multiple-threads-in-rust/
    //https://github.com/PhilipDaniels/rtest/blob/master/rtest_core/src/thread_clutch.rs
    {
        #[derive(Debug)]
        struct MutexCondInner {
            mutex: Mutex<Vec<String>>,
            cond: Condvar,
        };

        impl MutexCondInner {
            pub fn new() -> Self {
                Self {
                    mutex: Mutex::new(Vec::new()),
                    cond: Condvar::new(),
                }
            }

            pub fn Lock(&self) -> MutexGuard<Vec<String>> {
                return self.mutex.lock().unwrap();
            }
            
            pub fn NotifyOne(&self) {
                self.cond.notify_one();
            }

            pub fn Wait<'a, T>(&self, started: MutexGuard<'a, T>) -> LockResult<MutexGuard<'a, T>> {
                return self.cond.wait(started);
            }
            
            pub fn WaitTimeout<'a, T>(&self, started: MutexGuard<'a, T>, dur: Duration) 
                -> LockResult<(MutexGuard<'a, T>, WaitTimeoutResult)> {
                return self.cond.wait_timeout(started, dur);
            }
        };

        #[derive(Debug, Clone)]
        struct MutexCond {
            inner: Arc<MutexCondInner>,
        };

        impl MutexCond {
            pub fn new() -> Self {
                Self {
                    inner: Arc::new(MutexCondInner::new()),
                }
            }

            pub fn Lock(&self) -> MutexGuard<Vec<String>> {
                return self.inner.Lock();
            }

            pub fn NotifyOne(&self) {
                self.inner.NotifyOne();
            }

            pub fn Wait<'a, T>(&self, started: MutexGuard<'a, T>) -> LockResult<MutexGuard<'a, T>> {
                return self.inner.Wait(started);
            }
            
            pub fn WaitTimeout<'a, T>(&self, started: MutexGuard<'a, T>, dur: Duration) 
                -> LockResult<(MutexGuard<'a, T>, WaitTimeoutResult)> {
                return self.inner.WaitTimeout(started, dur);
            }
        };

        let mut mcond = MutexCond::new();
        let mut mcond2 = mcond.clone();

        thread::spawn(move|| {
            thread::sleep(Duration::from_millis(50));
            print!("thd 1 ready to lock\n");
            {
                let mut vec = mcond2.Lock();
                (*vec).push("AAAA".to_string());
                (*vec).push("BBB".to_string());
                (*vec).push("CCC".to_string());
                print!("thd1 ready to notify_one\n");
                mcond2.NotifyOne();
            }
            
            thread::sleep(Duration::from_millis(200));
            print!("thd 2 ready to lock\n");
            {
                let mut vec = mcond2.Lock();
                (*vec).push("DDD".to_string());
                (*vec).push("EEE".to_string());
                (*vec).push("FFF".to_string());
                print!("thd1 ready to notify_one 2\n");
                mcond2.NotifyOne();
            }


            for i in 3..6 {
                print!("thd 3 ready to lock {}\n", i);
                let mut vec = mcond2.Lock();
                (*vec).push("GGG".to_string());
                (*vec).push("HHH".to_string());
                (*vec).push("III".to_string());
                print!("thd 3 ready to notify_one {}\n", i);
                mcond2.NotifyOne();
            }
            print!("thd  done\n");
        });

        {
            print!("main 1 ready to lock\n");
            let mut vec = mcond.Lock();
            print!("main 1 *ret:{:?} type_of:{} A\n", *vec, type_of(&*vec));
            //*vec:[] type_of:alloc::vec::Vec<alloc::string::String>
            vec = mcond.Wait(vec).unwrap();
            print!("main 1 *ret:{:?} type_of:{} B\n", *vec, type_of(&*vec));
            //*ret:["AAAA", "BBB", "CCC"] type_of:alloc::vec::Vec<alloc::string::String>
        }
        
        {
            print!("main 2 ready to lock\n");
            let mut vec = mcond.Lock();
            vec = mcond.Wait(vec).unwrap();
            print!("main 2 *vec:{:?} type_of:{}\n", *vec, type_of(&*vec));
        }

        while true {
            print!("main 3 ready to lock with timeout\n");
            let mut vec = mcond.Lock();
            let mut ret = mcond.WaitTimeout(vec, Duration::from_millis(500)).unwrap();
            if ret.1.timed_out() {
                print!("main 3 wait 3 is timeout\n");
                break;
            }
            print!("main *(ret.0):{:?} type_of:{}\n", *(ret.0), type_of(&*(ret.0)));
        }
    }

    {
        #[derive(Debug, Clone)]
        struct Data {
            s: String,
        };

        let mut v: Vec<Data> = Vec::new();
        v.push(Data{s: "AAA".to_string()});
        v.push(Data{s: "BBB".to_string()});
        v.push(Data{s: "CCC".to_string()});
       
        let bbb = v.remove(1);
        print!("bbb:{:?} v:{:?}\n", bbb, v);
    }

    print!("\n\n");

    //https://bennetthardwick.com/blog/dont-use-boxed-trait-objects-for-struct-internals/
    {
        /**
         *  Message
         **/
        pub trait Message {
            fn HandlerId(&self) -> i32;
            fn MessageId(&self) -> i32;
            fn Data(&self) -> &String;
            fn as_any(&self) -> &dyn Any;
        };
        
        /**
         *  MessageHandler
         **/
        pub trait MessageHandler {
            fn OnMessage(&self, option_box_msg: Option<Box<dyn Message + Send>>) -> bool;
        }

        /**
         *  MessageQueueSync
         **/
        struct MessageQueueSync {
            messages_mutex: Mutex<Vec<Option<Box<dyn Message + Send>>>>,
            cond: Condvar,
        };

        impl MessageQueueSync {
            pub fn new() -> Self {
                Self {
                    messages_mutex: Mutex::new(Vec::new()),
                    cond: Condvar::new(),
                }
            }

            pub fn GetMessage(&self) -> Option<Box<dyn Message + Send>> {
                let mut messages_mutex_guard = self.messages_mutex.lock().unwrap();
                while (*messages_mutex_guard).len() == 0 {
                    messages_mutex_guard = self.cond.wait(messages_mutex_guard).unwrap();
                }
                (*messages_mutex_guard).remove(0)
            }

            pub fn GetMessageTimeout(&self, dur: Duration) -> Option<Box<dyn Message + Send>> {
                let mut messages_mutex_guard = self.messages_mutex.lock().unwrap();
                let mut message_option: Option<Box<dyn Message + Send>> = None;
                
                if (*messages_mutex_guard).len() == 0 {
                    let mut ret = self.cond.wait_timeout(messages_mutex_guard, dur).unwrap();
                    if !ret.1.timed_out() {
                       message_option = (*(ret.0)).remove(0);
                    }
                } else {
                    message_option = (*messages_mutex_guard).remove(0);
                }

                message_option
            }

            pub fn PostMessage(&self, message_option: Option<Box<dyn Message + Send>>) {
                let mut messages_mutex_guard = self.messages_mutex.lock().unwrap();
                (*messages_mutex_guard).push(message_option);
                self.cond.notify_all();
            }
        };


        /**
         *  MessageQueueHandlers
         **/
        struct MessageQueueHandlers {
            handlers_mutex: Mutex<HashMap<i32, Arc<dyn MessageHandler + Send + Sync>>>,
        };

        impl MessageQueueHandlers {
            pub fn new() -> Self {
                Self {
                    handlers_mutex: Mutex::new(HashMap::new()),
                }
            }

            pub fn RegisterMessageHandler(&self, handler_id: i32, handler: Arc<dyn MessageHandler + Send + Sync>) {
                let mut handlers_hash = self.handlers_mutex.lock().unwrap();
                let old_handler = handlers_hash.get(&handler_id);
                if !old_handler.is_none() {
                    print!("handler {} already exist\n", handler_id);
                    return;
                }

                handlers_hash.insert(handler_id, handler);
            }

            pub fn DispatchMessage(&self, option_box_msg: Option<Box<dyn Message + Send>>) -> bool {
                let mut handlers_hash = self.handlers_mutex.lock().unwrap();
                if let Some(handler) = handlers_hash.get(&option_box_msg.as_ref().unwrap().HandlerId()) {
                    return handler.OnMessage(option_box_msg);
                }
                false
            }
        };



        /**
         *  MessageQueue
         **/
        #[derive(Clone)]
        struct MessageQueue {
            //why need double arc in parent and here
            //this because the parent arc.clone() need child also support clone()
            message_queue_sync: Arc<MessageQueueSync>,
            message_queue_handlers: Arc<MessageQueueHandlers>,
        };

        impl MessageQueue {
            pub fn new() -> Self {
                Self {
                    message_queue_sync: Arc::new(MessageQueueSync::new()),
                    message_queue_handlers: Arc::new(MessageQueueHandlers::new()),
                }
            }

            pub fn GetMessage(&self) -> Option<Box<dyn Message + Send>> {
                self.message_queue_sync.GetMessage()
            }

            pub fn GetMessageTimeout(&self, duration: Duration) -> Option<Box<dyn Message + Send>> {
                self.message_queue_sync.GetMessageTimeout(duration)
            }

            pub fn PostMessage(&self, message_option: Option<Box<dyn Message + Send>>) {
                self.message_queue_sync.PostMessage(message_option);
            }

            pub fn RegisterMessageHandler(&self, handler_id: i32, handler: Arc<dyn MessageHandler + Send + Sync>) {
                self.message_queue_handlers.RegisterMessageHandler(handler_id, handler);
            }

            pub fn ProcessNextMessage(&self) -> bool {
                let message_option = self.GetMessage();
                if message_option.is_some()  {
                    return self.message_queue_handlers.DispatchMessage(message_option);
                }
                false
            }
        };


        /**
         *  MessageThread
         **/
        struct MessageThread {
            message_queue: Arc<MessageQueue>,
            thread: Option<thread::JoinHandle<()>>, 
        };

        impl MessageThread {
            pub fn new(message_queue: Arc<MessageQueue>) -> Self {
                Self {
                    message_queue,
                    thread: None,
                }
            }

            pub fn Start(&mut self) {
                if self.thread.is_some() {
                    return;
                }

                let message_queue = self.message_queue.clone();
                let thread = thread::spawn(move || {
                    while message_queue.ProcessNextMessage() {
                        ;
                    }
                    print!("MessageThread done\n");
                });

                self.thread = Some(thread);
                print!("MessageThread()  start {}\n", self.thread.is_none());
            }

            pub fn Stop(&mut self) {
                if self.thread.is_none() {
                    return;
                }
                
                if let Some(thread) = self.thread.take() {
                    self.message_queue.PostMessage(None);
                    thread.join().unwrap();
                    print!("MessageThread()  stopped {}\n", self.thread.is_none());
                }
            }
        };

        impl Drop for MessageThread {
            fn drop(&mut self) {
                self.Stop();
            }
        };


        /**
         *  HelloMessage
         **/
        struct HelloMessage {
            handler_id: i32,
            test: String,
        };

        impl HelloMessage {
            pub fn new(handler_id: i32, test: String) -> Self {
                Self {
                    handler_id,
                    test,
                }
            }

            pub fn DoHelloMessageOnlyFunction(&self) {
                print!("HelloMessage::DoHelloMessageOnlyFunction() handler_id:{} test:{}\n", self.handler_id, self.test);
            }
        };

        impl Message for HelloMessage {
            fn HandlerId(&self) -> i32 {
                self.handler_id
            }

            fn MessageId(&self) -> i32 {
                123
            }

            fn Data(&self) -> &String {
                &(self.test)
            }

            fn as_any(&self) -> &dyn Any {
                self
            }
        };

        /**
         *  WorldMessage
         **/
        struct WorldMessage {
            handler_id: i32,
            test: String,
        };

        impl WorldMessage {
            pub fn new(handler_id: i32, test: String) -> Self {
                Self {
                    handler_id,
                    test,
                }
            }

            pub fn DoWorldMessageOnlyFunction(&self) {
                print!("WorldMessage::DoWorldMessageOnlyFunction() handler_id:{} test:{}\n", self.handler_id, self.test);
            }
        };

        impl Message for WorldMessage {
            fn HandlerId(&self) -> i32 {
                self.handler_id
            }

            fn MessageId(&self) -> i32 {
                123
            }

            fn Data(&self) -> &String {
                &(self.test)
            }

            fn as_any(&self) -> &dyn Any {
                self
            }
        };


        /**
         *  TestMessageHandler
         **/
        struct TestMessageHandler {
            ok: i32,
        };

        impl TestMessageHandler {
            fn new() -> Self {
                Self {
                    ok: 123,
                }
            }
        };

        impl MessageHandler for TestMessageHandler {
            fn OnMessage(&self, option_box_msg: Option<Box<dyn Message + Send>>) -> bool {
                if option_box_msg.is_none() {
                    return false;
                }

                let box_msg = option_box_msg.unwrap();
                print!("box_msg type_of:{} \n", type_of(&box_msg));
                //Box<dyn others::channel_::test::Message+core::marker::Send>

                if let Some(hello_msg) = box_msg.as_ref().as_any().downcast_ref::<HelloMessage>() {
                    print!("hello_msg type_of:{} \n", type_of(&hello_msg));
                    //Option<&others::channel_::test::TestMessage>

                    //do some HelloMessage only function
                    hello_msg.DoHelloMessageOnlyFunction();
                } else if let Some(world_msg) = box_msg.as_ref().as_any().downcast_ref::<WorldMessage>() {
                    print!("world_msg type_of:{} \n", type_of(&world_msg));
                    //Option<&others::channel_::test::TestMessage>

                    //do some WorldMessage only function
                    world_msg.DoWorldMessageOnlyFunction();
                }
                print!("\n");
                return true;
            }
        };

        print!("===== single thread \n");
        //single thread version
        let test_handler = Arc::new(TestMessageHandler::new());
        let message_queue = Arc::new(MessageQueue::new());
        message_queue.RegisterMessageHandler(1, test_handler);

        //single thread version
        for i in 0..10 {
            if i%2 == 0 {
                message_queue.PostMessage(Some(Box::new(HelloMessage::new(1, "HEEEEEEEELLO".to_string()))));
            } else {
                message_queue.PostMessage(Some(Box::new(WorldMessage::new(1, "WOOOOOOOORLD".to_string()))));
            }
            message_queue.ProcessNextMessage();
        }


        print!("===== multi thread \n");
        let test_handler = Arc::new(TestMessageHandler::new());
        let message_queue = Arc::new(MessageQueue::new());
        message_queue.RegisterMessageHandler(1, test_handler);

        let mut message_thread = MessageThread::new(message_queue.clone());
        message_thread.Start();

        //thread version
        for i in 0..10 {
            if i%2 == 0 {
                message_queue.PostMessage(Some(Box::new(HelloMessage::new(1, "HEEEEEEEELLO".to_string()))));
            } else {
                message_queue.PostMessage(Some(Box::new(WorldMessage::new(1, "WOOOOOOOORLD".to_string()))));
            }
        }


    }


    print!("\n");


    log!("done");
}

