use std::sync::{Mutex, mpsc::{self,Receiver, Sender}};

pub struct Subscription<Msg : Clone + Send > {
    receiver: Receiver<Msg>, 
}

pub struct Dispatcher <Msg : Clone + Send > {
    senders: Mutex<Vec<Sender<Msg>>>
}

impl <Msg: Clone + Send > Dispatcher<Msg>{

    pub fn new()-> Self {
        let senders : Vec<Sender<Msg>> = vec![];
        Dispatcher { 
            senders : Mutex::new(senders),
        }
    }

    pub fn subscribe(&self) -> Subscription<Msg> {
        let (sender, receiver) = mpsc::channel();
        self.senders.lock().unwrap().push(sender);
        
        Subscription { receiver }
    }

    pub fn dispatch(&self, msg: Msg) {
        let mut senders_mutex = self.senders.lock().unwrap();
        senders_mutex.retain(|s| s.send(msg.clone()).is_ok());
    }
}

impl <Msg: Clone + Send> Subscription<Msg>{
    pub fn read(&self) -> Option<Msg> {
        self.receiver.recv().ok()
    }
}