use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::sync::{RwLock};

/// Cache è una struttura dati generica e thread-safe che memorizza coppie
/// chiave/valore per un periodo non superiore a una durata stabilita per
/// ciascuna coppia.
///
/// - Nell'intervallo di validità della coppia, le richieste di lettura basate
///   sulla chiave restituiscono il valore corrispondente, se presente.
/// - Trascorso tale periodo, le richieste relative alla stessa chiave non
///   restituiscono più il valore.
/// - Le letture possono sovrapporsi temporalmente; le scritture devono avvenire
///   in accesso esclusivo.
/// - Durante le operazioni di scrittura si esegue un ciclo di pulizia delle
///   coppie scadute per evitare la saturazione della struttura.
pub struct Cache<K: Eq + Hash, V> {
    hashmap: RwLock<HashMap<K, (Arc<V>, Instant)>>,
}

impl<K: Eq + Hash, V> Cache<K, V> {
    /// Crea una nuova istanza.
    pub fn new() -> Self {
        let hashmap = RwLock::new(HashMap::<K, (Arc<V>, Instant)>::new());

        Self { hashmap}
    }

    /// Restituisce il numero di coppie presenti nella mappa.
    pub fn size(&self) -> usize {
        let lock = self.hashmap.read();

        lock.unwrap().len()
    }

    /// Inserisce la coppia k/v con durata pari a d.
    pub fn put(&self, k: K, v: V, d: Duration) {
        let mut lock_hashmap = self.hashmap.write().unwrap();
        let instant = Instant::now();

        lock_hashmap.insert(k, (Arc::new(v), instant + d));
        lock_hashmap.retain(|k, v| instant < v.1);
    }   

    /// Rinnova la durata dell'elemento rappresentato dalla chiave k.
    /// Restituisce true se la chiave esiste e non è scaduta,
    /// altrimenti restituisce false.
    pub fn renew(&self, k: &K, d: Duration) -> bool {
        let mut lock_hashmap = self.hashmap.write().unwrap();
        let instant = Instant::now();
        let mut state = false;

        if lock_hashmap.contains_key(k) {
            let (v, i) = lock_hashmap.get(k).unwrap();
            if instant <= *i {
                state = true;

                let new_expiration = Instant::now() + d;
                lock_hashmap.get_mut(k).unwrap().1 = new_expiration;
            }
        }

        lock_hashmap.retain(|k, v| instant < v.1);

        return state;
    }

    /// Restituisce None se la chiave k è scaduta o non è presente nella cache;
    /// altrimenti restituisce Some(a), dove a è di tipo Arc<V>.
    pub fn get(&self, k: &K) -> Option<Arc<V>> {
        let lock_hashmap = self.hashmap.read().unwrap();

        if let Some((arc_v, i)) = lock_hashmap.get(k) {
            if Instant::now() > *i {
                return None
            }

            let arc_clone = arc_v.clone();
            return Some(arc_clone);
        } 

        return None;
    }


}