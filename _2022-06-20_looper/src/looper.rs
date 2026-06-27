use std::thread;
use std::sync::mpsc::{Sender, channel};
pub struct Looper<Message>
where Message: Send + 'static,
{
    sender: Option<Sender<Message>>,
    jh : Option<thread::JoinHandle<()>>,
}

impl<Message> Looper<Message>
where Message: Send + 'static,
{
    pub fn new<Process, Cleanup>(mut process: Process, mut cleanup: Cleanup) -> Self
    where
        Process: FnMut(Message) + Send + 'static,
        Cleanup: FnOnce() + Send + 'static
    {
        let (sender, receiver) = channel();

        let jh: thread::JoinHandle<()> = thread::spawn(move || {
            while let Ok(msg) = receiver.recv() {
                process(msg);
            }
            cleanup();
        });

    
        Looper {
            sender: Some(sender),
            jh: Some(jh),
        }
    }

    pub fn send(&self, msg: Message) {
        if let Some(s) = &self.sender {
            s.send(msg).unwrap();
        }
    }
}

impl<Message> Drop for Looper<Message>
where Message: Send + 'static,
{
    fn drop(&mut self) {
        // 1. Rimuoviamo il sender dalla struct e lo droppiamo.
        // Questo chiude il canale perché era l'ultimo sender rimasto.
        self.sender.take(); 
        
        // 2. Ora che il canale è chiuso, il thread uscirà dal loop recv() e chiamerà cleanup()
        if let Some(jh) = self.jh.take() {
            jh.join().unwrap();
        }
    }
}
