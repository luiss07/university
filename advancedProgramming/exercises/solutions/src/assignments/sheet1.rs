// Sheet 1 exercises

pub fn string_reverse(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn string_reverse_2(s: &str) -> String {
    let mut s_rev = String::new();
    for c in s.chars().rev() {
        s_rev.push(c);
    }
    s_rev
}

// mut s: String pass the ownership to s
pub fn append(mut s: String) -> String {
    s.push_str("foobar");
    s
}

pub fn is_armstrong(num: i32) -> bool {
    let count = num.to_string().len();
    let num_s = num.to_string();
    let mut sum = 0;
    for c in num_s.chars() {
        let number = match c.to_digit(10) {
            None => 0,
            Some(digit) => digit as i32
        };
        sum += number.pow(count as u32);
    }
    num == sum
}

pub fn is_armstrong_2(mut n: i32) -> bool {
    let original = n.clone();
    let mut digits = Vec::new();
    while n != 0 {
        digits.push(n % 10);
        n /= 10;
    }
    let num_digits = digits.len() as u32;
    let mut sum = 0;
    for d in digits {
        sum += d.pow(num_digits);
    }
    original == sum
}

pub fn transpose(mat : ((u32, u32), (u32, u32))) -> ((u32, u32), (u32, u32)) {
    let transposed_mat = ((mat.0.0, mat.1.0), (mat.0.1, mat.1.1));
    transposed_mat
}

pub fn sheet1() {
    // SHEET 1 EXERCISES
    println!("{}", string_reverse_2("ciao"));

    let mut str = String::from("prova");
    let str2 = append(str.clone());
    println!("{}, {}", str, str2);
    println!("{}", is_armstrong_2(10));
    println!("original: {:?} and transposed {:?}", ((1,2), (3,4)), crate::assignments::sheet1::transpose(((1, 2), (3, 4))))
}