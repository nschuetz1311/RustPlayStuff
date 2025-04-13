fn main() {
    let mut value = 9;
    value = 10;
    println!("the value is {}", value);

    let x = 46;
    // Shadowing
    let x = x + 1;
    println!("the value of x is {}", x);

    let small_value: i8 = 100;
    let sample_float: f32 = -700.35;
    println!("the small_value is: {}", small_value);
    println!("the float_value is: {}", sample_float);

    /*Arrays */
    let number = [1, 2, 3, 4, 5];
    println!("Element at index 0 is: {}", number[0]);

    /*Tuples */
    let person = ("Niko", 32, 1.86);
    println!("Name: {}, Age: {}, Height: {}m", person.0,person.1,person.2);

    /*const */
    const PI:f32 = 3.14;
    println!("Value of pi is {}", PI);
}
