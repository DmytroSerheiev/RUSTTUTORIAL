start new project👇
cargo new app

plagins
rust
Even Better TOML

comand
cargo run
cargo build --release

Змінні
оголошуетьця ключовим словом let
fn-оголошення функціі
println! - макрос для відображення значень змінних
типи данних
Цілі числа i32 i64 .....
Числа з плаваючою комою f32 f64 .....
Логічні значення
строка👇
let name: String = String::from("Dimon");
char👇
let symbol: char ='S';
let logic: bool = true;
let logic: bool = false;

Кортежі:👇
let tuple = ('👶', 87i32, true);
println!("Baybi {} age {} it is {} ?", tuple.0, tuple.1, tuple.2);
//=====================================================================
Структури
struct Student {
name: String,
remote: bool,
level: i32,
}
struct Grages(char, char, char, char, f64);
fn main() {
let user_1: Student = Student {
name: String::from("Dimon Serheiev"),
remote: true,
level: 5,
};
let user_2: Student = Student {
name: String::from("Diana Serheieva"),
level: 4,
remote: false,
};
let mark_1: Grages = Grages('A', 'B', 'C', 'D', 3.4);
let mark_2: Grages = Grages('D', 'A', 'D', 'C', 3.5);
println!(
"{},{},{},{},{},{},{},{},",
user_1.name, user_1.remote, user_1.level, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4
);
println!(
"{},{},{},{},{},{},{},{},",
user_2.name, user_2.remote, user_2.level, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4
);
}
Перечислення
enum WebEvent {
WELoad,
WEKeys(String, char),
WEClick {x: i64, y: i64}
}

//=================================
Вариант кортежа: WEKeys(KeyPress)

// Define a tuple struct #[derive(Debug)]
struct KeyPress(String, char);

// Define a classic struct #[derive(Debug)]
struct MouseClick { x: i64, y: i64 }

// Define the WebEvent enum variants to use the data from the structs
// and a boolean type for the page Load variant #[derive(Debug)]
enum WebEvent { WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress) }

fn main() {
// Instantiate a MouseClick struct and bind the coordinate values
let click = MouseClick { x: 100, y: 250 };
println!("Mouse click location: {}, {}", click.x, click.y);

    // Instantiate a KeyPress tuple and bind the key values
    let keys = KeyPress(String::from("Ctrl+"), 'N');
    println!("\nKeys pressed: {}{}", keys.0, keys.1);

    // Instantiate WebEvent enum variants
    // Set the boolean page Load value to true
    let we_load = WebEvent::WELoad(true);
    // Set the WEClick variant to use the data in the click struct
    let we_click = WebEvent::WEClick(click);
    // Set the WEKeys variant to use the data in the keys tuple
    let we_key = WebEvent::WEKeys(keys);

    // Print the values in the WebEvent enum variants
    // Use the {:#?} syntax to display the enum structure and data in a readable form
    println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_click, we_key);

}

Возврат значения функции 👇
fn goodbye(message: &str) {
println!("\n{}", message);
}

fn main() {
let formal = "Formal: Goodbye.";
let casual = "Casual: See you later!";
goodbye(formal);
goodbye(casual);
}

Возврат значения функции 👇

fn divide_by5(num: u32) -> u32 {
if num == 0 {
return 0;
}
num / 5
}
fn main() {
let num = 25;
println!("{} divide by 5 = {} ", num, divide_by5(num));
}
Итоги первого модуля
struct Car {
color: String,
transmission: Transmission,
convertible: bool,
mileage: u32,
}

#[derive(PartialEq, Debug)]

enum Transmission {
Manual,
SemiAuto,
Automatic,
}

fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
Car {
color,
transmission,
convertible,
mileage: 0,
}
}

fn main() {
let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
println!(
"Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}",
car.color, car.transmission, car.convertible, car.mileage
);

    car = car_factory(String::from("Silver"), Transmission::Automatic, true);
    println!(
        "Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}",
        car.color, car.transmission, car.convertible, car.mileage
    );

    car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    println!(
        "Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}",
        car.color, car.transmission, car.convertible, car.mileage
    );

}
 ## cargo test
