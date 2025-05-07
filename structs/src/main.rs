struct User {
    name:String,
    email:String,
    active:bool,
    sign_in_count:u64,
}

fn greet_user(user:&User) {
    println!("Hello {}, you have signed in {} times", user.name, user.sign_in_count);
}

fn main() {
    let user1 = User{
        name:String::from("Niko"),
        email:String::from("niko@email.com"),
        active:true,
        sign_in_count:5,
    };
    // println!("name: {}", user1.name);
    greet_user(&user1);
}
