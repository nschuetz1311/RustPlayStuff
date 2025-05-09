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

fn calculate_area (shape:Shape) {
    match shape {
        Shape::Circle(radius) => println!("the area of the circle is {}", 3.14 * radius * radius),
        Shape::Rectangle(width, height) => println!("the area of the rectangle is {}", width * height),
        Shape::Square(side) => println!("the are of the square is {}", side * side),
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
}
