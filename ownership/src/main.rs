fn main() {
    let s1 = String::from("hello");     // s1 is the owner of 'hello'
    let s2 = s1;                        // Ownership is transfered from s1 to s1
    // println!("{s1} world");          // will lead to an error as s1 has no ownership anymore
    println!("{s2} world");             // as s2 now has teh ownershup this will work


    let mut name = String::from("Niko");            // Ownership for name is at main
    name = print_greeting(name);                    // Ownership is transferred to  print_greeting
    println!("I own the string '{name}' again");    // As Ownership has been returned by the function we can still use it
}


fn print_greeting (name: String) -> String {
    println!("Welcome {name}");
    name                        // ensures Ownership is returned
}