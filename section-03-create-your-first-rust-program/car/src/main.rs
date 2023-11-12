// Declare Car struct to describe vehicle with four named fields
struct Car {
    colour: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

fn main() {
    println!("Hello, world!");
}
