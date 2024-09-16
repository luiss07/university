// The trait `Unknown` defines a method `serialize` that returns the implementer's `String` representation.
// - [1] implement it for `i32`
// - [1] implement it for `String`
// - [3] implement it for `Vec<T>`, where T implements Debug
// - [2] write a function `get_vec` that returns an empty vec of `Unknown` data
// - [3] write a function `print_vec` that takes as input a reference of a vec of `Unknown` data and prints its content
// ```
// trait Unknown {
//     fn serialize(&self) -> String;
// }
// ```

use std::fmt::Debug;

impl Unknown for i32 {
    fn serialize(&self) -> String {
        self.to_string()
    }
}

impl Unknown for String {
    fn serialize(&self) -> String {
        self.clone()
    }
}

impl<T> Unknown for Vec<T> where T: Debug {
    fn serialize(&self) -> String {
        format!("{:?}", self)
    }
}

fn get_vec() -> Vec<Box<dyn Unknown>> {
    Vec::new()
}

fn print_vec(v: &Vec<Box<dyn Unknown>>) {
    for u in v {
        println!("{}", u.serialize())
    }
}

// !share

trait Unknown {
    fn serialize(&self) -> String;
}

// ---

#[test]
fn test1() {
    let mut v = get_vec();
    v.push(Box::new("hiii!".to_string()));
    v.push(Box::new(-587));
    v.push(Box::new("xyz".to_string()));
    v.push(Box::new(vec![4, 5, 6]));
    print_vec(&v);
}
/*
hiii!
-587
xyz
[4, 5, 6]
 */

#[test]
fn test2() {
    let mut v = get_vec();
    v.push(Box::new("invalid data".to_string()));
    v.push(Box::new(58));
    v.push(Box::new(987));
    v.push(Box::new(vec![vec!["hi1", "hi2"], vec!["hi3"]]));
    print_vec(&v);
}
/*
invalid data
58
987
[["hi1", "hi2"], ["hi3"]]
 */
