use std::{sync::{Arc, Condvar, Mutex}, thread};

pub struct ExecutionLimiter{
    limit: usize,
    current_usage: Mutex<usize>,
    cv: Condvar,
}

impl ExecutionLimiter {
    pub fn new(limit: usize) -> Self {
        ExecutionLimiter {
            limit,
            current_usage: Mutex::new(0), 
            cv: Condvar::new(),
        }
    }
    
    pub fn execute<R>(&self, f: &dyn Fn() ->R) -> R {
        let mut curr_usage = self.current_usage.lock().unwrap();

        curr_usage = self.cv.wait_while(curr_usage,|count| *count >= self.limit).unwrap();
        *curr_usage +=1;

        drop(curr_usage); 
        let res = f();

        // Possibile variante per gestire il decremento in caso di panic sulla funzione f
        // let handle = thread::spawn(move || {
        // f()
        //});
        //let res = handle.join().unwrap();

        curr_usage = self.current_usage.lock().unwrap();
        *curr_usage -=1;
        self.cv.notify_one();

        res
    }
}