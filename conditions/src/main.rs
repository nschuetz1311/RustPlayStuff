fn main() {
    let is_weekend:bool = true;
    let activity = if is_weekend {"Hiking"} else {"Working"};
    println!("{} is todays activity", activity);


    /*For loop */
    let arr = [10, 12, 14, 18, 24];
    for element in arr {
        println!("{}", element)
    }

    /*while */
    let mut counter = 10;
    println!("Countdown!");
    while (counter > 0) {
        println!("{}", counter);
        counter -= 1;
    }
    println!("Take Off!");


    /*loop */
    let mut index = 1;
    loop {
        index += 1;
        println!("Index {}", index);

        if index == 100 {
            println!("Max index reached");
            break;
        }
    }

    /*fibonacci loop */
    let mut a: i32 = 0;
    let mut b: i32 = 1;
    let mut index: i32 = 0;
    loop {
        println!("{}",a);
        println!("{}",b);
        a = a + b;
        b = a + b;

        index += 1;
        if index == 5 { break;}
    }

    /*Average Temperature */
    let temp_of_week_arr: [f64; 7] = [70.1, 80.2, 75.3, 68.0, 77.7, 78.0, 69.4];
    let mut sum_of_temps: f64 = 0.0;
    
    for daily_temp in temp_of_week_arr {
        sum_of_temps += daily_temp;
    }

    println!("average temperature is: {}",(sum_of_temps / temp_of_week_arr.len() as f64));
    
}
