use std::time::Instant;
use std::sync::{Arc, Condvar, Mutex};

type TokenAcquirer = dyn Fn() -> Result<(String, Instant), String> + Sync;

#[derive(PartialEq)]
enum TokenManagerState {
    Empty,
    Pending,
    Valid(String, Instant)
}


pub struct TokenManager{
     token : Arc<Mutex<TokenManagerState>>,
     get_token_fn: Box<TokenAcquirer>,
     cv: Condvar
}


impl TokenManager {
    pub fn new(acquire_token: Box<TokenAcquirer> ) -> Self {
        TokenManager { 
            token: Arc::new(Mutex::new( TokenManagerState::Empty)),
            get_token_fn: acquire_token,
            cv: Condvar::new()
        }
    }

    pub fn get_token(&self) -> Result<String, String> {
        let mut token_guard = self.token.lock().unwrap();
        loop {
            match &(*token_guard){
                TokenManagerState::Empty => {
                    *token_guard = TokenManagerState::Pending;
                    drop(token_guard);
                    
                    let result= (self.get_token_fn)();

                    let mut token_guard = self.token.lock().unwrap();                

                    match result {
                        Ok( (token_string, instant) ) => {
                            *token_guard = TokenManagerState::Valid(token_string.clone(), instant);
                            return Ok(token_string);
                        },
                        Err(e) => {
                            *token_guard = TokenManagerState::Empty;
                            return Err(e);
                        }, 
                    }
                },
                TokenManagerState::Pending => {
                    token_guard = self.cv.wait_while( token_guard, |guard| *guard == TokenManagerState::Pending).unwrap();
                },
                TokenManagerState::Valid(token_string, instant) => {
                    if *instant < Instant::now() {
                        *token_guard = TokenManagerState::Pending;
                    } else {
                        return Ok(token_string.clone());
                    }
                }
            }

        }
    }

    pub fn try_get_token(&self) -> Option<String> {
        let token_guard = self.token.lock().unwrap();
        if let TokenManagerState::Valid(token_string, instant) = &*token_guard {
            if *instant < Instant::now() {
                return None;
            }
            Some(token_string.clone())
        } else {
            None
        }
    }
}