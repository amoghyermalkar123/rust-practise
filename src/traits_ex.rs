// 1.
// Create a struct MyBox<T> that has a single field of type T and implements 
// the Deref and DerefMut trait. The deref method should return a reference to the 
// T field, and the deref_mut method should return a mutable reference to the T field.
struct MyBox<T> {
    field: T
}

use std::ops::{Deref, DerefMut};

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.field
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.field
    }
}

// 2.
// Create a struct MyWrite that has a single field data of type Vec<u8>. 
// Implement the std::io::Write trait for MyWrite so that it adds the data 
// passed to it to the data field.
struct MyWrite {
    data: Vec<u8>,
}
use std::io::Write;
impl Write for MyWrite {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        for byte in buf{
            self.data.push(*byte);
        }
       Ok(buf.len()) 
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

// 3.
// Create a struct MyIterator that implements the Iterator trait and 
// generates an infinite sequence of the Fibonacci numbers.
struct MyIterator{
    prev_one: u64,
    prev_two: u64,
}

use std::iter::Iterator;

impl Iterator for MyIterator {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
       let new = self.prev_one + self.prev_two;

       self.prev_one = self.prev_two;
       self.prev_two = new;
       Some(new)
    }
}
