// 1.
// Create an enum with the four suits of a standard deck of cards 
// (hearts, diamonds, clubs, spades). The enum should have a method that 
// returns the number of cards in a suit.Create an enum with the four suits 
// of a standard deck of cards (hearts, diamonds, clubs, spades). 
// The enum should have a method that returns the number of cards in a suit.
enum Suite {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

impl Suite {
    fn get_num(self) -> u32 {
        match self {
            Self::Clubs | Self::Diamonds | Self::Hearts | Self::Spades => {13},
        }
    }
}

// 2.
// Create an enum that can store either an i32 or a f32. 
// The enum should have a method that returns the numeric value of the variant.
enum Num {
    I32(i32),
    F32(f32),
}

impl Num {
    fn get_i32<T>(self) -> i32 {
        match self {
            Self::F32(val) => {val as i32},
            Self::I32(val) => {val},
        }
    }
}

// 3.
// Create an enum that can store different types of shapes (rectangle, circle, triangle). 
// The enum should have a method that returns the area of the shape.
enum Shape {
    Rect {width: f32, height: f32},
    Circle {radius: f32},
}

impl Shape {
    fn get_area(self) -> f32 {
        match self {
            Self::Rect { width, height } => {
                width * height
            },
            Self::Circle { radius } => {
                2.0 * radius
            },
        }
    }
}

// 4.
// Create an enum that represents a binary tree. 
// Each variant should contain either a value or two other binary tree variants. 
// The enum should have a method that returns the sum of all the values in the tree.
enum BinaryTree {
    Leaf(i32),
    Node(Box<BinaryTree>,Box<BinaryTree>),
}
// Why box? Rust suggests recursive types sizes cant be known at compile time
// it suggests to use “indirection” which means that instead of storing a value directly, 
// we should change the data structure to store the value indirectly by storing a pointer 
// to the value instead.
impl BinaryTree {
    fn sum_all(self) -> i32 {
        match self {
            Self::Leaf(val) => {val},
            Self::Node(l, r) => {l.sum_all() + r.sum_all()}
        }
    }
}

// 5.
// Create an enum that can store different types of errors. 
// Each variant should contain an information about the error. 
// The enum should implement the std::error::Error trait and have a method 
// that returns a string describing the error.
use std::fmt::{Debug, Display};

#[derive(Debug)]
enum MyError {
    ParseFailed(String),
    FileNotFound(String),
}

use std::error::Error;

impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileNotFound(reason) | Self::ParseFailed(reason) => {
                write!(f, "operation failed because {}", reason)
            }
        }
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        match self {
           Self::FileNotFound(reason) | Self::ParseFailed(reason) => {
            reason.as_ref()
           },
        }
    }
}

// 6.
// Create an enum that can represent a type-safe and efficient dynamic dispatch system. 
// Each variant should contain a boxed trait object with a specific type. 
// The enum should have a method that takes a trait bound and calls the corresponding 
// method on the trait object.
trait Trait {
    fn something(&self);
}

enum TraitHolder {
    A(Box<dyn Trait>),
    B(Box<dyn Trait>),
}

impl TraitHolder {
    fn caller(&self) {
        match self {
            Self::A(obj) | Self::B(obj) => {
                obj.something()
            }
        }
    }
}

// 7.
use std::collections::HashMap;
enum Closure<'a, T> {
    Predicate(Box<(dyn Fn(&'a T) -> T + 'a)>),
}

// 8.
struct HashCallbacks<T> {
    machine: HashMap<String, Box<(dyn Fn(&T) -> bool)>>,
    once_machine:HashMap<String, Box<dyn FnOnce(&T) -> bool>>,
    mut_machine: HashMap<String, Box<dyn FnMut(&mut T) -> bool>>
}
