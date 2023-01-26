// 1. 
// Implement a function that takes a mutable reference to a struct that 
// contains a string field, and a reference to another string, and appends 
// the second string to the first. The function should return a new string 
// that contains the concatenation of the two strings.
struct Stringer<'a> {
    refered_str: &'a str,
}

fn concat<'a>(input_struct: &'a mut Stringer, another: &str) -> &'a str {
    let result = input_struct.refered_str.to_owned() + another;
    input_struct.refered_str.to_owned().push_str(&result);
    input_struct.refered_str.as_ref()
}

// 2.
// struct Victor<T> {
//     vector: Vec<T>,
// }

// impl<T> Victor<T> {
//     fn iter(&mut self, predicate: &dyn Fn(T) -> bool) -> impl Iterator<Item = T> {
//         self.vector.iter().filter(predicate)
//     }
// }

// 3.
// Implement a struct that holds a reference to another struct and provides 
// a method that returns a mutable reference to a field of the inner struct. 
// The struct should take a lifetime parameter that represents the lifetime of the inner struct.
struct Child {
    state: u32,
}

struct Parent<'a> {
    child: &'a mut Child
}

impl<'a> Parent<'a> {
    fn get_mut_child(&'a mut self) -> &'a mut u32 {
        &mut self.child.state
    }
}

// 4.
// Create a struct that holds a reference to a Vec of structs, and provides 
// a method that returns an iterator over the structs that match a given predicate. 
// The struct should take a lifetime parameter that represents the lifetime of the 
// structs in the Vec.
// struct Unit;

// struct Holder<'a> {
//     units: &'a Vec<Unit>,
//     predicate: &'a (dyn Fn(&Unit) -> bool +'a)
// }

// impl<'a> Holder<'a> {
//     fn get_iter(&'a self) -> impl Iterator<Item = &'a Unit> {
//         self.units.iter().filter(self.predicate)
//     }
// }
