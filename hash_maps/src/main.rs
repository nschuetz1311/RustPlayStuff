use std::collections::HashMap;



fn main() {
    // Hashmaps consinsts of pairs of Key and Value
    let mut population = HashMap::new();

    population.insert("Tokyo", 37400100);
    population.insert("London", 17400100);
    population.insert("Munich", 7400100);

    // Match with the Key will return only the Value of the Pair if a match is found
    // therefore the var in some will be the population number in my case.
    match population.get("Tokyo") {
        Some(&population_number) => println!("population of Tokyo:  {}", population_number),
        None => println!("No Match found for Tokyo"),
    }

    // Updating the Population of Tokyo
    population.insert("Tokyo", 47000000);
    println!("{:?}", population);

    // Removing entry from HashMap
    population.remove("Tokyo");
    println!("{:?}", population);

    //iterating through HashMaps needs to address both key and value
    for (city, pop) in &population {
        println!("{} has a population of {}", city, pop);
    }

    // if only one part of the K/V pair is needed use the respective method
    // to access them
    for value in population.values() {
        println!("population: {}", value);
    }

    for key in population.keys() {
        println!("{}", key);
    }

    // entry will check if the hashmap has content matching () and if not insert it
    // however it does not modify the already existing entry
    population.entry("Paris").or_insert(4000000);
    population.entry("London").or_insert(20000000);

    println!("{:?}", population);


    // HashMap Perfomance
    // for better perfomance use: String, i32 or usize as Keys
    let mut scores = HashMap::with_capacity(10);

    for i in 0..5 {
        scores.insert(i, i * 10);
    }

    // the capacity that is actually used might defer from the one I defined
    // Rust will allocated a capacity which it internally asseses as useful/correct
    // the minimum capacity however will be 10, the number defined be the developer
    println!("{:?}", scores);
    println!("capacity: {}", scores.capacity());
}
