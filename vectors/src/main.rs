fn main() {

    let mut numbers: Vec<i32> = Vec::new();

    numbers.push(10);
    numbers.push(11);
    numbers.push(12);

    println!("{:?}", numbers);

    // creating a vector using the vector macro

    let vec_m_numbers = vec![1,2,3,4,5];
    println!("{:?}", vec_m_numbers);
}
