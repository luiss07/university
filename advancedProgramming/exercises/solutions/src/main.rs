mod assignments;

use assignments::sheet1 as s1;

fn main() {
    let s = "ciao";
    let s_rev = s1::string_reverse_2(s);
    println!("{}", s_rev);
}
