use std::{sync::{Arc, Condvar, Mutex}, usize};
pub struct Exchanger<T: Send> {
    message: Arc<(Mutex<(Option<T>,Option<T>, usize)>, Condvar)>
}

impl<T:Send> Exchanger<T> {
    fn new() -> Exchanger<T>{
        let message = Arc::new((
            Mutex::new((None::<T>,None::<T>,0 as usize)),
            Condvar::new()
        ));
        Exchanger { message }
    }

    fn exchange(&self, t:T) -> Option<T> {
        let (lock, cv) = &*self.message;
        let mut tuple = lock.lock().unwrap();


        let index = tuple.2;
        match index {
            0 => {
                tuple.0 = Some(t);
                tuple.2 = 1;
            }
            1 => {
                tuple.1 = Some(t);
                tuple.2 = 0
            }
            usize::MAX => return None,
            _ => unreachable!()
        }
        cv.notify_one();

        tuple = cv.wait_while(tuple, |t| t.2 != index).unwrap();

        if tuple.2 == usize::MAX {
            return None;
        }

        let result = match index {
            0 => tuple.1.take(),
            1 => tuple.0.take(),
            usize::MAX => None,
            _ => unreachable!()
        };
        cv.notify_one();

        result

    } 
}

impl<T: Send> Drop for Exchanger<T> {
    fn drop(&mut self) {
        let (lock, cv) = &*self.message;
        let mut tuple = lock.lock().unwrap();
        tuple.2 = usize::MAX;
        cv.notify_one();
    }
}


