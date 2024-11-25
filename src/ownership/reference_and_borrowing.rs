pub fn main() {
    println!("# Reference And Borrowing");

    let name = String::from("I Komang Ary Antara");

    // Add "&" to reference into String
    let str_len = calculate_string_len(&name);
    println!("The '{name}' as {str_len:?} char");

    // Mutable reference
    let mut name = String::from("Ida Bagus Jatem Kamandalu");
    println!("{}", add_title_to_string(&mut name));

    // Mutable references have one big restriction: if you have a mutable reference to a value,
    // you can have no other references to that value.
    // This code that attempts to create two mutable references to s will fail:
    let mut s = String::from("Hi!");

    let r = &mut s;
    // let t = &mut s; -> error onccour because the variabel being reference as mutate reference
    // println!("{r}, {t}")

    let t = r;
    println!("{t}");

    // You can create new mutate reference if rust has dropped other one
    // You only can make one reference at a time
    let r = &mut s;
    println!("{r}");

    // The restriction preventing multiple mutable references 
    // to the same data at the same time allows for mutation but 
    // in a very controlled fashion. It’s something that 
    // new Rustaceans struggle with because most languages 
    // let you mutate whenever you’d like. The benefit of having this 
    // restriction is that Rust can prevent data races at compile time. 

    // We also cannot have a mutable reference while we have an immutable one to the same value.
    let r = &s; 
    let t = &s; 
    // let v = &mut s; Big problem

    println!("{r} {t} v");

    // But you can still make new reference when the other was dropped 
    let v = &mut s; 

    println!("{v}");

}

fn calculate_string_len(s: &String) -> usize {
    return s.len();
}

fn add_title_to_string(s: &mut String) -> String {
    s.push_str(", S.pd");
    s.to_string()
}
