use std::collections::HashMap;

mod assignments;
mod midterm;

use assignments::sheet1 as s1;
use assignments::sheet2 as s2;
use assignments::sheet3 as s3;
use midterm::midterm_10_10_2023 as mt1;
use midterm::midterm_11_10_2022_v1 as mt2;


fn sheet1() {
    // SHEET 1 EXERCISES
    println!("{}", s1::string_reverse_2("ciao"));

    let mut str = String::from("prova");
    let str2 = s1::append(str.clone());
    println!("{}, {}", str, str2);
    println!("{}", s1::is_armstrong_2(10));
    println!("original: {:?} and transposed {:?}", ((1,2), (3,4)), s1::transpose(((1,2), (3,4))))
}

fn sheet2() {
    // SHEET 2 EXERCISES

    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let ( slice1, slice2 ) = v.split_at_mut(5);
    println!("{:?}, {:?}", slice1, slice2);
    s2::modify_odd(slice1);
    println!("vector {:?}", v);

    println!("hash map {:?}", s2::count_character("aaabbbccc"));

    let mut v:Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let ( slice1, slice2 ) = v.split_at_mut(5);
    println!("{:?}", s2::split_at_value(slice1, 10).unwrap_or((&[0], &[0])));

    s2::sub_slice(&v, &vec![2,3,4]);

    let v = vec![5,2,7,20,4,5,0,34];
    println!("{:?}", s2::max_iter(&v).unwrap_or(0));
    println!("{:?}", s2::max_rec(&v).unwrap_or(0));

    let mut v_sort:Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8,9];
    println!("{:?}", s2::is_sorted_rec(&v));
    println!("{:?}", s2::is_sorted_rec(&v_sort));

    let mut vs = vec!["test".to_string()];
    s2::insert_if_longer(&mut vs, "test".to_string());
    println!("{:?}", vs);
    s2::insert_if_longer(&mut vs, "ciaociaociao".to_string());
    println!("{:?}", vs);

    let mut v = vec![5,2,7,20,4,5,0,34];
    println!("{:?}", s2::build_vector(v.iter()));

    s2::pancake_sort(&mut v);
    println!("{:?}", v);

    println!("{:?}", s2::merge_split(&[1,3,5,7,9,11], &[2,4,6,8,10,12]));
    println!("{:?}", s2::merge_split(&[1,3,5,7,9,11], &[10,12,12,14,16,18]));

    enum DoubleType {
        T1(i32),
        T2(String)
    }

    let v_i32_str = vec![DoubleType::T1(1), DoubleType::T2("string".to_string())];

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

}

fn sheet3() {
    // SHEET 3

    println!("{}", s3::is_it_luhn(&"4539 3195 0343 6467".to_string()));

    let owner_plate : HashMap<String, String> = HashMap::from([
        ("plate1".to_string(), "owner1".to_string()),
        ("plate2".to_string(), "owner2".to_string()),
        ("plate3".to_string(), "owner2".to_string())
    ]);
    println!("{:?}", owner_plate);

    let mut owner = match s3::recognise_owner(&owner_plate, "plate2".to_string()) {
        Some(owner) => owner,
        None => panic!("Owner not found")
    };
    println!("{}", owner);

    // owner = match s3::recognise_owner(&owner_plate, "plate5".to_string()) {
    //     Some(owner) => owner,
    //     None => panic!("Owner not found")
    // };
    // println!("{}", owner);

    #[derive(Eq, Hash, PartialEq)]
    enum Item {
        Pipas,
        Coke,
        Coffee,
        Juice
    }

    enum Coin {
        TenCents,
        TwentyCents,
        FiftyCents,
        Euro1,
        Euro2
    }

    impl Coin {
        fn to_cents(&self) -> u32 {
            match &self {
                Coin::TenCents => 10,
                Coin::TwentyCents => 20,
                Coin::FiftyCents => 50,
                Coin::Euro1 => 100,
                Coin::Euro2 => 200,
            }
        }
    }

    struct VendingMachine {
        items : HashMap<Item, usize>, // tracks Item : availableItems
        coins : u32 // cents currently inserted into the machine
    }

    impl VendingMachine {
        fn new(items: HashMap<Item, usize>) -> VendingMachine {
            VendingMachine {
                items,
                coins: 0
            }
        }

        fn add_item(&mut self, item : Item, new_items : usize) {
            let value = self.items.get(&item).unwrap_or(&(0usize));
            self.items.insert(item, value+new_items);
        }

        fn insert_coin(&mut self, coin : Coin) -> Result<&str, &str> {
            let check = match coin {
                Coin::TenCents => Ok("Inserted 10"),
                Coin::TwentyCents => Ok("Inserted 20"),
                Coin::FiftyCents => Ok("Inserted 50"),
                Coin::Euro1 => Ok("Inserted 100"),
                Coin::Euro2 => Ok("Inserted 200"),
            };

            if check.is_ok() {
                self.coins += coin.to_cents();
            }
            check
        }

        pub fn get_item_price(item : &Item) -> u32 {
            match item {
                Item::Coffee => 100,
                Item::Coke => 200,
                Item::Juice => 300,
                Item::Pipas => 400,
            }
        }

        fn buy(&mut self, item : &Item) -> Result<u32, &str> {
            if self.coins < VendingMachine::get_item_price(item) {
                Err("Not enough money").expect("TODO: panic message")
            }

            if let Some(available) = self.items.get(&item) {
                if *available > 0 {
                    let change = self.coins - VendingMachine::get_item_price(item);
                    self.coins = 0;
                    Ok(change)
                } else {
                    Err("Item terminated")
                }
            } else {
                Err("Item not available")
            }

        }
    }

    let coin = Coin::Euro2;
    println!("{}", coin.to_cents());

}


