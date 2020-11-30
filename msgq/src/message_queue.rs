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
    fn handler_id(&self) -> i32;
    fn as_any(&self) -> &dyn Any;
}

/**
 *  MessageHandler
 **/
pub trait MessageHandler {
    fn on_message(&self, option_box_msg: Option<Box<dyn Message + Send>>) -> bool;
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
pub struct MessageQueueHandlers {
    handlers_mutex: Mutex<HashMap<i32, Arc<dyn MessageHandler + Send + Sync>>>,
}

impl MessageQueueHandlers {
    pub fn new() -> Self {
        Self {
            handlers_mutex: Mutex::new(HashMap::new()),
        }
    }

    pub fn register_message_handler(&self, handler_id: i32, handler: Arc<dyn MessageHandler + Send + Sync>) {
        let mut handlers_hash = self.handlers_mutex.lock().unwrap();
        let old_handler = handlers_hash.get(&handler_id);
        if !old_handler.is_none() {
            print!("handler {} already exist\n", handler_id);
            return;
        }

        handlers_hash.insert(handler_id, handler);
    }

    pub fn dispatch_message(&self, option_box_msg: Option<Box<dyn Message + Send>>) -> bool {
        let handlers_hash = self.handlers_mutex.lock().unwrap();
        if let Some(handler) = handlers_hash.get(&option_box_msg.as_ref().unwrap().handler_id()) {
            return handler.on_message(option_box_msg);
        }
        false
    }
}



/**
 *  MessageQueue
 **/
#[derive(Clone)]
pub struct MessageQueue {
    //why need double arc in parent and here
    //this because the parent arc.clone() need child also support clone()
    message_queue_vector: Arc<MessageQueueVector>,
    message_queue_handlers: Arc<MessageQueueHandlers>,
}

impl MessageQueue {
    pub fn new() -> Self {
        Self {
            message_queue_vector: Arc::new(MessageQueueVector::new()),
            message_queue_handlers: Arc::new(MessageQueueHandlers::new()),
        }
    }

    pub fn get_message(&self) -> Option<Box<dyn Message + Send>> {
        self.message_queue_vector.get_message()
    }

    pub fn get_message_timeout(&self, duration: Duration) -> Option<Box<dyn Message + Send>> {
        self.message_queue_vector.get_message_timeout(duration)
    }

    pub fn post_message(&self, message_option: Option<Box<dyn Message + Send>>) {
        self.message_queue_vector.post_message(message_option);
    }

    pub fn register_message_handler(&self, handler_id: i32, handler: Arc<dyn MessageHandler + Send + Sync>) {
        self.message_queue_handlers.register_message_handler(handler_id, handler);
    }

    pub fn process_next_message(&self) -> bool {
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
    message_queue: Arc<MessageQueue>,
    thread: Option<thread::JoinHandle<()>>, 
}

impl MessageThread {
    pub fn new(message_queue: Arc<MessageQueue>) -> Self {
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

