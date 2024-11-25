/**
 * To define a struct, we enter the keyword struct and name the entire struct.
 * A struct’s name should describe the significance of the pieces of data being grouped together.
 * Then, inside curly brackets, we define the names and types of the pieces of data, which we call fields.
 */
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn main() {
    let ary = User {
        active: false,
        username: String::from("Ary"),
        email: String::from("Name@gmail.com"),
        sign_in_count: 1,
    };

    let username = get_username(&ary);
    let email = ary.email;
    let sign_in_count = ary.sign_in_count;
    println!("Username {username} from {email:?} with {sign_in_count} sign count.");

    let diva = new_user(String::from("Diva"), String::from("diva@gmail.com"), false);
    println!("Active User is {}", diva.active);

    // Use diva as other user property
    let ivy = User {
        active: false,
        ..diva
    };

    let email = &ivy.email;
    println!("ivy email {email}");
    println!("{ivy:?}");
    dbg!(&ivy);
}

fn get_username(u: &User) -> String {
    u.username.clone()
}
// Shorthand
fn new_user(username: String, email: String, active: bool) -> User {
    User {
        active,
        username,
        email,
        sign_in_count: 1,
    }
}
