// // Days of the week
// let days = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];

// // Get the first day of the week
// let first = days[0];

// // Get the second day of the week
// let second = days[1];

// // вектор
//  let three_num = vec![12, 34, 43];
//  println!("Initial vector: {:?}", three_num )

//  let zeroes = vec![0; 5];
//  println! ("Zeroes: {:?}", zeroes);

//  //Выход
// Initial vector: [15, 3, 46]
// Zeroes: [0, 0, 0, 0, 0]
// let mut fruit = Vec::new();
// fruit.push("apple");
// fruit.push("Banana");
// println!("Fruit:{:?}", fruit);

// //Выход
// Fruits: ["Apple", "Banana"]
// println!("Fruit:{:?}", frut:pop());
// println!("Frut: {:?}", frut);

// Выход
// Pop off: Some("Banana")
// Fruits: ["Apple"]

// let mut index_vec = vec![15, 3, 46];
// let three = index_vec[1];
// println!("Vector: {:?}, three = {}", index_vec, three);

// Выход
// Vector: [15, 3, 46], three = 3

// index_vec[1] = index_vec[1] + 5;
// println!("Vector: {:?}", index_vec);

// Выход
// Vector: [15, 8, 46]

// Практика ==============================================================

// #[derive(PartialEq, Debug)]
// // Declare Car struct to describe vehicle with four named fields
// // Corrected code: "mileage" u32 field removed, "age" tuple field added
// struct Car {
//     color: String,
//     motor: Transmission,
//     roof: bool,
//     age: (Age, u32),
// }

// #[derive(PartialEq, Debug)]
// // Declare enum for Car transmission type
// enum Transmission {
//     Manual,
//     SemiAuto,
//     Automatic,
// }

// #[derive(PartialEq, Debug)]
// // Corrected code: Declare enum for Car age
// enum Age {
//     New,
//     Used,
// }

// //////////////////////////////////////////////////

// // Get the car quality by testing the value of the input argument
// // - miles (u32)
// // Create a tuple for the car quality with the age ("New" or "Used") and miles
// // Return a tuple with the arrow `->` syntax
// fn car_quality(miles: u32) -> (Age, u32) {
//     // Declare and initialize the return tuple value
//     // For a new car, set the miles to 0
//     let quality: (Age, u32) = if miles == 0 {
//         (Age::New, 0)
//     } else {
//         (Age::Used, miles)
//     };

//     // Return the completed tuple to the caller
//     quality
// }
// //////////////////////////////////////////////////

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
//     // Corrected code: "mileage" is replaced with "age"
//     // Corrected code: Bind "age" to tuple returned from car_quality(miles)
//     Car {
//         color: color,
//         motor: motor,
//         roof: roof,
//         age: car_quality(miles),
//     }
// }

// //////////////////////////////////////////////////

// fn main() {
//     // Create car color array
//     // Corrected code: 0 = Blue, 1 = Green, 2 = Red, 3 = Silver
//     let colors = ["Blue", "Green", "Red", "Silver"];

//     // Declare the car type and initial values
//     // Corrected code: Declare "car" as mutable "Car" struct
//     // Corrected code: Declare "engine" as mutable "Transmission" enum, initialize to "Manual"
//     let mut car: Car;
//     let mut engine = Transmission::Manual;

//     //////////////////////////////////////////////////

//     // Order 3 cars, one car for each type of transmission
//     // Corrected code: Index into `colors` array and vary color for the orders

//     // Car order #1: New, Manual, Hard top
//     car = car_factory(String::from(colors[0]), engine, true, 0);
//     println!(
//         "Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles",
//         car.age.0, car.roof, car.motor, car.color, car.age.1
//     );

//     // Car order #2: New, Semi-automatic, Convertible
//     engine = Transmission::SemiAuto;
//     car = car_factory(String::from(colors[1]), engine, false, 100);
//     println!(
//         "Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles",
//         car.age.0, car.roof, car.motor, car.color, car.age.1
//     );

//     // Car order #3: New, Automatic, Hard top
//     engine = Transmission::Automatic;
//     car = car_factory(String::from(colors[2]), engine, true, 200);
//     println!(
//         "Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles",
//         car.age.0, car.roof, car.motor, car.color, car.age.1
//     );
// }

// // Используйте условия if/else =========================
// if 1 == 2 {
//     println!("True");
//    } else {
//         println!(Folse);}

// if 1 == 2 {}else{}

//     let formal = true;
// let greeting = if formal { // if used here as an expression
//     "Good day to you."     // return a String
// } else {
//     "Hey!"                 // return a String
// };
// println!("{}", greeting)
// fn main (){   // prints "Good day to you."
// let form = true;
//  let greeting = if form {"ok"} else{"Not ok"};
//  println! ("{}", greeting);
// }
// let num = 500; // num variable can be set at some point in the program
// let out_of_range: bool;
// if num < 0 {
//     out_of_range = true;
// } else if num == 0 {
//     out_of_range = true;
// } else if num > 512 {
//     out_of_range = true;
// } else {
//     out_of_range = false;
// }
//=================👇Упражнение: Работа с условиями if/else👇
#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car age
enum Age {
    New,
    Used,
}

fn car_quality(miles: u32) -> (Age, u32) {
    if miles > 0 {
        // If car has accumulated miles, return tuple for Used car with current mileage
        (Age::Used, miles)
    } else {
        // Return tuple for New car with zero miles
        (Age::New, 0)
    }
}

fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    // Get the car age and mileage using the car_quality function
    let (age, mileage) = car_quality(miles);

    // Use if/else conditional expression to check car age and roof type
    if age == Age::Used {
        // Check roof type for a used car
        if roof {
            // If roof is a hard top, print details
            println!(
                "Prepare a used car: {:?}, {}, Convertible, {} miles\n",
                motor, color, mileage
            );
        }
    } else {
        // Check roof type for a new car
        if roof {
            // If roof is a hard top, print details
            println!(
                "Prepare a new car: {:?}, {}, Hard top, 0 miles\n",
                motor, color
            );
        }
    }

    Car {
        color,
        motor,
        roof,
        age: (age, mileage),
    }
}
fn main() {
    // Car order #1: New, Manual, Hard top
    car_factory(String::from("Orange"), Transmission::Manual, true, 0);

    // Car order #2: Used, Semi-automatic, Convertible
    car_factory(String::from("Red"), Transmission::SemiAuto, false, 565);

    // Car order #3: Used, Automatic, Hard top
    car_factory(String::from("White"), Transmission::Automatic, true, 3000);
}
