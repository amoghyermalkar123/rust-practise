mod lifetime_ex;
mod lifetime_ex_two;
mod lifetime_advanced;
mod enum_ex;
mod box_ex;
mod traits_ex;

fn main() {
    let v1 = vec![11u8, 22];
    let result;
    {
        let v2 = vec![33u8];
        result = {
            let _x1: &Vec<u8> = &v1;
            let _x2: &Vec<u8> = &v2;
            _x1
        }
    }
    print!("{:?}", *result);
}
// more learnings on borrow checker

// 1.
// Then, the rule is simply that any object, in any point of the code, cannot have at the
// same time a mutable borrowing and some other borrowing.
// Put in other words, it can have:
// • no borrowing
// • or a single mutable borrowing
// • or a single immutable borrowing
// • or several immutable borrowings
// But it cannot have:
// • several mutable borrowings
// • nor a single mutable borrowing and one or more immutable
// borrowings

// 2.
// references should live less than the objects they borrow.
// just for this one rule to be followed, we have lifetime annotations,
// borrow checker rules, everything.

// 3.
// Inside any Rust function, you can refer only to:
// 1. objects owned by function arguments
// 2. objects owned by local variables
// 3. temporary objects
// 4. static objects
// 5. objects that are borrowed by function arguments, and that are
//    owned by some variable that preexists the current function
//    invocation

// 4.
// When a function returns a reference, such a reference cannot refer to an object
// owned by an argument of that function , or owned by a local variable of that
// function, or a temporary object, because when the function returns,
// every local variable, every function argument, and every temporary object is destroyed.
// So, such a reference would be dangling.
// Instead, a reference returned by a function can refer either to a static object,
// or to an object borrowed by a function argument.

// 5.
trait Tr {
    fn f<'a>(flag: bool, b: &'a i32, c: (char, &'a i32)) -> (&'a i32, f64, &'static i32);
}
// The "<'a>" clause is just a declaration. It means: <<In this function signature, a
// lifetime specifier is used; its name is "a">>. 
// The use of such "a" lifetime specifier means: “the first field of the return value
// borrows the same object already borrowed by the b argument and by the second field of
// the c argument, and so it must live less than such object”
// 'a is sort of like helping the rust borrow checker draw lines from reference
// to reference for it to apply borrow checking rules to avoid memory bugs
// tie this analogy with jonhoo's drawing line analogy from his rust for rustaceans book
