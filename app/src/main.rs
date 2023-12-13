// //? A package - пакет
// $ cargo new <project-name>
// my-project
// ├── src
// │  └── main.rs
// └── Cargo.toml
// //? A crate - Ящик
// cargo new:
// $ cargo new --lib my-library
//      Created library `my-library` package
// my-library
// ├── src
// │  └── lib.rs
// └── Cargo.toml
// //?A module - Модуль
// Константи
// Введіть псевдоніми
// Функції
// Структури
// Enums
// Риси
// implблоки
// Інші модулі
// mod math {
//     type Complex = (f64, f64);
//     pub fn sin(f: f64) -> f64 { /* ... */ }
//     pub fn cos(f: f64) -> f64 { /* ... */ }
//     pub fn tan(f: f64) -> f64 { /* ... */ }
// }

// println!("{}", math::cos(45.0));

// // Declare a private struct
// struct Foo;

// // Declare a public struct with a private field
// pub struct Bar {
//     field: i32,
// }

// // Declare a public enum with two public variants
// pub enum State {
//     PubliclyAccessibleVariant,
//     PubliclyAccessibleVariant2,
// }
// mod authentication;

// fn main() {
//     let mut user = authentication::User::new("jeremy", "super-secret");

//     println!("The username is: {}", user.get_username());
//     user.set_password("even-more-secret");
// }
//? Вправа – Наочність
// mod car_factory {
//     pub fn build_car() {
//         println!("Honk honk!");
//     }
// }

// fn main() {
//     car_factory::build_car();
// }
//? Вправа - Модулі
mod text_processing {
    pub mod letters {
        pub fn count_letters(text: &str) -> usize {
            text.chars().filter(|c| c.is_alphabetic()).count()
        }
    }

    pub mod numbers {
        pub fn count_numbers(text: &str) -> usize {
            text.chars().filter(|c| c.is_numeric()).count()
        }
    }
}

fn count_letters_and_numbers(text: &str) -> (usize, usize) {
    let number_of_letters = text_processing::letters::count_letters(text);
    let number_of_numbers = text_processing::numbers::count_numbers(text);
    (number_of_letters, number_of_numbers)
}

fn main() {
    assert_eq!(count_letters_and_numbers("221B Baker Street"), (12, 3));
    assert_eq!(count_letters_and_numbers("711 Maple Street"), (11, 3));
    assert_eq!(count_letters_and_numbers("4 Privet Drive"), (11, 1));
}
