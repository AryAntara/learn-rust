pub fn main() {
    // Ownership is a set of rules that govern how a Rust program manages memory.
    // All programs have to manage the way they use a computer’s memory while running.
    // Some languages have garbage collection that regularly looks for
    // no-longer-used memory as the program runs; in other languages,
    // the programmer must explicitly allocate and free the memory.
    // Rust uses a third approach: memory is managed through a system
    // of ownership with a set of rules that the compiler checks.
    // If any of the rules are violated, the program won’t compile.
    // None of the features of ownership will slow down your program while it’s running.
    // @see https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

    // First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

    // Each value in Rust has an owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.

    // # Variable Scoope

    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward

        // do stuff with s
        println!("{s}")
    } // this scope is now over, and s is no longer valid

    // The String type
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}"); // This will print `hello, world!`

    // Variables and Data Interacting with Move
    // Multiple variables can interact with the same data in different ways in Rust
    let x = 5;
    let y = x;
    println!("{x:?} x and {y:?} y");

    // We can probably guess what this is doing:
    // “bind the value 5 to x; then make a copy of the value in x and bind it to y.”
    // We now have two variables, x and y, and both equal 5.
    // This is indeed what is happening,
    // because integers are simple values with a known,
    // fixed size, and these two 5 values are pushed onto the stack.

    // Let use String
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{s1}") < it will be an error, because the ownership of the String was changed
    println!("{s2}");

    // Variables and Data Interacting with Clone
    // If we do want to deeply copy the heap data of the String,
    // not just the stack data, we can use a common method called clone.
    // We’ll discuss method syntax in Chapter 5,
    // but because methods are a common feature in many programming languages,
    // you’ve probably seen them before.

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    // Ownership And Function
    // The mechanics of passing a value to a function are similar to those when assigning a value to a variable.
    // Passing a variable to a function will move or copy, just as assignment does.
    let space = String::from(" ");
    take_ownership(space);

    // Not logger exists
    // println!("{space:?}")

    // Return Values and Scope
    // Returning values can also transfer ownership.
    let mut hello_world = String::from("hello world");
    hello_world = take_ownership(hello_world);

    println!("{hello_world:?}");

    // Returning multiple value using tuple
    let name = String::from("I Komang ary antara");
    let (name, length) = get_length(name);

    println!("My name is {name} and my name length is {length}")
}

fn get_length(s: String) -> (String, usize) {
    let len = s.len();
    return (s, len);
}

fn take_ownership(s: String) -> String {
    s
}
