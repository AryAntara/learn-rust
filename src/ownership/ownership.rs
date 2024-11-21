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

}
