use std::{collections::VecDeque, sync::{Condvar, Mutex}};

pub struct State<E:Send>{
    queue: VecDeque<E>,
    is_closed: bool
}

pub struct MpMcChannel<E:Send>{
    state: Mutex<State<E>>,
    n: usize,
    cv: Condvar
}

impl <E:Send> MpMcChannel<E>{
    pub fn new(n: usize) -> Self{
        MpMcChannel {
            state: Mutex::new(State {
                queue: VecDeque::new(),
                is_closed: false
            }),
            n,
            cv: Condvar::new()
        }
    }

    pub fn send(&self, e: E) -> Option<()> {
        let mut guard =  self.state.lock().ok()?; 

        guard = self.cv.wait_while(guard, |s| s.queue.len() == self.n && !s.is_closed).ok()?;

        if guard.is_closed {
            None
        } else {
            guard.queue.push_back(e);
            self.cv.notify_all();
            Some(())
        }
    }

    pub fn recv(&self) -> Option<E> {
        let mut guard = self.state.lock().ok()?;

        guard = self.cv.wait_while(guard, |s| s.queue.is_empty() && !s.is_closed).ok()?;
        
        if guard.is_closed && guard.queue.is_empty() {
            None
        } else {
            let e = guard.queue.pop_front();
            self.cv.notify_all();

            e
        } 
    }
    
    pub fn shutdown(&self) -> Option<()> {
        let mut guard = self.state.lock().ok()?;

        if guard.is_closed {
            // Già chiuso
            None
        } else{
            guard.is_closed = true;
            self.cv.notify_all();
            Some(())
        }
    }
}