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

// 1. Define a Book struct with fields for the title, author, and number of pages.
struct Book{
    title:String,
    author:String,
    number_of_pages:u32,
}

// 2. Implement a method on the Book struct:
//     get_summary to return a summary of the book as a string.
impl Book{
    fn get_summary(&self)->String{
        format!("{}, was written by {} and has {} pages",self.title, self.author, self.number_of_pages)
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

    let name = "Jane";
    let age = 30; 
    let introduction = format!("Hi, my name is {} and I am {} years old.", name, age); 
    println!("{}", introduction); 

    // 3. In the main function, create an array of books.
    let book_array = [
        Book {
            title: String::from("Book1"),
            author: String::from("Author1"),
            number_of_pages: 500,
        },
        Book {
            title: String::from("Book2"),
            author: String::from("Author2"),
            number_of_pages: 400,
        },
        Book {
            title: String::from("Book3"),
            author: String::from("Author3"),
            number_of_pages: 170,
        },
    ];

    // 4. Use a for loop to iterate through the array and print the summary of each book using the get_summary method.
    for book in book_array {
        println!("{}",book.get_summary());
    }
}
