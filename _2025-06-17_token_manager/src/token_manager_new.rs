use std::{sync::{Condvar, Mutex}, time::Instant};

type TokenAcquirer = dyn Fn() -> Result<(String, Instant), String> + Sync+Send;

#[derive(PartialEq)]
pub enum TokenState{
    Empty,
    Pending,
    Valid {token: String, time: Instant},
}

pub struct TokenManager{
    state: Mutex<TokenState>,
    cv: Condvar,
    acquire_token : Box<TokenAcquirer>
}

impl TokenManager{
    pub fn new(acquire_token: Box<TokenAcquirer> ) -> Self {
               TokenManager { state: Mutex::new(TokenState::Empty), cv: Condvar::new(), acquire_token } 
    } 
    pub fn get_token(&self) -> Result<String, String> {
        loop{
            let mut guard = self.state.lock().unwrap();

            match &*guard {
                TokenState::Empty => {
                    *guard = TokenState::Pending;
                    drop(guard);

                    let value = (self.acquire_token)();
                    guard = self.state.lock().unwrap();

                    match value {
                        Ok((token, time)) => {
                            *guard = TokenState::Valid { token: token.clone(), time  };
                            self.cv.notify_all();
                            return Ok(token);
                        },
                        Err(e) => {
                            *guard = TokenState::Empty;
                            self.cv.notify_all();
                            return Err(e);
                        }
                    }
                },
                TokenState::Pending => {
                    guard = self.cv.wait_while(guard, |s| *s == TokenState::Pending).unwrap();
                },
                TokenState:: Valid { token, time } => {
                    if *time > Instant::now(){
                        return Ok(token.clone());
                    } else {
                        *guard = TokenState::Empty;
                    }
                }
            }
        }
    }
    pub fn try_get_token(&self) -> Option<String> {
         let guard = self.state.lock().unwrap();

         match &*guard {
            TokenState:: Valid { token, time } => {
                    if *time > Instant::now(){
                        return Some(token.clone());
                    } else {
                        return None;
                    }
            },
            _ => { None}
        }
    }
}