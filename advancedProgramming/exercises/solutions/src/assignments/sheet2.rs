
pub fn modify_odd(v : &mut [u32]) {
    for elem in v.iter_mut() {
        if (*elem % 2) != 0 {
            *elem = 0;
        }
    }
}

use std::cmp::max;
use std::collections::HashMap;
use std::ops::Index;

pub fn count_character(s : &str) -> HashMap<char, u32> {
    let mut str_map: HashMap<char, u32> = HashMap::new();

    /* OPTION 1*/
    // let mut value : u32 = 0;
    // for c in s.chars() {
    //     value = 0;
    //     if (str_map.contains_key(&c)) {
    //         value = str_map.remove(&c).unwrap_or(0);
    //     }else {
    //         str_map.insert(c, 1);
    //     }
    //     if (value != 0){
    //         value += 1;
    //         str_map.insert(c, value);
    //     }
    // }

    /* OPTION 2 */
    // for c in s.chars() {
    //     if let Some(val) = str_map.get_mut(&c) {
    //         *val += 1;
    //     } else {
    //         str_map.insert(c, 1);
    //     }}

    /* OPTION 3 */
    for c in s.chars() {
        let v = *str_map.get_mut(&c).unwrap_or(&mut 0)+1;
        str_map.insert(c, v);
    }
    str_map
}

pub fn split_at_value(slice: &mut [i32], value : i32) -> Option<(&[i32], &[i32])> {
    let len = slice.len();
    if !slice.contains(&value) {
        return None
    }
    let mut index = 0;
    for i   in 0..len {
        if slice[i] == value {
            index = i;
        }
    }
    let (slice1, slice2) = slice.split_at(index);
    Some((slice1, slice2))
}

pub fn sub_slice(v1 : &Vec<i32>, v2 : &Vec<i32>) {
    let mut found = false;
    let mut v2_i = 0;
    for i in 0..v1.len() {
        if v2_i < v2.len() {
            if v1[i] == v2[v2_i] {
                v2_i += 1;
                found = true;
            } else {
                v2_i = 0;
                found = false;
            }
        } else {
            break;
        }
    }
    if found {
        println!("{:?}", v2);
    } else {
        println!("Not found");
    }
}

pub fn max_iter(v : &[i32]) -> Option<i32> {
    if v.len() == 0 {
        return None;
    }
    let mut max = v[0];
    for el in v.iter() {
        if *el > max {
            max = *el;
        }
    }
    Some(max)
}

pub fn max_rec(v : &[i32]) -> Option<i32> {
    if v.len() == 0 {
        return None;
    }
    if v.len() == 1 {
        return Some(v[0]);
    }

    let (split1, split2) = v.split_at(v.len() / 2);

    let max1 = max_rec(split1);
    let max2 = max_rec(split2);

    match (max1, max2) {
        (None, None) => None,
        (e1, None) => e1,
        (None, e2) => e2,
        (Some(e1), Some(e2)) => Some(max(e1,e2))
    }
}

pub fn swap(v : &mut [i32]) {
    if v.len() <= 1 {
        return;
    }

    let tmp = v[0];
    v[0] = v[v.len()-1];
    v[v.len()-1] = tmp;
}

pub fn is_sorted_rec(v : &[i32]) -> bool {
    if v.len() < 2 {
        return true;
    }

    if v[v.len()-1] >= v[v.len()-2] {
        is_sorted_rec(v.split_at(v.len()-1).0)
    }else{
        false
    }
}

pub fn insert_if_longer(vs : &mut Vec<String>, s : String) {
    if s.len() > 10 {
        vs.push(s);
    }
}

use std::slice::Iter;

pub fn build_vector(iter : Iter<i32>) -> Vec<&i32> {
    // let mut v = Vec::new();
    // for e in iter {
    //     v.push(e);
    // }
    // v

    /*OPTION 2*/
    let v : Vec<&i32> = iter.collect();
    v
}

pub fn find_max_index(v : &Vec<i32>, k : usize) -> usize {
    let mut index = 0;
    for i in 0..k {
        if v[i] > v[index] {
            index = i;
        }
    }
    index
}

pub fn pancake_sort(v : &mut Vec<i32>) {
    let mut len = v.len();
    //let mut v_sort = v.clone();
    while len > 1 {
        let max_index = find_max_index(v, len);
        if max_index != len - 1 {
            if max_index != 0 {
                //let (split1, split2) = v.split_at(max_index);
                //split1.reverse()
                v[..max_index+1].reverse()
            }
            v[..len].reverse();
        }
        len -= 1;
    }
}

pub fn merge_split(split1 : &[i32], split2 : &[i32]) -> Vec<i32> {
    let mut v = Vec::new();
    let mut index1 = 0;
    let mut index2 = 0;

    for i in 0..split1.len() + split2.len() {
        if index1 != split1.len() && index2 != split2.len() {
            if split1[index1] < split2[index2] {
                v.push(split1[index1]);
                index1 += 1;
            } else {
                v.push(split2[index2]);
                index2 += 1;
            }
        } else if index1 == split1.len() {
            v.push(split2[index2]);
            index2 += 1;
        } else {
            v.push(split1[index1]);
            index1 += 1;
        }
    }
    v
}

enum Operation {
    Add,
    Sub,
    Mul,
    Div
}

enum Expression {
    Number(i32),
    Operation {
        left : Box<Expression>,
        right : Box<Expression>,
        op : Operation
    }
}

// pub fn evaluate_expression(ex : Expression) -> Result<i32, String> {
//
// }