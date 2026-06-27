use std::sync::{Condvar, Mutex};
use std::time::Instant;

pub struct DelayedQueue <T: Send>{
    queue:  Mutex<Vec<(Instant, T)>>,
    cv: Condvar,
}

impl <T:Send> DelayedQueue<T> {
    pub fn new() -> Self{
        DelayedQueue { 
            queue: Mutex::new(Vec::new()),
            cv : Condvar::new()
        }
    }

    pub fn offer(&self, t:T, i: Instant){
        let mut guard = self.queue.lock().unwrap();
        guard.push((i, t));
        
        guard.sort_by(|a, b| a.0.cmp(&b.0));
        
        // se cambia qualcosa nel vec deve togliere
        self.cv.notify_all();
    }

    pub fn take(&self) -> Option<T> {
        let mut guard = self.queue.lock().unwrap();

        while !guard.is_empty() {

           let value = guard.remove(0);
           let now = Instant::now();
           
            if value.0 > now {
                let dur = value.0 - now;
                guard.insert(0, value);
                guard = self.cv.wait_timeout(guard, dur).unwrap().0;    
            } else{
                return Some(value.1);
            }
        }

        None
    }

    pub fn size(&self) -> usize{
        let guard = self.queue.lock().unwrap();
        guard.len() 
    }
}