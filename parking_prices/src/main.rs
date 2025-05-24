/* Step 1: Define the CarType enum - SUV, Sedan, Coupe */
enum CarType{
    SUV,
    Sedan,
    Coupe,
}

/* Step 2: Define the Vehicle enum
    Car -> Car types
    Truck -> Cargo Capacity
    Motorcycle -> boolean value?
*/
enum Vehicle{
    Car(CarType),
    Truck(i32),
    Motorcycle,
}

/* Step 3: Write a function to calculate parking rates */
fn get_parkig_rate (vehicle:Vehicle) -> i8 {
    match vehicle {
        Vehicle::Motorcycle => 10,
        Vehicle::Truck(cargo_capacity) => {
            if cargo_capacity > 10 {
                25
            } else {
                20
            }
        },
        Vehicle::Car(car_type) => {
            match car_type {
                CarType::Coupe => 10,
                CarType::Sedan => 15,
                CarType::SUV => 20,
            }
        }
    }
}

/* Step 4: Create a vehicle and pass it to the function */
fn main() {
    let forerunner = Vehicle::Car(CarType::SUV);
    let parking_rate = get_parkig_rate(forerunner);

    println!("The parking rate for this car is: {}",parking_rate);
}
