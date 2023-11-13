#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
struct Car {
    colour: String,
    transmission: Transmission,
    convertible: bool,
    age: (Age, u32) 
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Old,
}

fn main() {
    // We have orders for three new cars!
    // We'll declare a mutable car variable and reuse it for all the cars
    let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
    println!(
        "Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}",
        car.colour, car.transmission, car.convertible, car.age.1
    );

    car = car_factory(String::from("Silver"), Transmission::Automatic, true);
    println!(
        "Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}",
        car.colour, car.transmission, car.convertible, car.age.1
    );

    car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    println!(
        "Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}",
        car.colour, car.transmission, car.convertible, car.age.1
    );
}

// Build a "Car" by using values from the input arguments
// - Colour of car (String)
// - Transmission type (enum value)
// - Convertible (boolean, true if car is a convertible)
fn car_factory(colour: String, transmission: Transmission, convertible: bool) -> Car {
    Car {
        colour: colour,
        transmission: transmission,
        convertible: convertible,
        age: (Age::New, 0),
    }
}
