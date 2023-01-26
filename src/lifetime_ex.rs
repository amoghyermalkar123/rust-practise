// ChatGPT
// Here are a few exercises to help you learn and practice Rust lifetimes:

// 1. Create a struct that holds a reference to a string. 
// Make sure to annotate the lifetime of the reference.
struct StringHolder<'a> {
    refer: &'a str,
}

// 2. Write a function that takes two references to strings and returns the longer one. Annotate 
// the lifetimes of the input references and the lifetime of the returned reference.
fn longer_string<'a>(str_a: &'a str, str_b: &'a str) -> &'a str {
    if str_a.len() > str_b.len() {
        return str_a
    }
    str_b
}

// 3. Create a struct that holds a reference to a vector. Annotate the lifetime of the reference.
struct VecHolder<'a, T> {
    refer: &'a Vec<T>,
}

// 4. Write a function that takes a reference to a vector and a closure, and calls the closure 
// with a reference to each element of the vector. Annotate the lifetimes of the input references and the lifetime of the closure.
fn refer_vec_clos<'a, T>(my_vec: &'a Vec<T>, closure: Box<dyn Fn(&T) -> &T >) -> Vec<&'a T> {
    let mut result = vec![];
    for item in my_vec.iter() {
        result.push(closure(item));
    }
    result
}
// chatgpt answer
fn dn<'a, T>(my_vec: &'a Vec<T>, closure: &'a dyn Fn(&T) -> &T ) -> Vec<&'a T> {
    let mut result = vec![];
    for item in my_vec.iter() {
        result.push(closure(item));
    }
    result
}

// 5. Write a function that takes a reference to a vector and returns a new vector that contains 
// only the elements of the input vector that satisfy a certain condition. Annotate the 
// lifetimes of the input and output references.
fn vecops<'a, T>(src: &'a mut Vec<T>, predicate: &'a dyn Fn(&T) -> bool) -> Vec<&'a T> 
{
    let mut result = vec![];

    for item in src.iter() {
        if predicate(item) {
            result.push(item);
        }
    }
    result
}

// NOTE: Remember that the key to understanding Rust's lifetime system is to think about 
// how references relate to each other and to the data they point to. Try to come up with clear, 
// consistent rules for how the lifetimes of your references relate to each other, 
// and make sure that the lifetimes you annotate are as precise as possible.
