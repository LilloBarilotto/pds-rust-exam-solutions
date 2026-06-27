use std::result::Result;
use std::sync::mpsc::{self, Sender, Receiver, SendError};
use std::sync::{Arc, Mutex};

pub struct MultiChannel<u8> {
    channels: Mutex<Vec<Sender<u8>>>
}

impl MultiChannel<u8> {
    // crea un nuovo canale senza alcun ricevitore collegato
    pub fn new() -> Self  {
        Self { 
            channels: Mutex::new(vec![]), 
        }
    }
    pub fn subscribe(&self) -> Receiver<u8> {
        let (tx, rx) = mpsc::channel();

        let mut channel_guard = self.channels.lock().unwrap();
        channel_guard.push(tx);

        rx
    }
                
    pub fn send(&self, data: u8) -> Result<(), SendError<u8>> {
        let mut channel_guard = self.channels.lock().unwrap();

        channel_guard.retain(|s| s.send(data).is_ok());

        if channel_guard.len() == 0{
            return Err(SendError(data));
        }

        Ok(())
    }
}