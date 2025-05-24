use rand::Rng;
mod messages;
mod calculations;

fn main() {
    // random number generator
    let mut rng = rand::rng();

    // range in the braces is displayed by x..y
    // where x is the starting number and y the last possible number
    let random_number:u32 = rng.random_range(1..1000);

    println!("the generated number is: {random_number}");

    messages::greet("Niko");
    let difference = calculations::subtract(420, 69);
    println!("The difference is {difference}");
}
