use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub trait Cache {
 fn put(&self, key: &str, value: Arc<Vec<u8>>, expires_in: Duration);
 fn get(&self, key: &str) -> Option<Arc<Vec<u8>>>;
}

pub struct State{
    map: HashMap<String, (Arc<Vec<u8>>, Instant)>,
    order: VecDeque<String>
}

pub struct LRUCache{
    state: Mutex<State>,
    size: usize
}


impl LRUCache {
       pub fn new(size :usize) -> Self{
        let state = State { 
            map : HashMap::new(),
            order: VecDeque::new(),
        };

        LRUCache { state: Mutex::new(state), size }
    }
}

impl Cache for LRUCache{
    fn get(&self, key: &str) -> Option<Arc<Vec<u8>>> {
        let mut guard = self.state.lock().unwrap();
        let now = Instant::now();

        if let Some((value, expiry)) = guard.map.get(key) {
            if *expiry > now {
                let val = Arc::clone(value);
                // Sposta la chiave in fondo: è la più recente
                if let Some(pos) = guard.order.iter().position(|x| x == key) {
                    guard.order.remove(pos);
                }
                guard.order.push_back(key.to_string());
                return Some(val);
            }
        }
        None
    }

    fn put(&self, key: &str, value: Arc<Vec<u8>>, expires_in: Duration) {
        if self.size == 0 {
            return;
        }

        let mut guard = self.state.lock().unwrap();
        let now = Instant::now();
        let expiry = now + expires_in;

        // Se la chiave è nuova e la cache è piena, dobbiamo fare spazio
        if !guard.map.contains_key(key) && guard.map.len() >= self.size {
            // 1. Identifica le chiavi scadute
            let expired_keys: Vec<String> = guard.map.iter()
                .filter(|(_, (_, exp))| *exp <= now)
                .map(|(k, _)| k.clone())
                .collect();

            if !expired_keys.is_empty() {
                // Rimuovi tutte le coppie scadute
                for k in expired_keys {
                    guard.map.remove(&k);
                    if let Some(pos) = guard.order.iter().position(|x| x == &k) {
                        guard.order.remove(pos);
                    }
                }
            } else {
                // 2. Se non ce ne sono di scadute, rimuovi quella LRU (la prima della coda)
                if let Some(lru_key) = guard.order.pop_front() {
                    guard.map.remove(&lru_key);
                }
            }
        }
        
        // Se la chiave esisteva già, rimuovila dalla vecchia posizione nella coda
        if let Some(pos) = guard.order.iter().position(|x| x == key) {
            guard.order.remove(pos);
        }

        // Inserimento o aggiornamento
        guard.map.insert(key.to_string(), (value, expiry));
        guard.order.push_back(key.to_string());
    }
}