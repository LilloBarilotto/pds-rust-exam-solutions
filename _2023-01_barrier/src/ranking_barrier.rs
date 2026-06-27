use std::sync::{Arc, Mutex, Condvar};
use std::sync::{Mutex, Condvar};

/// Una barriera ciclica che blocca N thread finché tutti non hanno chiamato `wait`.
/// Questa implementazione usa un contatore di "generazione" per evitare race condition
/// tra i thread che escono e quelli che entrano in un nuovo ciclo.

pub struct State{
    counter: usize,
    generation: usize,
}

pub struct Barrier {
    n: usize,
    state: Mutex<State>,
    cv : Condvar, // (contatore, generazione)
}

impl Barrier {
    pub fn new(n: usize) -> Barrier {
        Barrier {
            n,
            cv : Condvar::new(),
            state: Mutex::new(
                State{
                    counter: 0,
                    generation: 0
            })
        }
    }

    pub fn wait(&self) {  
        // se ci sono già N-1 nella barriera, non aggiungi al counter ma sblocchi gli altri!
        let mut guard = self.state.lock().unwrap();

        if guard.counter == self.n -1 {
            //sono il wait che farà uscire tutti della mia gen
            guard.counter = 0;
            guard.generation += 1;
            self.cv.notify_all();
        } else {
            // mi metto in wait anche io
            guard.counter +=1;
            let curr_gen = guard.generation;

            // esco se c'è stato un wait che ha cambiato la generazione e noi di curr_gen siamo free!
            let _unused = self.cv.wait_while(guard, |s| s.generation == curr_gen).unwrap();
        }

    }
}

/// Una versione della barriera che restituisce il "rango" di arrivo di ogni thread.
pub struct RankingBarrier {
    n: usize,
    state: Mutex<State>,
    cv : Condvar, // (contatore, generazione)
}

impl RankingBarrier {
    pub fn new(n: usize) -> RankingBarrier {
        RankingBarrier {
            n,
            cv : Condvar::new(),
            state: Mutex::new(
                State{
                    counter: 0,
                    generation: 0
            })
        }
    }

    pub fn wait(&self) -> usize {
        // se ci sono già N-1 nella barriera, non aggiungi al counter ma sblocchi gli altri!
        let mut guard = self.state.lock().unwrap();
        let res_counter = guard.counter;

        if guard.counter == self.n -1 {
            //sono il wait che farà uscire tutti della mia gen
            guard.counter = 0;
            guard.generation += 1;
            self.cv.notify_all();
        } else {
            // mi metto in wait anche io
            guard.counter +=1;
            let curr_gen = guard.generation;

            // esco se c'è stato un wait che ha cambiato la generazione e noi di curr_gen siamo free!
            let _unused = self.cv.wait_while(guard, |s| s.generation == curr_gen).unwrap();
        }

        res_counter +1
    }
}
