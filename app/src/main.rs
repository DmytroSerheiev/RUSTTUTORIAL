//! Модульні тести в Rust
// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }
// #[test]

// fn add_works() {
//     assert_eq!(add(1, 2), 3);
//     assert_eq!(add(10, 12), 22);
//     assert_eq!(add(5, -2), 3);
// }

// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }
// //? Очікувані невдачі
// #[test]
// #[should_panic]
// fn add_works() {
//     assert_eq!(add(1, 2), 3);
//     assert_eq!(add(10, 12), 22);
//     assert_eq!(add(5, -2), 4);
// }

// //? Ігноруйте тести
// #[test]
// #[ignore = "not yet reviewed by the Q.A. team"]
// fn add_negatives() {
//     assert_eq!(add(-2, -2), -4)
// }
//  //! Більшість модульних тестів входять до підмодуля з #[cfg(test)]атрибутом.
//  //! fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// #[cfg(test)]
// mod add_function_tests {
//     use super::*;

//     #[test]
//     fn add_works() {
//         assert_eq!(add(1, 2), 3);
//         assert_eq!(add(10, 12), 22);
//         assert_eq!(add(5, -2), 3);
//     }

//     #[test]
//     #[should_panic]
//     fn add_fails() {
//         assert_eq!(add(2, 2), 7);
//     }

//     #[test]
//     #[ignore]
//     fn add_negatives() {
//         assert_eq!(add(-2, -2), -4)
//     }
// }
//! Вправа
// pub fn is_even(num: i32) -> bool {
//     num % 2 == 0
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn is_true_when_even() {
//         assert!(is_even(2));
//     }

//     #[test]
//     fn is_false_when_odd() {
//         assert!(!is_even(1));
//     }
// }
// ? тести з документації
// $ cargo new --lib basic_math
// $ cd basic_math
// pub fn add(a: i32, b: i32) -> i32 {
//     a + b
// }
pub struct Pizza {
    pub topping: String,
    pub inches: u8,
}

impl Pizza {
    pub fn pepperoni(inches: u8) -> Self {
        Pizza::bake("pepperoni", inches)
    }

    pub fn mozzarella(inches: u8) -> Self {
        Pizza::bake("mozzarella", inches)
    }

    fn bake(topping: &str, inches: u8) -> Self {
        Pizza {
            topping: String::from(topping),
            inches,
        }
    }
}