fn midterm_10_10_2023() {

    // 1 no x is moved
    // 2 error
    // 3 20 20
    // 4 return 8
    // 5 yes
    // 6 error passing array as non mutable
    // 7 No, v is moved at line 2, and can't be used at line 3

    // es 9
    let a1 = mt1::A::A1(1,2,3);
    let a2 = mt1::A::A2('a','b');
    println!("B2: {:?}, B1:{:?}", mt1::bforma(a1), mt1::bforma(a2));
    let a1 = mt1::A::A1(1,6,30);
    let a2 = mt1::A::A2('t','z');
    println!("B2: {:?}, B1:{:?}", mt1::bforma(a1), mt1::bforma(a2));

    // es 10
    let e1 = mt1::E::A("hello".to_string());
    let e2 = mt1::E::B(true);
    println!("{:?} {:?}", e1, e1.count_vowels());
    println!("{:?} {:?}", e2, e2.count_vowels());

    let f1 = mt1::F::new();
    let f2 = mt1::F::F2(10);
    let f3 = mt1::F::F2(20);
    println!("{:?} {:?}", f1, f1.calculation());
    println!("{:?} {:?}", f2, f2.calculation());
    println!("{:?} {:?}", f3, f3.calculation());

    // es 11
    mt1::print_n(Some(3));
    mt1::print_n(None);

    // es 12
    let b = mt1::Balance{amt:100,active:true};
    let b2 = mt1::Balance{amt:200,active:true};
    println!("maybericher {:?}", b.maybericher(b2));

    let b = mt1::Balance{amt:100,active:true};
    let b2 = mt1::Balance{amt:0,active:true};
    println!("maybericher {:?}", b.maybericher(b2));

    let b = mt1::Balance{amt:100,active:false};
    let b2 = mt1::Balance{amt:200,active:true};
    println!("maybericher {:?}", b.maybericher(b2));

    let b = mt1::Balance{amt:100,active:true};
    let b2 = mt1::Balance{amt:200,active:false};
    println!("maybericher {:?}", b.maybericher(b2));

    // es 13
    let g = mt1::G::new(4, 2);
    let result = g.square();
    println!("{:?}", result);

    // es 14
    let mut x = mt1::X::new();
    let mut y = mt1::Y::new();
    println!("X {:?} - Y {:?}", x, y);
    let (mut x, mut y) = mt1::swapstr(x,y);
    println!("X {} - Y {}", x, y);
    let z1 = x.getstr();
    let z2 = y.getstr();
    println!("{},{},{},{}",z1,z2,x.s,y.c);

    // es 15
    let l = mt1::L::new();
    let m = mt1::M::new();
    println!("{:?} {:?}",l,m);

    let l = mt1::L::new_with_params("world".to_string(), 10);
    let m = mt1::M::new_with_params("world".to_string(), 10.0);
    println!("{:?} {:?}",l,m);

    let mut l = mt1::L::new_with_params("world".to_string(), 10);
    let mut m = mt1::M::new_with_params("hello".to_string(), 10.0);
    mt1::swap_string(&mut l, &mut m);
    println!("{:?} {:?}",l,m);

    // es 16
    let v = vec!["hello".to_string(), "world".to_string(), "how".to_string(), "are".to_string(), "you".to_string()];
    let result = mt1::neighbour(&v, 0);
    println!("{:?}",result);

    let v = vec!["hello".to_string(), "world".to_string(), "how".to_string(), "are".to_string(), "you".to_string()];
    let result = mt1::neighbour(&v, 3);
    println!("{:?}",result);

    let v = vec!["hello".to_string(), "world".to_string(), "how".to_string(), "are".to_string(), "you".to_string()];
    let result = mt1::neighbour(&v, 4);
    println!("{:?}",result);

    // es 17
    let mut v = vec![Some(1),Some(2),None,Some(3)];
    mt1::removeelement(&mut v);
    println!("{:?}",v);

    let mut v = vec![None,Some(2),None, None, Some(5)];
    mt1::removeelement(&mut v);
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
    mt1::hashandhash(&mut h1,&mut h2);
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
    mt1::hashandhash(&mut h1,&mut h2);
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
    let ret = mt1::unique(h1, 5);
    println!("{:?}",ret);
    let mut h1 = makehashmap2();
    deterministicprinter2(&h1);
    let ret = mt1::unique(h1, 2).unwrap();
    deterministicprinter2(&ret);
}

