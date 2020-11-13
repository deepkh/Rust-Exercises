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
use crate::message_queue::*;


/**
 *  HelloMessage
 **/
struct HelloMessage {
    handler_id: i32,
    test: String,
}

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
}

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
}

/**
 *  WorldMessage
 **/
struct WorldMessage {
    handler_id: i32,
    test: String,
}

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
}

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
}


/**
 *  TestMessageHandler
 **/
struct TestMessageHandler {
    ok: i32,
}

impl TestMessageHandler {
    pub fn new() -> Self {
        Self {
            ok: 123,
        }
    }
}

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
}


pub fn TestMessageQueue() {
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

