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
}
