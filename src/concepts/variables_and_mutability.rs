pub fn main() {
    println!("# Variables And Mutability");

    // # Variable
    // by default, variables are immutable.
    // This is one of many nudges Rust gives you to write
    // your code in a way that takes advantage of the safety
    // and easy concurrency that Rust offers.
    // However, you still have the option to make your variables mutable.

    // this is a immutable variable.
    let age = 28;
    println!("{}", age);

    // !: cannot mutate immutable variable `age`
    // age = 28;

    // this is a mutable variable
    let mut name = "ary";
    println!("name before is {}", name);

    // change name with other one;
    name = "Ary Antara";
    println!("name after is {}", name);

    // # Constant
    // Like immutable variables,
    // constants are values that are bound to a name
    // and are not allowed to change, but there are a few differences
    // between constants and variables.
    const RELIGION: &str = "hindu";
    println!("My religion is {}", RELIGION);

    // # Shadowing
    // Rustaceans say that the first variable is shadowed by the second, 
    // which means that the second variable is what the compiler 
    // will see when you use the name of the variable.
    let favorite_food = "apple"; 
    {
        let favorite_food = "orange";
        println!("in bracket, my favorite is {}", favorite_food);
    }
    println!("out bracket, my favorite is {}", favorite_food);

    // can assign with different type, but it is reassign as new variable
    let favorite_food = true; 
    println!("now is it a boolean {}", favorite_food);

}
