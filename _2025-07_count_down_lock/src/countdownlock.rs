use std::sync::{Arc, Condvar, Mutex, WaitTimeoutResult};
use std::time::Duration;

/// CountDownLock permette a uno o più thread di attendere che un gruppo
/// di operazioni eseguite da altri thread siano eseguite.
/// Incapsula un contatore ed offre tre metodi thread-safe.

pub struct CountDownLock {
    // TODO: campi interni (es. Mutex<usize> + Condvar)
    counter: Arc<(Mutex<usize>, Condvar)>,
}

impl CountDownLock {
    /// Inizializza la struttura, impostando ad n il contatore.
    pub fn new(n: usize) -> Self {

        let counter = Arc::new((Mutex::new(n), Condvar::new()));

        CountDownLock {
            counter
        }
    }

    /// Decrementa il contatore, se maggiore di 0.
    pub fn count_down(&self) {

        let  (lock, cv) = &*self.counter;

        let mut count = lock.lock().unwrap();

        if *count > 0 {
            *count -=1;
            
            if *count == 0 {
                drop(count);
                cv.notify_all();
            }
        }
    }

    /// Blocca l'esecuzione del chiamante senza consumare cicli di CPU,
    /// finché il contatore non diventa zero.
    pub fn wait(&self) {
        
        let (lock, cv) = &*self.counter;

        let count = lock.lock().unwrap();

        let _ = cv.wait_while(count, |l| *l != 0);
    }

    /// Blocca l'esecuzione del chiamante senza consumare cicli di CPU,
    /// in attesa che il contatore raggiunga 0 per una durata massima
    /// pari a `d`, restituendo il risultato dell'attesa.
    pub fn wait_timeout(&self, d: Duration) -> WaitTimeoutResult {
        let (lock, cv) = &*self.counter;

        let count = lock.lock().unwrap();

        cv.wait_timeout_while(count, d, |l| *l != 0).unwrap().1
    }
}