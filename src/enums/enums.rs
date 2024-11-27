struct User {
    username: String,
    age: i32,
}

enum Phone {
    Iphone,
    Samsung,
}

// Enum can store anything
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Enum with type
enum Printer<T> {
    Write(T),
}

impl Message {
fn exec(&self) {
        match self {
            Message::Quit => println!("Quit command!"),
            Message::Move { x, y } => println!("I will move for {x} x and {y} y"),
            Message::Write(str) => println!("{str}"),
            Message::ChangeColor(r, g, b) => println!("I need to change my color into {r} {g} {b}"),
        }
    }
}

impl Printer<i32> {
    fn o(&self) {
        match self {
            Printer::Write(c) => println!("{c:?}"),
        }
    }
}

impl Printer<User> {
    fn o(&self) -> String {
        match self {
            Printer::Write(c) => {
                let username = c.username.clone();
                if c.age > 12 {
                    println!("Mr. {username}")
                } else {
                    println!("Bocil {username}")
                }

                username
            }
        }
    }
}

pub fn main() {
    // Where structs give you a way of grouping together related fields and data,
    // like a Rectangle with its width and height,
    // enums give you a way of saying a value is one of a possible set of values.
    // For example, we may want to say that Rectangle is one of a set of possible
    // shapes that also includes Circle and Triangle. To do this, Rust allows us to encode these possibilities as an enum.
    let _my_phone = Phone::Samsung;
    let _jaka_phone = Phone::Iphone;

    let messager = Message::Quit;

    messager.exec();

    let messager = Message::Write(String::from("Hello world"));
    messager.exec();

    let printer = Printer::Write(12);
    printer.o();

    let username = String::from("Ary");
    match get_user(username) {
        Some(user) => {
            let printer = Printer::Write(user);
            printer.o();
        }
        None => {
            println!("User not found!")
        }
    }

    let username = String::from("Jaka");
    match get_user(username) {
        Some(user) => {
            let printer = Printer::Write(user);
            printer.o();
        }
        None => {
            println!("User not found!")
        }
    }
}

fn get_user(username: String) -> Option<User> {
    let users: [User; 3] = [
        new_user(String::from("Ary"), 12),
        new_user(String::from("Jaka"), 24),
        new_user(String::from("Bob"), 40),
    ];

    let mut found_user = None;
    for user in users {
        if user.username == username {
            found_user = Some(user);
        }
    }

    return found_user;
}

fn new_user(username: String, age: i32) -> User {
    User { username, age }
}
