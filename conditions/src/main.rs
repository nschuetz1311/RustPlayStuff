fn main() {
    let is_weekend:bool = true;
    let activity = if is_weekend {"Hiking"} else {"Working"};
    println!("{} is todays activity", activity);


    /*For loop */
    let arr = [10, 12, 14, 18, 24];
    for element in arr {
        println!("{}", element)
    }
}
