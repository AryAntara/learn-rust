// The if let syntax lets you combine if and let
// into a less verbose way to handle values that
// match one pattern while ignoring the rest.
// Consider the program in Listing 6-6 that matches
// on an Option<u8> value in the config_max variable
// but only wants to execute code if the value is the Some variant.
enum usState {
    Alabama, 
    Arkansas
}

enum Coin {
    Penny,
    Pickle,
    Quarter(usState),
}

pub fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }


    let coin = Coin::Quarter(usState::Alabama);
    if let Coin::Quarter(state) = coin {
        println!("You coin have {state}")
    }
}
