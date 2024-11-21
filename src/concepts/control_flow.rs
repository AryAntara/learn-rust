pub fn main() {
    println!("# Control Flow");

    let age = 10;

    // # If condition
    if age > 5 {
        println!("You are old!");
    } else if age == 4 {
        println!("You are same with me");
    } else {
        println!("you are to young");
    }

    let is_married = true;
    let label = if is_married {
        "Married"
    } else {
        "Not yet married"
    };

    println!("My married status {label}");

    // # Loop
    let mut count = 0;
    loop {
        println!("i  will saving!");
        count += 1;
        if count > 10 {
            break;
        }
    }

    // Naming the loop
    let mut count = 0;
    let total_count = loop {
        count += 1;
        println!("I am the parent!");
        'child_1: loop {
            count += 1;
            println!("I am the first child!");
            'child_2: loop {
                count += 1;
                println!("I am the second child!");

                if count > 10 {
                    break 'child_2;
                }
            }

            if count > 20 {
                break 'child_1;
            }
        }

        if count > 30 {
            break count;
        }
    };

    println!("Total count is {total_count}");

    // # White statement
    let mut is_young = true;
    let mut age = 0;
    while is_young {
        println!("I am younger :v");
        age += 1;
        is_young = !(age > 18);
    }

    if !is_young {
        println!("I am older")
    }

    // # For statement
    for item in 1..100 {
        println!("{item:?}. Ini si a")
    }
}
