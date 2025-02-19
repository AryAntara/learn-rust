mod concepts;
mod enums;
mod ownership;
mod structs;

use concepts::{comments, control_flow, data_types, functions, variables_and_mutability};
use enums::enums as enumeration;
use ownership::{ownership as ownership_concept, reference_and_borrowing};
use structs::{defination, method};

use crate::enums::match_op;

fn main() {
    println!("Concepts");

    // Variable and mutability;
    variables_and_mutability::main();

    // Data Types
    data_types::main();

    // Function
    functions::main();

    // Comments
    comments::main();

    // Controll flow
    control_flow::main();

    println!("Ownership");

    // What is the ownership?
    ownership_concept::main();

    // Reference And Borrowing
    reference_and_borrowing::main();

    println!("Struct");

    // Defination
    defination::main();

    method::main();

    println!("Enum");

    enumeration::main();

   match_op::main(); 
}
