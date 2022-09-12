use std::fmt;

fn main() {
    let mut car1 = car_factory("blue".to_string(), Transmission::Manual, true);
    println!("The car color is: {}",car1.color);
    let mut car2 = car_factory("red".to_string(), Transmission::Manual, true);
    println!("The car color is: {}",car2.color);
    
    let cars = [car1, car2];
    let bytes = [0,5];
    println!("{}",cars[1]);

    print!("{}",bytes[1]);
    
    
}


// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}


impl fmt::Display for Car {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Color {}, transmission {}, convertible {}, mileage {}", 
            self.color, self.transmission, self.convertible, self.mileage)
    }
}


#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

impl fmt::Display for Transmission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}",self)
    }
}


// Build a "Car" by using values from the input arguments
// - Color of car (String)
// - Transmission type (enum value)
// - Convertible (boolean, true if car is a convertible)
fn car_factory(color: String, transmission: Transmission, convertible: bool) ->Car {


    let car: Car = Car {color: color, transmission: transmission, convertible: convertible, mileage: 0};
    return car;
}

