// // ! Правила определения области действия
// {
//     // ? право собственности на переменную
//     let mascot = Strict::from("ferris");
//     let ferris = mascot;
//     // мы передели право собственности  mascot  переменной ferris и mascot Больше не можем ипользоват
// // В Rust только одна вещь может одновременно владеть частью данных.
// }
// // ?Право собственности на функции Передача чего-либо в качестве аргумента функции перемещает это в функцию.
// fn process(input: String) {}

// fn caller() {
//     let s = String::from("Hello, world!");
//     process(s); // Ownership of the string in `s` moved into `process`
//     process(s); // Error! ownership already moved.
// }
// // ? Копирование вместо перемещения. Простые типи капируются, а не перемещаются
// fn process(input: u32) {}

// fn caller() {
//     let n = 1u32;
//     process(n); // Ownership of the number in `n` copied into `process`
//     process(n); // `n` can be used again because it wasn't moved, it was copied.
// }
// // ? Клонирование типов перед их перемещением, это трудозатратно, поэтому лучше использовать ссылки
// fn process(s: String) {}

// fn main() {
//     let s = String::from("Hello, world!");
//     process(s.clone()); // Passing another value, cloned from `s`.
//     process(s); // s was never moved and so it can still be used.
// }
// // ? силочний метод Ссылки позволяют нам «заимствовать» значения, не становя на них права собственности с помощью ссылочного символа ( &)
// let greeting = String::from("hello");
// let greeting_reference = &greeting; // We borrow `greeting` but the string data is still owned by `greeting`
// println!("Greeting: {}", greeting); // We can still use `greeting`
// // ? Ссылки в функциях
// fn print_greeting(message: &String) {
//   println!("Greeting: {}", message);
// }

// fn main() {
//   let greeting = String::from("Hello");
//   print_greeting(&greeting); // `print_greeting` takes a `&String` not an owned `String` so we borrow `greeting` with `&`
//   print_greeting(&greeting); // Since `greeting` didn't move into `print_greeting` we can use it again
// }
// //? нельзя менять заимствованные значения Этот код не скомпилируется

// fn change(message: &String) {
//   message.push_str("!"); // We try to add a "!" to the end of our message
// }

// fn main() {
//   let greeting = String::from("Hello");
//   change(&greeting);
// }
// //? вот как правельно изменять функцию по ссилке
// fn main() {
//     let mut greeting = String::from("hello");
//     change(&mut greeting);
// }

// fn change(text: &mut String) {
//     text.push_str(", world");
// }
// //? Заимствование и изменяемые ссылки(ошибка)
// fn main() {
//     let mut value = String::from("hello");

//     let ref1 = &mut value;
//     let ref2 = &mut value;

//     println!("{}, {}", ref1, ref2);
// }
// //? Заимствование и изменяемые ссылки(ошибка)
// fn main() {
//     let mut value = String::from("hello");

//     let ref1 = &value;
//     let ref2 = &mut value;

//     println!("{}, {}", ref1, ref2);
// }
// //? Проверка ссылок с использованием времени жизни
// fn main() {
//     let x;
//     {
//         let y = 42;
//         x = &y; // We store a reference to `y` in `x` but `y` is about to be dropped.

//     }
// // ошибка компиляции
//     println!("x: {}", x); // `x` refers to `y` but `y has been dropped!
// }
// // вроде рабочий вариант
// fn main() {
//     let magic1 = String::from("abracadabra!");
//     let magic2 = String::from("shazam!");

//     let result = longest_word(&magic1, &magic2);
//     println!("The longest magic word is {}", result);
// }
// fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// //? Аннотирование времени жизни в типах
// //? Вывод в консоли
// //Highlight("quick brown fox")
// //Highlight("lazy dog")

// #[derive(Debug)]
// struct Highlight<'document>(&'document str);

// fn main() {
//     let text = String::from("The quick brown fox jumps over the lazy dog.");
//     let fox = Highlight(&text[4..19]);
//     let dog = Highlight(&text[35..43]);
//     println!("{:?}", fox);
//     println!("{:?}", dog);
// }
// ! Упражнение – время жизни
fn copy_and_return<'a>(vector: &'a mut Vec<String>, value: &'a str) -> &'a String {
    vector.push(String::from(value));
    vector.get(vector.len() - 1).unwrap()
}

fn main() {
    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";

    let mut names = Vec::new();

    assert_eq!("Joe", copy_and_return(&mut names, &name1));
    assert_eq!("Chris", copy_and_return(&mut names, &name2));
    assert_eq!("Anne", copy_and_return(&mut names, &name3));

    assert_eq!(
        names,
        vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
    )
}
