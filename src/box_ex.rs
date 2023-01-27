// 1.
// Create a struct Person with two fields: name and age. 
// Use Box<T> to store a Person value on the heap and print the value of its fields.
struct Person {
    name: String,
    age: i32,
}

fn person() {
    let p = Box::new(Person{
        name: String::from("amogh"),
        age: 24,
    });
    println!("name: {} and age: {}", p.name, p.age)
}

// 2.
// Create a struct Tree that has a value field of type i32 and a children 
// field of type Vec<Box<Tree>>. Create a function that takes a Box<Tree> 
// and prints the values of all nodes in the tree, in a pre-order traversal.
struct Tree {
    value: i32,
    left: Box<Tree>,
    right: Box<Tree>,
}

impl Tree {
    fn pot(&self) {
        println!("{}", self.value);
        self.left.pot();
        self.right.pot();
    }
}

// 3.
// Create a struct MyBox that has a single field of type T and implements the Drop trait. 
// In the drop method, print the value of the field. Create a main function that creates 
// a MyBox<i32> with the value of 42 and then drops it.
struct MyBox<T> {
    field:T
}

use std::ops::Drop;

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("dropping field");
    }
}

// 4.
// Create a function that takes a Box<T> and a closure that takes a reference to T 
// and returns a new value of T. The function should use the closure to update the 
// value in the Box<T> and return the new value. Test the function by calling it 
// with a Box<i32> and a closure that increments the value.
fn update_box<T>(boxed: Box<T>, update_fn: &dyn Fn(&T) -> T) -> T {
    let value = update_fn(& boxed);
    value
}
