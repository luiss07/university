use std::collections::HashMap;

pub fn is_it_luhn(s : &String) -> bool {
    if s.len() <= 1 {
        return false;
    }
    let s = s.replace(" ", "");
    let mut v : Vec<char> =  s.chars().collect();
    let mut sum = 0;
    for i in 0..v.len() {
        if v[i].is_digit(10) {
            if i % 2 == 0 {
                let num = v[i].to_digit(10).unwrap_or(0);
                let res = num * 2;
                if res > 10 {
                    sum += res - 9;
                } else {
                    sum += res;
                }
            } else {
                sum += v[i].to_digit(10).unwrap_or(0);
            }
        } else {
            return false;
        }
    }
     sum % 10 == 0
}

pub fn recognise_owner(hm : &HashMap<String,String>, plate : String) -> Option<&String> {
    hm.get(&plate)
}