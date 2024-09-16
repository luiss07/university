// - define an i32 constant named "CONSTANT" inside a module named "odd_module" and assign to it the value 123
// - define an i32 constant named "CONSTANT" inside a module named "even_module" and assign to it the value 246
// - define a public function "get_constant" inside the module "getter_function" that take as input an u32 named "value", and return
// - the constant inside "odd_module" if "value" is odd. otherwise it returns the constant inside "even_module"
// - just to avoid confusion remember that in Italian: odd = dispari, even = pari

mod odd_module{
    pub const CONSTANT: i32 = 123;
}
mod even_module{
    pub const CONSTANT: i32 = 246;
}
mod getter_function{
    pub fn get_constant(value: u32) -> i32{
        if value%2 == 0{
            super::even_module::CONSTANT
        }else{
            super::odd_module::CONSTANT
        }
    }
}

// ---

#[test]
fn main_test_1(){
    println!("odd constant: {}",odd_module::CONSTANT);
    println!("even constant: {}",even_module::CONSTANT);
    print!("test function: {}", getter_function::get_constant(100));
}
/*
odd constant: 123
even constant: 246
test function: 246
*/




#[test]
fn main_test_2(){
    println!("odd constant: {}",odd_module::CONSTANT);
    println!("even constant: {}",even_module::CONSTANT);
    println!("test function: {}", getter_function::get_constant(105));
}
/*
odd constant: 123
even constant: 246
test function: 123
*/
