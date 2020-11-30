use libhelper::helper::type_of;
use std::sync::{Arc};
use std::any::Any;
use crate::message_queue::*;


/**
 *  HelloMessage
 **/
struct HelloMessage {
    test: String,
}

impl HelloMessage {
    pub fn new(test: String) -> Self {
        Self {
            test,
        }
    }

    pub fn do_hello_message_only_function(&self) {
        print!("HelloMessage::do_hello_message_only_function() test:{}\n", self.test);
    }
}

impl Message for HelloMessage {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/**
 *  WorldMessage
 **/
struct WorldMessage {
    test: String,
}

impl WorldMessage {
    pub fn new(test: String) -> Self {
        Self {
            test,
        }
    }

    pub fn do_world_message_only_function(&self) {
        print!("WorldMessage::do_world_message_only_function() test:{}\n", self.test);
    }
}

impl Message for WorldMessage {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn test_message_handler(option_box_msg: Option<Box<dyn Message + Send>>) -> bool {
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
        hello_msg.do_hello_message_only_function();
    } else if let Some(world_msg) = box_msg.as_ref().as_any().downcast_ref::<WorldMessage>() {
        print!("world_msg type_of:{} \n", type_of(&world_msg));
        //Option<&others::channel_::test::TestMessage>

        //do some WorldMessage only function
        world_msg.do_world_message_only_function();
    }
    print!("\n");
    return true;
}

pub fn test_message_queue() {
    print!("===== single thread \n");
    //single thread version
    let message_queue = Arc::new(MessageQueueBlock::new());
    message_queue.set_message_handler(Arc::new(test_message_handler));

    //single thread version
    for i in 0..10 {
        if i%2 == 0 {
            message_queue.post_message(Some(Box::new(HelloMessage::new("HEEEEEEEELLO".to_string()))));
        } else {
            message_queue.post_message(Some(Box::new(WorldMessage::new("WOOOOOOOORLD".to_string()))));
        }
        message_queue.process_next_message();
    }


    print!("===== multi thread \n");
    let message_queue = Arc::new(MessageQueueBlock::new());
    message_queue.set_message_handler(Arc::new(test_message_handler));

    let mut message_thread = MessageThread::new(message_queue.clone());
    message_thread.start();

    //thread version
    for i in 0..10 {
        if i%2 == 0 {
            message_queue.post_message(Some(Box::new(HelloMessage::new("HEEEEEEEELLO".to_string()))));
        } else {
            message_queue.post_message(Some(Box::new(WorldMessage::new("WOOOOOOOORLD".to_string()))));
        }
    }
}

