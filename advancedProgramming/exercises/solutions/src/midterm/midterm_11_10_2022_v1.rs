use std::ascii::AsciiExt;

// es 9
pub enum XX {
    Y1(i32, i32),
    Y2(String)
}

pub fn data(xx : &XX) -> i32 {
    match xx {
        XX::Y1(n1, n2) => n1*n2,
        XX::Y2(s) => s.len() as i32
    }
}

//es 10
pub enum Z {
    Y1(i32, i32),
    Y2(bool, String)
}

pub fn maybelength(z : &Z) -> Option<i32> {
    match z {
        Z::Y2(_, s) => Some(s.len() as i32),
        _ => None
    }
}

// es 11

pub mod enumx {
    pub enum X {
        Y(String)
    }
}

pub mod structx {
    pub struct X {
        pub i: String
    }
}

pub mod modfun {
    use crate::midterm::midterm_11_10_2022_v1::enumx;
    use crate::midterm::midterm_11_10_2022_v1::structx;

    pub fn longer(e : &enumx::X, s : &structx::X) -> usize {
        match e {
            enumx::X::Y(es) => if es.len() > s.i.len() { es.len() } else { s.i.len() }
        }
    }
}

// es 12

pub fn maybesqrt(n : i32) -> Option<i32> {
    if n < 0 {
        return None;
    }
    Some((n as f32).sqrt() as i32)
}

 // es 13
pub struct Balance {
     pub amt: i32
 }

impl Balance {
    pub fn maybewithdraw(&self, n : i32) -> Option<i32> {
        if self.amt < n {
            return None;
        }
        Some(self.amt-n)
    }
}

// es 14

use std::char::from_u32;
use std::fmt::{Display, Formatter};

pub fn prevchar(c : char ) -> i32 {
    ((c as u32) - 1) as i32
}

pub fn replwithprev(s : &mut String) -> Result<String, ()> {
    let mut new_s = String::new();

    if s.contains('a') {
        return Err(());
    }

    for c in s.chars(){
        if let Some(prevc) = from_u32(prevchar(c) as u32) {
            new_s.push(prevc)
        } else {
            return Err(());
        }
        // match from_u32(prevchar(c) as u32) {
        //     Some(prevc) => new_s.push(prevc),
        //     None => ()
        // }
    }

    Ok(new_s)
}

// es 15
use std::mem::replace;
use crate::midterm::midterm_11_10_2022_v1::C::{C1, C2};

#[derive(Debug)]
pub struct X {
    pub s : String,
    pub i : i32
}

#[derive(Debug)]
pub struct Y {
    pub z : bool,
    pub c : String
}

impl X {
    pub fn new() -> X {
        X{s:"xxx".to_string(), i:10}
    }

    pub fn getstring(&mut self) -> String {
        let s = self.s.clone();
        self.s = "".to_string();
        s
    }
}

impl Y {
    pub fn new() -> Y {
        Y{z:true, c:"op".to_string()}
    }

    pub fn getstring(&mut self) -> String {
        let s = self.c.clone();
        self.c = "".to_string();
        s
    }
}

impl Display for X {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "S {}, I {}", self.s, self.i)
    }
}

impl Display for Y {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "B {}, C {}", self.z, self.c)
    }
}

pub fn swapstr(mut x : X, mut y : Y)  -> (X,Y){
    let c = replace(&mut y.c, x.s.clone());
    x.s = c;
    (x,y)
}

// es 16
#[derive(Debug)]
pub enum C {
    C1(i32, i32),
    C2(bool, String)
}

#[derive(Debug)]
pub struct D {
    c : C,
    s : String
}

impl D {
    pub fn new() -> D {
        D{c:C1(0,0), s:"".to_string()}
    }

    pub fn new_with_c(c : C) -> D {
        match c {
            C1(_,_) => D{c, s:"not there".to_string()},
            C2(b,s) => D{c:C2(b,s.clone()), s:s.clone() }
        }
    }

    pub fn larger(&self) -> usize {
        match &self.c {
            C1(_,_) => self.s.len(),
            C2(_,s) => if self.s.len() > s.len() { self.s.len() } else { s.len() }
        }
    }
}

impl Display for D {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.c {
            C1(_,_) => write!(f, "{} with {:?}", self.s, self.c),
            C2(_,_) => write!(f, "{} with {:?}", self.s, self.c)
        }
    }
}

// es 17

pub fn swapelconcat(vs : &mut Vec<String>, i : usize, j : usize) -> Option<&mut Vec<String>> {
    if vs.len() < i || vs.len() < j {
        return None;
    }
    let i_s = vs[i].clone();
    let j_s = vs[j].clone();

    vs[i] += &j_s;
    vs[j] += &i_s;
    Some(vs)
}

// es 18
pub fn veclengths(vs : &Vec<String>) -> Vec<i32> {
    let mut vi = Vec::new();

    for s in vs.iter() {
        vi.push(s.len() as i32);
    }

    vi
}

// es 19
pub fn removeelfromvector(vs : &mut Vec<String>, n : usize) {
    for i in 0..vs.len() {
        if vs[i].len() == n {
            vs.remove(i);
            return
        }
    }
}