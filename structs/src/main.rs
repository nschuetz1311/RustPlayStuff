struct User {
    name:String,
    email:String,
    active:bool,
    sign_in_count:u64,
}

fn greet_user(user:&User) {
    println!("Hello {}, you have signed in {} times", user.name, user.sign_in_count);
}

struct Color (i32,i32,i32); // RGB values
struct Point (f64,f64,f64); // xyz coordinates

struct Rectangle{
    width:u32,
    height:u32,
}

impl Rectangle{
    fn area(&self)->u32{
        self.width * self.height
    }
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

    let red = Color(0xff,0,0);
    let p1 = Point(100.9, 25.6, 37.6);
    let shape1 = Rectangle{
        width:5,
        height:15,
    };
    println!("the area of the rectangle is: {}", shape1.area());
}
