use std::collections::HashMap;

fn main() {

    let c = 's';
    println!("{}", prev_char(c));

    let s = "abcdefg_1234_ABCDEFG";
    let s = prev_str(s);
    println!("{}", s);

    let s1 = Student::new("marco", 1);
    let s2 = Student::new("anto", 2);
    let s3 = Student::new("anna", 3);
    let mut university = University::new("Trento", &vec![s1, s2, s3]);

    println!("{}", university);

    println!("{}", university.remove_student(1).unwrap().id);

    let mut fleet = AirFleet{
        fleet: Vec::new(),
    };

    let airplane1 = Airplane{
        company: AirplaneCompany::Airbus,
        model: "A380".to_string(),
    };

    let airplane2 = Airplane{
        company: AirplaneCompany::Boeing,
        model: "747".to_string(),
    };

    let airplane3 = Airplane{
        company: AirplaneCompany::Airbus,
        model: "A320".to_string(),
    };

    fleet.add_airplane(airplane1);
    fleet.add_airplane(airplane2);
    fleet.add_airplane(airplane3);

    println!("{:?}", fleet.search_airplane("A380"));
    println!("{:?}", fleet.search_airplane("747"));
    println!("{:?}", fleet.search_airplane("A320"));
    println!("{:?}", fleet.search_airplane("A330"));

    let mut hashmap = HashMap::new();
    hashmap.insert(1, "ciao".to_string());
    hashmap.insert(2, "ciao".to_string());
    hashmap.insert(3, "ciao".to_string());

    let hashmap = Maps{
        map: hashmap,
    };

    let hashmap = string_to_tuple(hashmap);

    println!("{:?}", (hashmap.get(&1).unwrap().0, hashmap.get(&1).unwrap().1.clone()));

}

use std::char::from_u32;
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
//use crate::AirplaneCompany::{Airbus, Boeing};
//use crate::AirplaneCompany::Airbus;
use crate::another_mod::string_to_tuple;
use crate::hashmap::Maps;

fn prev_char(c : char) -> char {
    from_u32((c as u32) - 1).unwrap()
}

fn prev_str(s : &str) -> String {
    let mut new_str = String::new();

    for e in s.chars() {
        if e.is_alphabetic() && ( e != 'a' && e != 'A')  {
            new_str.push(prev_char(e));
        } else {
            new_str.push(e);
        }
    }
    new_str
}

struct X {
    s : Option<String>,
    i : i32
}

impl X {
    fn new(s : &str, i : i32) -> X {
        X{s:Some(s.to_string()), i}
    }

    fn take_str(&mut self) -> Option<String> {
        let ret = self.s.clone();
        self.s = None;
        ret
    }
}

struct NameSurname {
    name: String,
    surname : String
}

fn replace_surname(mut ns_str : NameSurname, surname : String) -> String {
    let old_surname = ns_str.surname;
    ns_str.surname = surname;
    old_surname
}

#[derive(Debug)]
#[derive(Clone)]
struct Student {
    name : String,
    id : u32
}

struct University {
    name : String,
    students : Vec<Student>
}

impl Student {
    fn new(name : &str, id : u32) -> Student {
        Student{name:name.to_string(), id}
    }
    
    
}

impl University {
    fn new(name:&str, students : &[Student]) -> University {
        University{name:name.to_string(), students:Vec::from(students)}
    }

    fn remove_student(&mut self, id: u32) -> Result<Student, &'static str> {
        for i in 0..self.students.len() {
            if self.students[i].id == id {
                return Ok(self.students.remove(i));
            }
        }
        Err("Not found")
    }
}

impl Display for Student {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for University {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\nStudents: {:?}", self.name, self.students)
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
enum AirplaneCompany {
    Airbus,
    Boeing
}

struct Airplane {
    company : AirplaneCompany,
    model : String
}

struct AirFleet {
    fleet: Vec<Airplane>
}

impl AirFleet {
    fn remove_boeing(&mut self) {
        for i in 0..self.fleet.len() {
            if self.fleet[i].company == AirplaneCompany::Boeing {
                self.fleet.remove(i);
            }
        }
    }

    fn add_airplane(&mut self, airplane: Airplane) {
        self.fleet.push(airplane)
    }

