pub fn main() {
    println!("# Comment");
    println!(
        "{} <- normal comment in rust.",
        "// you can add two slash an the first of your comment"
    );

    comment()
}

/**
 * Doc comment in rust, you only can use top of a function 
 */
fn comment(){}
