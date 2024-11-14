trait Printable {
    fn println(&self);
}

impl Printable for i32 {
    fn println(&self) {
        println!("{self}");
    }
}

impl Printable for String {
    fn println(&self) {
        println!("{self}");
    }
}

impl<T> Printable for Vec<T> where T : Printable {
    fn println(&self) {
        for e in self {
            e.println();
        }
    }
}
/*
 STATIC DISPATCH (monomorphization) means that the compiler fills in
 the concrete types for generic ones at compile time and duplicate
 code if necessary (no performance penalty at runtime)
 */
// fn print(arg : impl Printable) {
//     arg.println();
// }

/*
 DYNAMIC DISPATCH (DYN)
*/
fn print(arg : &dyn Printable) {
    arg.println();
}

fn es1_test() {
    let number = 42;
    let text = String::from("Hello, world!");
    let numbers = vec![1, 2, 3];
    let texts = vec![String::from("Rust"), String::from("is"), String::from("fun!")];

    // Testing static dispatch by directly calling `println` on each type.
    // number.println();
    // text.println();
    // numbers.println();
    // texts.println();

    // Testing dynamic dispatch by calling `print` on references to `Printable` types.
    print(&number);
    print(&text);
    print(&numbers);
    print(&texts);
}
fn es2_test() {
    // Create an empty library and populate it
    let mut library = Library::default();
    library.populate();

    // Print out the library to see the books and their categories
    println!("{:#?}", library);
}

use rand::{RngCore};

#[derive(Debug, Default)]
enum Category {
    #[default]
    Fantasy,
    Horror,
    SciFi,
    Romance,
    Thriller,
    Historical,
    Comedy,
    Drama,
    Poetry,
    Other
}

#[derive(Debug)]
struct Book {
    title : String,
    cat : Category
}

#[derive(Debug, Default)]
struct Library {
    bookcases : [Vec<Book>; 10]
}

impl Default for Book {
    fn default() -> Self {
        let mut title = String::new();
        for _ in 0..10 {
            title.push((rand::thread_rng().next_u32() % 26 +
                ('a' as u32)) as u8 as char);
        }
        Book {
            title,
            cat: Category::default(),
        }
    }
}

impl Book {
    fn default_with_cat(cat : Category) -> Self {
        Book {
            cat,
            ..Self::default()
        }
    }
}

trait Populatable {
    fn populate(&mut self);
}

impl Populatable for Library {
    fn populate(&mut self) {
        for bookcase in self.bookcases.iter_mut() {
            for _ in 0..3 {
                bookcase.push(Book::default());
            }
        }
    }
}

use std::fmt::{Debug,Display};

fn restricted<T,U> (t1 : T, t2 : T, u : U) -> impl PartialOrd + Display + Debug where T: Debug + PartialOrd + Display, U : Display {
    let res= if t1 < t2 { t1 } else { t2 };
    println!("minor: {res:?}");
    println!("u: {u}");
    res
}

fn es3_test() {
    // Test with integers
    let a = 10;
    let b = 20;
    let message = "Comparing two integers:";
    let result = restricted(&a, &b, &message);
    println!("Result: {:?}", result);

    // Test with floating-point numbers
    let x = 3.14;
    let y = 2.71;
    let note = "Comparing two floats:";
    let result = restricted(&x, &y, &note);
    println!("Result: {result}");

    // Test with strings
    let str1 = String::from("apple");
    let str2 = String::from("orange");
    let label = "Comparing two strings:";
    let result = restricted(&str1, &str2, &label);
    println!("Result: {result}");
}

#[derive(Clone)]
struct Task {
    name : String,
    priority : i32,
    done: bool
}

struct Tasks {
    tasks : Vec<Task>
}

impl Task {
    fn new(name : String, priority : i32, done : bool) -> Self {
        Self{name, priority, done}
    }
}

impl Tasks {
    fn new(tasks : &[Task]) -> Self {
        Self{tasks : tasks.to_vec()}
    }
}

impl Iterator for Tasks {
    type Item = Task;

    fn next(&mut self) -> Option<Self::Item> {
        self.tasks.iter().position(|t| !t.done).map(|i| self.tasks.remove(i))
    }
}

// ES 5
struct Pair(i32, String);

impl std::ops::Add<i32> for Pair {
    type Output = Pair;

    fn add(self, rhs: i32) -> Self::Output {
        Pair(self.0 + rhs, self.1)
    }
}

impl std::ops::Sub<i32> for Pair {
    type Output = Pair;

    fn sub(self, rhs: i32) -> Self::Output {
        Pair(self.0 - rhs, self.1)
    }
}

impl std::ops::Add<&str> for Pair {
    type Output = Pair;

    fn add(self, rhs: &str) -> Self::Output {
        Pair(self.0, self.1 + rhs)
    }
}

impl std::ops::Sub<&str> for Pair {
    type Output = Pair;

    fn sub(self, rhs: &str) -> Self::Output {
        Pair(self.0, self.1.replace(rhs, ""))
    }
}

impl std::ops::Add for Pair {
    type Output = Pair;

    fn add(self, rhs: Self) -> Self::Output {
        self + rhs.0 + rhs.1.as_str()
    }
}

impl std::ops::Sub for Pair {
    type Output = Pair;

    fn sub(self, rhs: Self) -> Self::Output {
        self - rhs.0 - rhs.1.as_str()
    }
}

impl std::ops::Mul<i32> for Pair {
    type Output = Pair;

    fn mul(self, rhs: i32) -> Self::Output {
        Pair(self.0.pow(rhs as u32), self.1.repeat(rhs as usize))
    }
}

struct Open;
struct Closed;
struct Stopped {
    reason : String
}
// ES 6
use rand::{self, Rng};
struct Gate<S> {
    state : S
}

impl Gate<Open> {
    fn new() -> Gate<Open> {
        Gate { state: Open }
    }

    fn close(self) -> Result<Gate<Closed>, Gate<Stopped>> {
        let r = rand::thread_rng().gen_range(0..20);
        match r {
            0..=12 => Ok(Gate{ state: Closed }),
            13..=15 => Err(Gate{
                state: Stopped{
                    reason: "Motor error".to_string(),
                },
            }),
            _ => Err(Gate{
                state: Stopped{
                    reason: "Photocell detected an object".to_string(),
                },
            }),
        }
    }
}

impl Gate<Closed> {
    fn new() -> Gate<Closed> {
        Gate { state: Closed }
    }

    fn close(self) -> Result<Gate<Open>, Gate<Stopped>> {
        let r = rand::thread_rng().gen_range(0..20);
        match r {
            0..=12 => Ok(Gate { state: Open }),
            13..=15 => Err(Gate{
                state: Stopped{
                    reason: "Motor error".to_string(),
                },
            }),
            _ => Err(Gate{
                state: Stopped{
                    reason: "Photocell detected an object".to_string(),
                },
            }),
        }
    }
}

impl Gate<Stopped> {
    fn new(reason : &str) -> Gate<Stopped> {
        Gate{ state: Stopped{reason: reason.to_string()} }
    }
    fn open(self) -> Gate<Open> {
        Gate { state : Open }
    }
    fn close(self) -> Gate<Closed> {
        Gate { state : Closed }
    }
}

pub fn sheet5() {
    es1_test();
    es2_test();
    es3_test();
    // es 4 no test
    // es 5 no test
}