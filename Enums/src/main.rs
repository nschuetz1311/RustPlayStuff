enum TrafficLight {
    Red,
    Yellow,
    Green
}


fn main() {
    let light = TrafficLight::Red;
    /* Pattern matching */
    match light{
        TrafficLight::Red => println!("Stop"),
        TrafficLight::Yellow => println!("Be prepared to stop"),
        TrafficLight::Green => println!("Go!"),
    }
}
