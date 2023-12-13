// //! загальні типи даних
// //? не можно змішувати👇

// struct POint<T> {
//     x: T,
//     y: t,
// }
// fn main() {
//     let boolean = Point { x: true, y: false };
//     let integer = Point { x: 1, y: 5 };
//     let float = Point { x: 1.5, y: 56.6 };
//     let string_slise = Point {
//         x: "high",
//         y: "low",
//     };
// }
//? можно змішувати👇
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// fn main() {
//     let integer_and_boolean = Point { x: 5, y: false };
//     let float_and_string = Point { x: 1.0, y: "hey" };
//     let integer_and_float = Point { x: 5, y: 4.0 };
//     let both_integer = Point { x: 10, y: 30 };
//     let both_boolean = Point { x: true, y: true };
// }
// ? Реалізація Трейду
// trait Area {
//     fn area(&self) -> f64;
// }

// struct Circle {
//     radius: f64,
// }

// struct Rectangle {
//     width: f64,
//     height: f64,
// }

// impl Area for Circle {
//     fn area(&self) -> f64 {
//         use std::f64::consts::PI;
//         PI * self.radius.powf(2.0)
//     }
// }

// impl Area for Rectangle {
//     fn area(&self) -> f64 {
//         self.width * self.height
//     }
// }
// fn main() {
//     let circle = Circle { radius: 5.0 };
//     let rectangle = Rectangle {
//         width: 10.0,
//         height: 20.0,
//     };

//     println!("Circle area: {}", circle.area());
//     println!("Rectangle area: {}", rectangle.area());
// }
// ? Використовуйте derive
// #[derive(Debug, PartialEq)]
// struct Point {
//     x: i32,
//     y: i32,
// }
// use std::fmt;

// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }
// fn main() {
//     let point = Point { x: 1, y: 2 };
//     println!("Point coordinates: {}", point);
// }
//? Trait AsJson:
// trait AsJson {
//     fn as_json(&self) -> String;
// }
// fn send_data_as_json(value: &impl AsJson) {
//     println!("Sending JSON data to server...");
//     println!("-> {}", value.as_json());
//     println!("Done!\n");
// }
//? ще один способ з явною ознакою Т типу
// trait AsJson {
//     fn as_json(&self) -> String;
// }
// fn send_data_as_json<T: AsJson>(value: &T) {
//     println!("Sending JSON data to server...");
//     println!("-> {}", value.as_json());
//     println!("Done!\n");
// }

// struct Person {
//     name: String,
//     age: u8,
//     favorite_fruit: String,
// }

// struct Dog {
//     name: String,
//     color: String,
//     likes_petting: bool,
// }

// impl AsJson for Person {
//     fn as_json(&self) -> String {
// 	    format!(
// 	        r#"{{ "type": "person", "name": "{}", "age": {}, "favoriteFruit": "{}" }}"#,
// 	        self.name, self.age, self.favorite_fruit
// 	    )
//     }
// }

// impl AsJson for Dog {
//     fn as_json(&self) -> String {
// 	    format!(
// 	        r#"{{ "type": "dog", "name": "{}", "color": "{}", "likesPetting": {} }}"#,
// 	        self.name, self.color, self.likes_petting
// 	    )
//     }
// }
// fn main() {
//     let laura = Person {
//     	name: String::from("Laura"),
// 	    age: 31,
// 	    favorite_fruit: String::from("apples"),
//     };

//     let fido = Dog {
// 	    name: String::from("Fido"),
// 	    color: String::from("Black"),
// 	    likes_petting: true,
//     };

//     send_data_as_json(&laura);
//     send_data_as_json(&fido);
// }
//? Використовуйте ітератори
// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }
// #[derive(Debug)]
// struct Counter {
//     length: usize,
//     count: usize,
// }

// impl Counter {
//     fn new(length: usize) -> Counter {
//         Counter { count: 0, length }
//     }
// }
// impl Iterator for Counter {
//     type Item = usize;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.count += 1;
//         if self.count <= self.length {
//             Some(self.count)
//         } else {
//             None
//         }
//     }
// }
// fn main() {
//     let mut counter = Counter::new(6);
//     println!("Counter just created: {:#?}", counter);

//     assert_eq!(counter.next(), Some(1));
//     assert_eq!(counter.next(), Some(2));
//     assert_eq!(counter.next(), Some(3));
//     assert_eq!(counter.next(), Some(4));
//     assert_eq!(counter.next(), Some(5));
//     assert_eq!(counter.next(), Some(6));
//     assert_eq!(counter.next(), None);
//     assert_eq!(counter.next(), None); // further calls to `next` will return `None`
//     assert_eq!(counter.next(), None);

//     println!("Counter exhausted: {:#?}", counter);
// }
//! вправа
// struct Container<T> {
//     value: T,
// }

// impl<T> Container<T> {
//     pub fn new(value: T) -> Self {
//         Container { value }
//     }
// }

// fn main() {
//     assert_eq!(Container::new(42).value, 42);
//     assert_eq!(Container::new(3.14).value, 3.14);
//     assert_eq!(Container::new("Foo").value, "Foo");
//     assert_eq!(
//         Container::new(String::from("Bar")).value,
//         String::from("Bar")
//     );
//     assert_eq!(Container::new(true).value, true);
//     assert_eq!(Container::new(-12).value, -12);
//     assert_eq!(Container::new(Some("text")).value, Some("text"));
// }
//! вправа
struct Groups<T> {
    inner: Vec<T>,
}

impl<T> Groups<T> {
    fn new(inner: Vec<T>) -> Self {
	    Groups { inner }
    }
}

impl<T: PartialEq> Iterator for Groups<T> {
    type Item = Vec<T>;

    // TODO: Write the rest of this implementation.
    fn next(&mut self) -> Option<Self::Item> {
        // if the inner vector is empty, we are done
        if self.inner.is_empty() {
            return None;
        }

        // lets check the span of equal items
        let mut cursor = 1;
        let first = &self.inner[0];
        for element in &self.inner[1..] {
            if element == first {
                cursor += 1;
            } else {
                break;
            }
        }

// we use the `Vec::drain` to extract items up until the cursor
        let items = self.inner.drain(0..cursor).collect();

        // return the extracted items
        Some(items)
    }
}

fn main() {
    let data = vec![4, 1, 1, 2, 1, 3, 3, -2, -2, -2, 5, 5];
    // groups:     |->|---->|->|->|--->|----------->|--->|
    assert_eq!(
	    Groups::new(data).into_iter().collect::<Vec<Vec<_>>>(),
	    vec![
	        vec![4],
    	    vec![1, 1],
	        vec![2],
    	    vec![1],
	        vec![3, 3],
	        vec![-2, -2, -2],
    	    vec![5, 5],
	    ]
    );

    let data2 = vec![1, 2, 2, 1, 1, 2, 2, 3, 4, 4, 3];
    // groups:      |->|---->|---->|----|->|----->|->|
    assert_eq!(
	    Groups::new(data2).into_iter().collect::<Vec<Vec<_>>>(),
	    vec![
	        vec![1],
    	    vec![2, 2],
	        vec![1, 1],
	        vec![2, 2],
    	    vec![3],
	        vec![4, 4],
	        vec![3],
	    ]
    )
}

