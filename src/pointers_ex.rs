use core::fmt;
use std::collections::HashMap;
use std::error;
// 1.
// Create a struct that holds a vector of Rc pointers to other structs. Write a method on the struct that takes an index and increments a field in the struct that the Rc pointer at that index points to.
use std::rc::Rc;

struct RcHolder<MyStruct> {
    holder: Vec<Rc<MyStruct>>,
}

struct MyStruct {
    field: i32,
}

impl RcHolder<MyStruct> {
    fn update_at_index(&mut self, idx: usize, val: i32) -> Option<()> {
        let m = Rc::get_mut(&mut self.holder[idx]);
        if let Some(ms) = m {
            ms.field = val;
        }
        Some(())
    }
}

// 2.
// Implement a thread-safe cache using an Arc of a Mutex. The cache should have a method for inserting key-value pairs and another method for retrieving values by key.
use std::sync::{Arc, Mutex};

#[derive(Debug)]
enum CacheError {
    PutFailed(String),
    GetFailed(String),
}

impl fmt::Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PutFailed(reason) => {
                write!(f, "put operation failed because {}", reason)
            }
            Self::GetFailed(reason) => {
                write!(f, "get operation failed because {}", reason)
            }
        }
    }
}

impl error::Error for CacheError {
    fn description(&self) -> &str {
        match self {
            CacheError::GetFailed(reason) | CacheError::PutFailed(reason) => reason.as_ref(),
        }
    }
}

struct Cache {
    internal: Arc<Mutex<HashMap<String, i32>>>,
}

impl Cache {
    fn put(&mut self, key: String, val: i32) -> Result<(), CacheError> {
        let k = self.internal.as_ref().lock();
        if let Ok(mut hold) = k {
            let result = hold.insert(key, val);
            if let Some(_) = result {
                Ok(())
            } else {
                Err(CacheError::PutFailed(String::from("could not insert key")))
            }
        } else {
            Err(CacheError::PutFailed(String::from(
                "failed to get lock on hashmap",
            )))
        }
    }

    fn get(&self, key: &str) -> Result<Option<i32>, CacheError> {
        if let Ok(hold )= self.internal.as_ref().lock() {
        Ok(hold.get(key).cloned()) 
        } else {
            Err(CacheError::GetFailed(String::from("failed to get key")))
        }
    }
}
