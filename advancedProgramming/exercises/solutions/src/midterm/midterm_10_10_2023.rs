use std::cmp::PartialEq;
// use crate::midterm::midterm_10_10_2023::A::{A1, A2};
// use crate::midterm::midterm_10_10_2023::B::{B1, B2};
// use crate::midterm::midterm_10_10_2023::F::F1;

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
        A2(c1, c2) => B1(c1 as i32, c2 as i32),
        A1(i1, i2, i3) => {
            // let s = String::from(i1.to_string() + "-" + &i2.to_string() + "-" + &i3.to_string());
            B2(String::from(i1.to_string() + "-" + &i2.to_string() + "-" + &i3.to_string()))
            // option 2
            // B2(format!("{}-{}-{}", i1, i2, i3))
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

// es 15
#[derive(Debug)]
pub struct L {
    pub s : String,
    pub n : i32
}

#[derive(Debug)]
pub struct M {
    pub s : String,
    pub n : f64
}

impl L {
    pub fn new() -> L {
        L{s:"hello".to_string(), n:0}
    }

    pub fn new_with_params(s : String, n : i32) -> L {
        L{s, n}
    }
}

// es 15
impl M {
    pub fn new() -> M {
        M{s:"hello".to_string(), n:0.0}
    }

    pub fn new_with_params(s : String, n : f64) -> M {
        M{s, n}
    }
}

pub fn swap_string(l: &mut L, m : &mut M){
    let s = replace(&mut l.s, m.s.clone());
    m.s = s;
}

// es 16
pub fn neighbour(vs : &Vec<String>, i : usize) -> Result<String, ()> {
    if i == vs.len()-1 {
        return Err(())
    };

    Ok(vs[i].clone() + &vs[i+1].clone())
}


// es 17
pub fn removeelement(vo : &mut Vec<Option<i32>>) {
    for i in 0..vo.len()-1 {
        match vo[i] {
            Some(elem) => {
                if ( elem % 2 ) != 0 {
                    vo.remove(i);
                    return
                }
            },
            None => {
                vo.remove(i);
                return
            }
        }
    }
}
use std::collections::HashMap;
use std::hash::Hash;
use crate::midterm::midterm_10_10_2023::A::{A1, A2};
use crate::midterm::midterm_10_10_2023::B::{B1, B2};
use crate::midterm::midterm_10_10_2023::F::F1;

// es 18
pub fn hashandhash(h1 : &mut HashMap<i32, String>, h2 : &mut HashMap<String, i32>) {
    let mut vs = Vec::new();
    for (s , i) in h2.iter_mut() {
        if h1.contains_key(&(s.len() as i32)) {
            vs.push(s.clone());
        }
    }

    for s in vs {
        h2.remove(&s);
    }
}

// es 19
pub fn unique(mut h1 : HashMap<i32, String>, l : i32) -> Option<HashMap<i32, String>> {
    if h1.values().any(|v| v.len() == l as usize) {
        return None;
    } else {
        h1.insert(h1.len() as i32, "X".repeat(l as usize));
    }
    Some(h1)
}

pub fn midterm_10_10_2023() {

    // 1 no x is moved
    // 2 error
    // 3 20 20
    // 4 return 8
    // 5 yes
    // 6 error passing array as non mutable
    // 7 No, v is moved at line 2, and can't be used at line 3

    // es 9
    let a1 = A1(1, 2, 3);
    let a2 = A2('a', 'b');
    println!("B2: {:?}, B1:{:?}", bforma(a1), bforma(a2));
    let a1 = A1(1, 6, 30);
    let a2 = A2('t', 'z');
    println!("B2: {:?}, B1:{:?}", bforma(a1), bforma(a2));

    // es 10
    let e1 = E::A("hello".to_string());
    let e2 = E::B(true);
    println!("{:?} {:?}", e1, e1.count_vowels());
    println!("{:?} {:?}", e2, e2.count_vowels());

    let f1 = F::new();
    let f2 = F::F2(10);
    let f3 = F::F2(20);
    println!("{:?} {:?}", f1, f1.calculation());
    println!("{:?} {:?}", f2, f2.calculation());
    println!("{:?} {:?}", f3, f3.calculation());

    // es 11
    print_n(Some(3));
    print_n(None);

    // es 12
    let b = Balance{amt:100,active:true};
    let b2 = Balance{amt:200,active:true};
    println!("maybericher {:?}", b.maybericher(b2));

    let b = Balance{amt:100,active:true};
    let b2 = Balance{amt:0,active:true};
    println!("maybericher {:?}", b.maybericher(b2));

    let b = Balance{amt:100,active:false};
    let b2 = Balance{amt:200,active:true};
    println!("maybericher {:?}", b.maybericher(b2));

    let b = Balance{amt:100,active:true};
    let b2 = Balance{amt:200,active:false};
    println!("maybericher {:?}", b.maybericher(b2));

    // es 13
    let g = G::new(4, 2);
    let result = g.square();
    println!("{:?}", result);

    // es 14
    let mut x = X::new();
    let mut y = Y::new();
    println!("X {:?} - Y {:?}", x, y);
    let (mut x, mut y) = swapstr(x, y);
    println!("X {} - Y {}", x, y);
    let z1 = x.getstr();
    let z2 = y.getstr();
    println!("{},{},{},{}",z1,z2,x.s,y.c);

    // es 15
    let l = L::new();
    let m = M::new();
    println!("{:?} {:?}",l,m);

    let l = L::new_with_params("world".to_string(), 10);
    let m = M::new_with_params("world".to_string(), 10.0);
    println!("{:?} {:?}",l,m);

    let mut l = L::new_with_params("world".to_string(), 10);
    let mut m = M::new_with_params("hello".to_string(), 10.0);
    swap_string(&mut l, &mut m);
    println!("{:?} {:?}",l,m);

    // es 16
    let v = vec!["hello".to_string(), "world".to_string(), "how".to_string(), "are".to_string(), "you".to_string()];
    let result = neighbour(&v, 0);
    println!("{:?}",result);

    let v = vec!["hello".to_string(), "world".to_string(), "how".to_string(), "are".to_string(), "you".to_string()];
    let result = neighbour(&v, 3);
    println!("{:?}",result);

    let v = vec!["hello".to_string(), "world".to_string(), "how".to_string(), "are".to_string(), "you".to_string()];
    let result = neighbour(&v, 4);
    println!("{:?}",result);

    // es 17
    let mut v = vec![Some(1),Some(2),None,Some(3)];
    removeelement(&mut v);
    println!("{:?}",v);

    let mut v = vec![None,Some(2),None, None, Some(5)];
    removeelement(&mut v);
    println!("{:?}",v);

    //es 18
    use std::fmt::Debug;
    fn makehashmap()->HashMap<i32,String>{
        let mut h = HashMap::new();
        h.insert(3,"what1".to_string());
        h.insert(4,"what2".to_string());
        h.insert(1,"what3".to_string());
        h.insert(6,"what4".to_string());
        h.insert(22,"what78".to_string());
        return h;
    }
    fn deterministicprinter<T,U>(h:&HashMap<T,U>) where T :
    Debug + Ord, U : Debug + Ord{
        let mut v : Vec<(&T,&U)> = h.iter().collect();
        v.sort();
        println!("{:?}",v);
    }

    let mut h2: HashMap<String,i32> = HashMap::new();
    let mut h1 = makehashmap();
    h2.insert("w".to_string(), 2);
    h2.insert("wh".to_string(), 4);
    h2.insert("wha".to_string(), 1);
    h2.insert("what".to_string(), 8);
    h2.insert("what1".to_string(), 3);
    deterministicprinter(&h1);
    deterministicprinter(&h2);
    hashandhash(&mut h1, &mut h2);
    deterministicprinter(&h1);
    deterministicprinter(&h2);

    let mut h2: HashMap<String,i32> = HashMap::new();
    let mut h1 = makehashmap();
    h2.insert("whoooooooooooooooooooo".to_string(), 2);
    h2.insert("whoo".to_string(), 4);
    h2.insert("who".to_string(), 1);
    h2.insert("whoooo".to_string(), 8);
    h2.insert("wheeeee".to_string(), 2);
    deterministicprinter(&h1);
    deterministicprinter(&h2);
    hashandhash(&mut h1, &mut h2);
    deterministicprinter(&h1);
    deterministicprinter(&h2);

    // es 19
    fn makehashmap2()->HashMap<i32,String>{
        let mut h = HashMap::new();
        h.insert(2,"what1".to_string());
        h.insert(4,"what2".to_string());
        h.insert(1,"what3".to_string());
        h.insert(5,"what".to_string());
        return h;
    }
    fn deterministicprinter2<T,U>(h:&HashMap<T,U>) where T : Debug + Ord,
                                                         U : Debug + Ord{
        let mut v : Vec<(&T,&U)> = h.iter().collect();
        v.sort();
        println!("{:?}",v);

    }

    let mut h1 = makehashmap2();
    deterministicprinter2(&h1);
    let ret = unique(h1, 5);
    println!("{:?}",ret);
    let mut h1 = makehashmap2();
    deterministicprinter2(&h1);
    let ret = unique(h1, 2).unwrap();
    deterministicprinter2(&ret);
}
