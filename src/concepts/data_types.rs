pub fn main() {
    println!("# Data Types");

    // # INT and UINT type
    // An integer is a number without a fractional component.
    // This type declaration indicates that
    // the value it’s associated with should be an unsigned integer
    // (signed integer types start with i instead of u)
    // that takes up 32 bits of space.
    // We can use any of these variants to declare the type of an integer value.
    let age_as_i16: i8 = 1;
    let age_as_i32: i32 = 1;
    let age_as_i64: i64 = 1;
    let age_as_i128: i128 = 1;

    println!(
        "{} in int 8, {} in int 32, {} in int 64, {} in int 128",
        age_as_i16, age_as_i32, age_as_i64, age_as_i128
    );

    // Can add _ to make the int easier to read
    let price: i32 = 100_000;
    println!("{}", price);

    // If you try to add value that out of range an int can hold it will throw err
    // Error message : literal out of range for `i8`
    // let price: i8 = 256;

    // # Floating Point
    // Rust also has two primitive types for floating-point numbers,
    // which are numbers with decimal points.
    // Rust’s floating-point types are f32 and f64,
    // which are 32 bits and 64 bits in size, respectively.
    // The default type is f64 because on modern CPUs,
    // it’s roughly the same speed as f32 but is capable of more precision.
    // All floating-point types are signed.

    let weight = 22.4;
    println!("{} must be f64", weight);

    let weight: f32 = 2.4;
    println!("{} it is f32", weight);

    // If you try to add double precision on the f32 it will not print all of the number
    let weight: f32 = 1.0019749432987;
    println!("{}", weight);

    // # Numberic Operation
    // Rust supports the basic mathematical operations
    // you’d expect for all the number types: addition, subtraction, multiplication, division, and remainder.
    // Integer division truncates toward zero to the nearest integer.

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;
    println!(
    "{} is addition operation result\n{} is subtraction operation result\n{} is multiplication operation result\n{} is division operation result\n{} is remainder operation result\n", sum, difference, product, quotient, remainder
        );

    // # Boolean type
    // As in most other programming languages,
    // a Boolean type in Rust has two possible values: true and false.
    // Booleans are one byte in size.
    // The Boolean type in Rust is specified using bool
    let mut is_married = true;

    println!("A year ago, married status is {}", is_married);
    is_married = false;

    println!("And now, married status is {}", is_married);

    // # Char type
    // Rust’s char type is the language’s most primitive alphabetic type.
    let a: char = 'A';
    println!("{} is a in uppercase", a);

    // # Tuple type 
    // A tuple is a general way of grouping together
    // a number of values with a variety of types into one compound type. 
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.

    let student: (&str, i32, bool) = ("a", 12, true);
    println!("{:?}", student);

    // tuple can be extacted into single variable 
    let (_, age, _) = student;

    println!("{}", age);

    // Tuple can accessed with period "." too 
    println!("{}", student.1);

    // # The Array type 
    // Another way to have a collection of multiple values is with an array. 
    // Unlike a tuple, every element of an array must have the same type. 
    // Unlike arrays in some other languages, arrays in Rust have a fixed length.
    let students: [&str; 2] = ["ary", "agus"];
    println!("{:?}", students)

}