    fn search_airplane(&self, model : &str) -> Result<AirplaneCompany, String> {
        // for e in self.fleet.iter() {
        //     if e.model == model.to_string() {
        //         return match e.company {
        //             AirplaneCompany::Airbus => Ok(Airbus),
        //             AirplaneCompany::Boeing => Ok(Boeing)
        //         }
        //     }
        // }
        // Err("Not in this fleet".to_string())

        let airplane = self.fleet.iter().find(|&ap| ap.model == model.to_string());
        match airplane {
            Some(airplane) => match airplane.company {
                AirplaneCompany::Airbus => Ok(AirplaneCompany::Airbus),
                AirplaneCompany:: Boeing => Ok(AirplaneCompany::Boeing)
            },
            None => Err("Not in this fleet".to_string())
        }
    }
}



mod unumber {
    pub type Unumber = usize;
}

mod hashmap {
    use super::unumber::Unumber;
    use std::collections::HashMap;

    pub struct Maps {
        pub map : HashMap<Unumber, String>
    }
}

mod another_mod {
    use super::hashmap;
    use super::unumber::Unumber;    use std::collections::HashMap;

    pub fn string_to_tuple(maps : hashmap::Maps) -> HashMap<Unumber, (Unumber, String)> {
        let mut new_map : HashMap<Unumber, (Unumber, String)> = HashMap::new();
        for (key, value) in maps.map.iter() {
            new_map.insert(key.clone(), (value.len(), value.clone()));
        }
        new_map
    }
}

// es 14

struct Size {
    width: f32,
    height: f32
}

impl Size {
    fn new(width: f32, height: f32) -> Size {
        Size{width: width, height: height}
    }

    fn area(&self) -> f32 {
        self.height*self.width
    }

    fn compare(&self, s1 : &Size) -> Option<bool> {
        let area1 = self.area();
        let area2 = s1.area();
        if area1 == area2 {
            None
        } else if area1 > area2 {
            Some(true)
        } else {
            Some(false)
        }
    }
}



struct MaybePoint {
    x : Option<i32>,
    y : Option<i32>
}

impl MaybePoint {
    fn new(x : Option<i32>, y : Option<i32>) -> MaybePoint {
        MaybePoint{x, y}
    }

    fn is_some(&self) -> bool {
        match self.x {
            Some(_) => match self.y {
                Some(_) => true,
                None => false
            },
            None => false
        }
    }

    fn maybe_len(&self) -> Option<f32> {
        match self.x {
            Some(x) => match self.y {
                Some(y) => {
                    let len = ((x.pow(2) + y.pow(2))as f32).sqrt();
                    Some(len)
                },
                None => None
            },
            None => None
        }
    }
}

fn res1(i : i32) -> Result<i32, String> {
    if i % 10 == 0 {
        return Ok(10);
    }
    Err("err".to_string())
}

fn res2(res : Result<i32, &str>) -> Result<i32, String> {
    match res {
        Ok(i) => {
            if i % 5 == 0 {
            return Ok(10);
        }
            Err("error".to_string())
        },
        Err(s) => Err(s.to_string())
    }
}

fn wrapper(i : i32) -> Result<i32, String> {
    let res1_result = res1(i)?;  // Use ? to propagate errors from res1
    res2(Ok(res1_result))?;  // Use ? to propagate errors from res2
    Ok(res1_result)
}

fn order(vs : Vec<String>) -> Vec<String> {
    let mut new_vs = Vec::new();
    for i in 0..vs.len() {
        let str = i.to_string() + &" - " + &vs[i];
        new_vs.push(str);
    }
    new_vs
}

mod modx {
    pub enum X {
        S(char),
        C(String)
    }
}

mod mody {
    pub enum X {
        F(f64, usize)
    }
}

mod modsum {
    use super::modx;
    use super::mody;

    pub fn sum(xx : modx::X, xy : mody::X) -> f64 {
        match xy {
            mody::X::F(f,u) => {
                let product = f*u as f64;
                match xx {
                    modx::X::S(c) => {
                        (c as u64) as f64 + product
                    },
                    modx::X::C(s) => s.len() as f64 + product
                }
            }
        }

    }
}