use std::sync::{Mutex, Arc, Condvar};
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::any::Any;


//https://bennetthardwick.com/blog/dont-use-boxed-trait-objects-for-struct-internals/
/**
 *  Message
 **/
pub trait Message {
    fn as_any(&self) -> &dyn Any;
}

/**
 *  MessageQueueVector
 **/
pub struct MessageQueueVector {
    messages_mutex: Mutex<Vec<Option<Box<dyn Message + Send>>>>,
    cond: Condvar,
}

impl MessageQueueVector {
    pub fn new() -> Self {
        Self {
            messages_mutex: Mutex::new(Vec::new()),
            cond: Condvar::new(),
        }
    }

    pub fn get_message(&self) -> Option<Box<dyn Message + Send>> {
        let mut messages_mutex_guard = self.messages_mutex.lock().unwrap();
        while (*messages_mutex_guard).len() == 0 {
            messages_mutex_guard = self.cond.wait(messages_mutex_guard).unwrap();
        }
        (*messages_mutex_guard).remove(0)
    }

    pub fn get_message_timeout(&self, dur: Duration) -> Option<Box<dyn Message + Send>> {
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

    pub fn post_message(&self, message_option: Option<Box<dyn Message + Send>>) {
        let mut messages_mutex_guard = self.messages_mutex.lock().unwrap();
        (*messages_mutex_guard).push(message_option);
        self.cond.notify_all();
    }
}


/**
 *  MessageQueueHandlers
 **/
pub struct MessageQueueHandler {
    handler_mutex: Mutex<Option<Arc<dyn Fn(Option<Box<dyn Message + Send>>) -> bool  + Send + Sync>>>,
}

impl MessageQueueHandler {
    pub fn new() -> Self {
        Self {
            handler_mutex: Mutex::new(None),
        }
    }

    pub fn set_message_handler(&self, handler: Arc<dyn Fn(Option<Box<dyn Message + Send>>) -> bool  + Send + Sync>) {
        let mut handler_option = self.handler_mutex.lock().unwrap();
        *handler_option = Some(handler);
    }

    pub fn dispatch_message(&self, option_box_msg: Option<Box<dyn Message + Send>>) -> bool {
        let mut handler_option = self.handler_mutex.lock().unwrap();
        if let Some(handler) = &*handler_option {
            return handler(option_box_msg);
        }
        false
    }
}


pub trait MessageQueue {
    fn get_message(&self) -> Option<Box<dyn Message + Send>>;
    fn get_message_timeout(&self, duration: Duration) -> Option<Box<dyn Message + Send>>;
    fn post_message(&self, message_option: Option<Box<dyn Message + Send>>);
    fn set_message_handler(&self, handler: Arc<dyn Fn(Option<Box<dyn Message + Send>>) -> bool  + Send + Sync>);
    fn process_next_message(&self) -> bool;
}

/**
 *  MessageQueue
 **/
#[derive(Clone)]
pub struct MessageQueueBlock {
    //why need double arc in parent and here
    //this because the parent arc.clone() need child also support clone()
    message_queue_vector: Arc<MessageQueueVector>,
    message_queue_handlers: Arc<MessageQueueHandler>,
}

impl MessageQueueBlock {
    pub fn new() -> Self {
        Self {
            message_queue_vector: Arc::new(MessageQueueVector::new()),
            message_queue_handlers: Arc::new(MessageQueueHandler::new()),
        }
    }
}

impl MessageQueue for MessageQueueBlock {
    fn get_message(&self) -> Option<Box<dyn Message + Send>> {
        self.message_queue_vector.get_message()
    }

    fn get_message_timeout(&self, duration: Duration) -> Option<Box<dyn Message + Send>> {
        self.message_queue_vector.get_message_timeout(duration)
    }

    fn post_message(&self, message_option: Option<Box<dyn Message + Send>>) {
        self.message_queue_vector.post_message(message_option);
    }
    
    fn set_message_handler(&self, handler: Arc<dyn Fn(Option<Box<dyn Message + Send>>) -> bool  + Send + Sync>) {
        self.message_queue_handlers.set_message_handler(handler);
    }

    fn process_next_message(&self) -> bool {
        let message_option = self.get_message();
        if message_option.is_some()  {
            return self.message_queue_handlers.dispatch_message(message_option);
        }
        false
    }
}


/**
 *  MessageThread
 **/
pub struct MessageThread {
    message_queue: Arc<dyn MessageQueue + Send + Sync>,
    thread: Option<thread::JoinHandle<()>>, 
}

impl MessageThread {
    pub fn new(message_queue: Arc<dyn MessageQueue + Send + Sync>) -> Self {
        Self {
            message_queue,
            thread: None,
        }
    }

    pub fn start(&mut self) {
        if self.thread.is_some() {
            return;
        }

        let message_queue = self.message_queue.clone();
        let thread = thread::spawn(move || {
            while message_queue.process_next_message() {
                
            }
            print!("MessageThread done\n");
        });

        self.thread = Some(thread);
        print!("MessageThread()  start {}\n", self.thread.is_none());
    }

    pub fn stop(&mut self) {
        if self.thread.is_none() {
            return;
        }
        
        if let Some(thread) = self.thread.take() {
            self.message_queue.post_message(None);
            thread.join().unwrap();
            print!("MessageThread()  stopped {}\n", self.thread.is_none());
        }
    }
}

impl Drop for MessageThread {
    fn drop(&mut self) {
        self.stop();
    }
}