fn midterm_11_10_2022() {
    // 1 no, x is moved
    // 2 error
    // 3 error (cannot mutate the owner while it is &mut by another variable)
    // 4 error
    // 5 error
    // 6 100
    // 7 z
    // 8 error

    // es 9
    let xx1 = mt2::XX::Y1(2,3);
    println!("data {}",mt2::data(&xx1));
    let xx1 = mt2::XX::Y2(String::from("test"));
    println!("data {}",mt2::data(&xx1));

    // es 10
    let z1 = mt2::Z::Y1(1,2);
    let z2 = mt2::Z::Y2(true,String::from("new"));
    println!("len {:?}", mt2::maybelength(&z1));
    println!("len {:?}", mt2::maybelength(&z2));

    // es 11
    let ex = mt2::enumx::X::Y(String::from("test"));
    let sx = mt2::structx::X{i:String::from("asd")};
    println!("Longer {}", mt2::modfun::longer(&ex,&sx));
    let ex = mt2::enumx::X::Y(String::from("asdasd"));
    println!("Longer {}", mt2::modfun::longer(&ex,&sx));

    // es 12
    println!("Maybesqrt 4 {:?} ", mt2::maybesqrt(4));
    println!("Maybesqrt -4 {:?} ", mt2::maybesqrt(-4));

    // es 13
    let b = mt2::Balance{amt:100};
    println!("maybewith {:?}", b.maybewithdraw(10));
    println!("maybewith {:?}", b.maybewithdraw(200));

    // es 14
    println!("char {}, prev {}", 'c', mt2::prevchar('c'));
    println!("char {}, prev {}", 'a', mt2::prevchar('a'));
    println!("char {}, prev {}", 'z', mt2::prevchar('z'));
    let mut s = String::from("Pign");
    println!("S {}",s);
    let ret = mt2::replwithprev(&mut s);
    println!("Returned: {:?}",ret);
    let mut s = String::from("pigna");
    println!("S {}",s);
    let ret = mt2::replwithprev(&mut s);
    println!("Returned: {:?}",ret);

    // es 15
    let x = mt2::X::new();
    let y = mt2::Y::new();
    println!("X {:?} - Y {:?}", x, y);
    let (x,y) = mt2::swapstr(x,y);
    println!("X {} - Y {}", x, y);

    // es 16
    let c1 = mt2::C::C1(0,1);
    let c2 = mt2::C::C2(true, String::from("no way jose"));
    println!("gotten {:?}",mt2::D::new());
    let d1 = mt2::D::new_with_c(c1);
    println!("dbg {:?}",d1);
    println!("fmt D: {}",d1);
    let d2 = mt2::D::new_with_c(c2);
    println!("dbg {:?}",d2);
    println!("fmt D: {}",d2);
    println!("larger {}",d1.larger());
    println!("larger {}",d2.larger());

    // es 17
    let mut my_vec = vec!["a".to_string(), "b".to_string(), "c".to_string()];

    // Swapping and concatenating elements at indices 0 and 1
    if let Some(v) = mt2::swapelconcat(&mut my_vec, 0, 1) {
        println!("Mutated vector: {:?}", v);
    } else {
        println!("Invalid indices");
    }

    // es 18
    let mut v1 = vec![String::from("ab");4];
    println!("Lengths {:?}", mt2::veclengths(&v1));

    // es 19
    let mut v: Vec<String> = vec![String::from("what");4];
    v.push(String::from("now"));    v.push(String::from("what"));
    println!("{:?}",v);
    mt2::removeelfromvector(&mut v, 3);
    println!("{:?}",v);
}


fn main() {

    // sheet1();
    // sheet2();
    // sheet3();

    // midterm_10_10_2023();
    midterm_11_10_2022();
}