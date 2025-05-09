enum TrafficLight {
    Red,
    Yellow,
    Green
}

/* Enums with Values */
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
}

/* Option Enums
 *      safe Division
 */
enum Name {
    Variant1,
    Variant2,
}


fn calculate_area (shape:Shape) {
    match shape {
        Shape::Circle(radius) => println!("the area of the circle is {}", 3.14 * radius * radius),
        Shape::Rectangle(width, height) => println!("the area of the rectangle is {}", width * height),
        Shape::Square(side) => println!("the are of the square is {}", side * side),
    }
}

// Option return value states that there may be None or Some return value. the value between <> is
// the return value for Some.
fn safe_division(numerator:f64, denominator:f64) -> Option<f64> {
    if (denominator == 0.0) {
        None
    } else {
        Some(numerator/denominator)
    }

}

fn main() {
    let light = TrafficLight::Red;
    /* Pattern matching */
    match light{
        TrafficLight::Red => println!("Stop"),
        TrafficLight::Yellow => println!("Be prepared to stop"),
        TrafficLight::Green => println!("Go!"),
    }

    let shape1 = Shape::Rectangle(30.0, 17.1);

    calculate_area(shape1);

    // the return type of the function is optional f64 therefore it can be None or Some which can
    // afterward be checked with match
    let result = safe_division(35.0, 0.0);
    match result{
        None => println!("Division through zero is not allowed!"),
        Some(value) => println!("Success! the result is {}", value),
    }

    /* If Let */
    /* If-Let allows to only check for 1 or more options within the enum.
     * whereas match always has to compute all possible return values.
     * the comparison apparently only wants one = instead of ==*/
    if let TrafficLight::Red = light {
        println!("you are not allowed to pass");
    } else if let TrafficLight::Yellow = light {
        println!("be prepared to stop!");
    }
}
