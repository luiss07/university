use std::cmp::PartialEq;
use crate::midterm::midterm_10_10_2023::A::{A1, A2};
use crate::midterm::midterm_10_10_2023::B::{B1, B2};
use crate::midterm::midterm_10_10_2023::F::F1;

pub enum A {
    A2(char, char),
    A1(i32, i32, i32)
}

#[derive(Debug)]
pub enum B {
    B1(i32, i32),
    B2(String)
}

pub fn bforma(a : A) -> B {
    match a {
        A2(c1, c2) => {
            B1(c1 as i32, c2 as i32)
        },
        A1(i1, i2, i3) => {
            //let s = String::from(i1.to_string() + "-" + &i2.to_string() + "-" + &i3.to_string());
            //B2(s)
            // option 2
            B2(format!("{}-{}-{}", i1, i2, i3))
        }
    }
}

#[derive(Debug)]
pub enum E {
    A(String),
    B(bool)
}

#[derive(Debug)]
pub enum F {
    F1(String),
    F2(i32)
}

impl E {
    pub fn count_vowels(&self) -> i32 {
        match &self {
            E::A(s) => {
                let mut count = 0;
                for c in s.chars() {
                    // if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'  {
                    //     count += 1;
                    // }
                    if "aeioiAEIOU".contains(c) {
                        count += 1;
                    }
                }
                count
            },
            _ => 0
        }
    }
}

impl F {
    pub fn calculation(&self) -> usize {
        match &self {
            F1(s) => s.len(),
            F::F2(i) => *i as usize
        }
    }

    pub fn new() -> F {
        F1(String::from("hello"))
    }
}

pub fn print_n(opt : Option<usize>) {
    match opt {
        Some(num) => {
            for _ in 0..num {
                println!("{num}")
            }
        }
        None => println!("Error")
    }
}

pub struct Balance {
    pub amt : i32,
    pub active : bool
}

impl Balance {
    pub fn maybericher(&self, bal : Balance) -> Option<bool> {
        if self.active && bal.active {
            if self.amt > bal.amt {
                Some(true)
            } else {
                Some(false)
            }
        } else {
            None
        }
    }
}

pub struct G {
    pub x : i32,
    pub y : i32
}

impl G {
    pub fn new(x : i32, y : i32) -> G {
        G{x,y}
    }

    pub fn square(&self) -> Result<i32, ()> {
        if self.x == (self.y^2) {
            Ok(self.x)
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
pub struct X {
    pub s : String,
    pub i : i32
}

use::std::mem::replace;
use std::fmt;
use std::fmt::{write, Formatter};

#[derive(Debug)]
pub struct Y {
    pub b : bool,
    pub c : String
}

impl fmt::Display for X {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "S {}, I {}", self.s, self.i)
    }
}

impl fmt::Display for Y {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "B {}, C {}", self.b, self.c)
    }
}

impl X {
    pub fn new() -> X {
        X { s: "xxx".to_string(), i: 10 }
    }

    pub fn getstr(&mut self) -> String {
        replace(&mut self.s, "".to_string())
    }
}

impl Y {
    pub fn new() -> Y {
        Y { b : true, c : "op".to_string() }
    }

    pub fn getstr(&mut self) -> String {
        replace(&mut self.c, "".to_string())
    }
}

pub fn swapstr(mut x: X, mut y : Y) -> (X, Y) {
    let tmp = x.s;
    x.s = y.c;
    y.c = tmp;
    (x,y)
}