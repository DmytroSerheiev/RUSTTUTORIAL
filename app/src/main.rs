// ! Option <T> если отсутствует значение или пустое

// ? Стандартная библиотека Rust предоставляет Option<T>перечисление, которое можно использовать, когда возможно отсутствие значения. Option<T>широко используется в коде Rust. Это полезно для работы со значениями, которые могут существовать или быть пустыми.

// fn main() {
//     let num = vec!["1", "2", "3", "4", "5"];

//     let first = num.get(0);
//     println!("{:?}", first);

//     let third = num.get(2);
//     println!("{:?}", third);

//     let non_existent = num.get(99);
//     println!("{:?}", non_existent);

// ? match В Rust есть мощный оператор match. Вы можете использовать его для управления ходом вашей программы, предоставляя шаблоны. При matchобнаружении соответствующего шаблона он запускает код, который вы предоставили с этим шаблоном.

// for &index in [0, 2, 99].iter() {
//     match num.get(index) {
//         Some(&"3") => println!("2 are awesome!!!"),
//         Some(num_name) => println!("It's a delicious {}!", num_name),
//         None => println!("There is no fruit! 😥"),
//     }
// }
// ? Выражение if let Rust предлагает удобный способ проверить, соответствует ли значение одному шаблону.

//     let a_number: Option<u8> = Some(7);
//     match a_number {
//         Some(7) => println!("That's my lucky number! "),
//         _ => {}
//     }
// }
//? Используйте unwrap и expect Вы можете попытаться получить доступ к внутреннему значению типа Optionнапрямую, используя unwrapметод. Однако будьте осторожны, потому что этот метод вызовет панику, если вариант — None.

//     //unwrap
// let gift = Some("candy");
// assert_eq!(gift.unwrap(), "candy");

// let empty_gift: Option<&str> = None;
// assert_eq!(empty_gift.unwrap(), "candy"); // This will panic!

//    //expect
// let a = Some("value");
// assert_eq!(a.expect("fruits are healthy"), "value");

// let b: Option<&str> = None;
// b.expect("fruits are healthy"); // panics with `fruits are healthy`
// // ? метод котоий не приводит к панике
// assert_eq!(Some("dog").unwrap_or("cat"), "dog");
// assert_eq!(None.unwrap_or("cat"), "cat");

//! Упражнение. Использование типа «Option» для устранения отсутствия
// struct Person {
//     first: String,
//     middle: Option<String>,
//     last: String,
// }

// fn build_full_name(person: &Person) -> String {
//     let mut full_name = String::new();
//     full_name.push_str(&person.first);
//     full_name.push_str(" ");

//     // TODO: Implement the part of this function that handles the person's middle name.

//     if let Some(middle_name) = &person.middle {
//         full_name.push_str(middle_name);
//         full_name.push_str(" ");
//     }

//     full_name.push_str(&person.last);
//     full_name
// }

// fn main() {
//     let john = Person {
//         first: String::from("James"),
//         middle: Some(String::from("Oliver")),
//         last: String::from("Smith"),
//     };
//     assert_eq!(build_full_name(&john), "James Oliver Smith");

//     let alice = Person {
//         first: String::from("Alice"),
//         middle: None,
//         last: String::from("Stevens"),
//     };
//     assert_eq!(build_full_name(&alice), "Alice Stevens");

//     let bob = Person {
//         first: String::from("Robert"),
//         middle: Some(String::from("Murdock")),
//         last: String::from("Jones"),
//     };
//     assert_eq!(build_full_name(&bob), "Robert Murdock Jones");
// }
//    ? Используйте тип результата для обработки ошибок (Result тип)
// ? В отличие от Optionтипа, описывающего возможность отсутствия значения , Resultтип лучше всего подходит там, где могут произойти сбои .
// enum Result<T, E> {
//     Ok(T):  // A value T was obtained.
//     Err(E): // An error of type E was encountered instead.
// }
// //=========================================

// #[derive(Debug)]
// struct DivisionByZeroError;

// fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
//     if divisor == 0.0 {
//         Err(DivisionByZeroError)
//     } else {
//         Ok(dividend / divisor)
//     }
// }

// fn main() {
//     println!("{:?}", safe_division(9.0, 3.0));
//     println!("{:?}", safe_division(4.0, 0.0));
//     println!("{:?}", safe_division(0.0, 2.0));
// }
//! Упражнение. Использование типа Result для обработки ошибок
//! use std::fs::File;
use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

fn read_file_contents(path: PathBuf) -> Result<String, Error> {
    // Создаем пустую строку, в которую будем считывать содержимое файла
    let mut string = String::new();

    // Доступ к файлу по указанному пути
    // ---------------------------------
    // TODO #1:
    // - Передаем переменную `file` в случае успеха, или
    // - Возвращаемся из функции рано, если есть ошибка
    // Получаем файловый дескриптор (handle) с помощью File::open и присваиваем его переменной file
    let mut file: File = match File::open(&path) {
        Ok(file_handle) => file_handle,
        Err(io_error) => return Err(io_error),
    };

    // Считываем содержимое файла в переменную `String` с помощью `read_to_string`
    // ---------------------------------
    // Путь успеха уже заполнен
    // TODO #2: Возвращаемся из функции рано, если есть ошибка
    // Считываем содержимое файла с использованием file.read_to_string и сохраняем его в переменной string
    match file.read_to_string(&mut string) {
        Ok(_) => (),
        Err(io_error) => return Err(io_error),
    };

    // TODO #3: Возвращаем переменную `string` в ожидаемом формате функции
    // Возвращаем содержимое файла как часть успешного результата, оборачиваем его в Ok
    Ok(string)
}

fn main() {
    // Проверка, успешно ли считано содержимое файла "src/main.rs"
    if let Ok(_) = read_file_contents(PathBuf::from("src/main.rs")) {
        println!("Программа нашла основной файл.");
    }

    // Проверка, возникла ли ошибка при считывании содержимого файла "non-existent-file.txt"
    if let Err(_) = read_file_contents(PathBuf::from("non-existent-file.txt")) {
        println!("Программа сообщила об ошибке для файла, который не существует.");
    }
}
