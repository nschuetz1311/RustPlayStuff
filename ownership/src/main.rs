fn main() {
    let s1 = String::from("hello");     // s1 is the owner of 'hello'
    let s2 = s1;                        // Ownership is transfered from s1 to s1
    // println!("{s1} world");          // will lead to an error as s1 has no ownership anymore
    println!("{s2} world");             // as s2 now has teh ownershup this will work


    let mut name = String::from("Niko");            // Ownership for name is at main
    name = print_greeting(name);                    // Ownership is transferred to  print_greeting
    println!("I own the string '{name}' again");    // As Ownership has been returned by the function we can still use it

    let name2 = String::from("Alex");
    print_greeting_ref(&name2);                     // instead of transferring the ownership we can only reference the string
    println!("I still own the string '{name2}'");   // this leads to no ownership loss when using it in the main function again

    /*Slice Type */
    let message = String::from("Hello World!");
    let slice = &message[..5];          // to slice always use a reference to the string.(&) 'x..y' specifies where the slice starts.
    println!("{slice}")                       // if starting at 0 you can skip x and if going to the end you can skip y
}


fn print_greeting (name: String) -> String {
    println!("Welcome {name}");
    name                        // ensures Ownership is returned
}

// change the variable type to &(reference)type to ensure you can access it correctly
fn print_greeting_ref (name: &String) {
    println!("Welcome {name}");
}