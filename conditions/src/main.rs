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
}
