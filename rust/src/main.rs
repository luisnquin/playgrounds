fn main() {
    let letters = [
        "W", "e", "l", "c", "o", "m", "e", " ", "t", "o", " ", "m", "y", " ", "R", "u", "s", "t",
        " ", "p", "l", "a", "y", "g", "r", "o", "u", "n", "d",
    ];

    let mut word = String::new();

    for letter in letters {
        word += letter
    }

    println!("{}", word)
}

// #[derive(PartialEq, Debug)]
// // Declare Car struct to describe vehicle with four named fields
// struct Car {
//     color: String,
//     motor: Transmission,
//     roof: bool,
//     age: (Age, u32),
// }

// #[derive(PartialEq, Debug)]
// enum Age {
//     New,
//     Used,
// }

// #[derive(PartialEq, Debug)]
// // Declare enum for Car transmission type
// enum Transmission {
//     Manual,
//     SemiAuto,
//     Automatic,
// }

// // fn main() {
// //     let colors = ["Blue", "Green", "Red", "Silver"];

// //     let mut car: Car;

// //     let mut engine = Transmission::Manual;
// //     car = car_factory(String::from(colors[1]), engine, true, 0);
// //     println!(
// //         "Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles",
// //         car.age.0, car.roof, car.motor, car.color, car.age.1
// //     );

// //     println!("ninym ralei <3");

// //     engine = Transmission::SemiAuto;
// //     car = car_factory(String::from(colors[2]), engine, false, 500);
// // }

// // Build a new "Car" using the values of four input arguments
// // - color (String)
// // - motor (Transmission enum)
// // - roof (boolean, true if the car has a hard top roof)
// // - miles (u32)
// // Call the car_quality(miles) function to get the car age
// // Return an instance of a "Car" struct with the arrow `->` syntax
// fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
//     // Create a new "Car" instance as requested
//     // - Bind first three fields to values of input arguments
//     // - "age" field calls "car_quality" function with "miles" input argument
//     Car {
//         color: color,
//         motor: motor,
//         roof: roof,
//         age: car_quality(miles),
//     }
// }

// fn car_quality(miles: u32) -> (Age, u32) {
//     if miles == 0 {
//         return (Age::New, miles);
//     }

//     return (Age::Used, miles);
// }
