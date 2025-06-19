fn main() {

    let mut numbers: Vec<i32> = Vec::new();

    numbers.push(10);
    numbers.push(11);
    numbers.push(12);

    println!("{:?}", numbers);

    // creating a vector using the vector macro

    let vec_m_numbers = vec![1,2,3,4,5];
    println!("{:?}", vec_m_numbers);

    let mut fruits = vec!["apple", "bannana", "orange"];

    // add an element
    fruits.push("grape");
    println!("{:?}", fruits);

    // remove an element
    let removed_fruit = fruits.pop();
    println!("{:?}: removed {:?}", fruits, removed_fruit.unwrap());

    // accessing and modifying elements inside a vector
    let numbers_a = vec![100, 200, 300, 400, 500];

    let second = numbers_a[1];
    println!("The second element is: {}", second);

    // use get method
    match numbers_a.get(5) {
        Some(value) => println!("The value at index 5 is :{}", value),
        None => println!("No value at index 5"),
    }

    // iterating over a vector
    let animals = vec!["dog", "cat", "rabbit"];

    for animal in &animals {
        println!("{}", animal);
    }

    // mutable reference of vectors
    let mut numbers_b = vec![1, 2, 3, 4, 5];

    for entry in &mut numbers_b {
        *entry *= 2;
    }

    println!("the modified vector-output looks like this: {:?}", numbers_b);
}
