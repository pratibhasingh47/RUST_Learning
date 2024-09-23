use std::ops::Add;
// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// pub fn add_two(a: usize) -> usize {
//     a + 2
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn larger_can_hold_smaller() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };

//         assert!(larger.can_hold(&smaller));
//     }

//     #[test]
//     fn smaller_cannot_hold_larger() {
//         let larger = Rectangle {
//             width: 8,
//             height: 7,
//         };
//         let smaller = Rectangle {
//             width: 5,
//             height: 1,
//         };

//         assert!(!smaller.can_hold(&larger));
//     }
// }
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }

//     #[test]
//     fn another() {
//         panic!("Make this test fail");
//     }

// }



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_adds_two() {
//         let result = add_two(2);
//         assert_eq!(result, 4);
//     }
// }

// pub fn greeting(name: &str) -> String {
//     format!("Hello {name}!")
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn greeting_contains_name() {
//         let result = greeting("Carol");
//         assert!(result.contains("Carol"));
//     }
// }

pub struct Guess {
    value: i32,
}

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 || value > 100 {
//             panic!("Guess value must be between 1 and 100, got {value}.");
//         }

//         Guess { value }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[should_panic]
//     fn greater_than_100() {
//         Guess::new(200);
//     }
// }

// --snip--

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 {
//             panic!(
//                 "Guess value must be greater than or equal to 1, got {value}."
//             );
//         } else if value > 100 {
//             panic!(
//                 "Guess value must be less than or equal to 100, got {value}."
//             );
//         }

//         Guess { value }
//     }
// }

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // #[should_panic(expected = "less than or equal to 100")]
    // fn greater_than_100() {
    //     Guess::new(200);
    // }
    use std::ops::Add;
    #[test]
    fn it_works() -> Result<(), String> {
        // let result = add(2, 2);
        let result = 2.add(2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

}