use std::vec;

// 1.
struct One<'a> {
    string: &'a str,
}

struct Two<'a> {
    string: &'a str,
    one: &'a One<'a>,
}

struct Three<'a> {
    string: &'a str,
    two: &'a Two<'a>,
}

fn third_and_concat<'a>(third: &'a Three) -> String {
    let all_three: String;
    all_three = third.string.to_owned() + third.two.string + third.two.one.string;
    all_three
}

// 2.
fn vec_clos<'a, T>(vecs: &'a Vec<T>, predicate: &(dyn Fn(&T) -> bool + 'a)) -> Vec<&'a T> {
    let mut new_vec = vec![];
    for item in vecs {
        if predicate(item) {
           new_vec.push(item); 
        }
    }
    new_vec
}
// The line bool + 'a is called a "trait bound" in Rust. It's used to constrain the type of the predicate function.
// In this case, &dyn Fn(&T) -> bool is the trait bound that specifies that the predicate function is a type that implements the Fn trait and takes a reference to a T and returns a bool.
// The + 'a is an additional trait bound that specifies that the type implementing the Fn trait should also have a lifetime parameter 'a. This means that the predicate function can reference data with a lifetime of 'a.
// This is important, because it ensures that the predicate function can only operate on references to data that lives at least as long as the input vector, which is also annotated with the lifetime 'a. This guarantees that the predicate function can't operate on references to data that has already been freed, and it helps to prevent data race conditions and other safety issues.
// Overall, the line bool + 'a is used to ensure that the predicate function is aware of the lifetime of the input vector and it can operate on the input values in a safe way.

// 3.
fn vec_struct<'a, T>(vecs: &'a mut Vec<T>, closure: & (dyn Fn(&mut T) -> () + 'a)) {
    for item in vecs {
        closure(item);
    }
}

// 4.
struct Something {
    some: u32,
}

fn somer(input: &Something) -> &u32 {
    &input.some
}
