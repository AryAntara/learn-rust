pub fn main() {
    println!("# Function");

    let height = 4;
    let width = 2;

    // Try call the calculate_range function
    let range = calculate_range(width, height);
    println!("The range is {}", range);
    println!("The width is {}", get_width((width, height)));
    println!("The height is {}", get_height((width, height)));
    println!("Perimiter of circle is {}", get_perimiter(7));
}

// A function with parameter
fn calculate_range(w: i32, h: i32) -> i32 {
    return w * h;
}

/**
 * Statements are instructions that perform some action and do not return a value.
 */
fn get_width(rect: (i32, i32)) -> i32 {
    return rect.0;
}

/**
 * Expressions evaluate to a resultant value
 */
fn get_height(rect: (i32, i32)) -> i32 {
    rect.1
}

/**
 * The Expressions can be used in a macro
 */
fn get_perimiter(rad: i32) -> f32 {
    let pi_times_twice = {
        let pi = 3.14;
        pi * 2.0
    };

    return pi_times_twice * rad as f32;
}
