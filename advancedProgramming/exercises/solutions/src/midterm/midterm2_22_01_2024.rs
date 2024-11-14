// ES 1
// trait Nextable {
//     fn gimme_next(&self) -> Option<Self> where Self: Sized;
// }
trait Nextable {
    type res : std::fmt::Debug;
    fn gimme_next(&self) -> Option<Self::res>;
}

impl Nextable for i32 {
    type res = i32;
    fn gimme_next(&self) -> Option<Self::res> {
        Some(self+1)
    }
}

impl Nextable for char {
    type res = char;
    fn gimme_next(&self) -> Option<Self::res> {
        Some(Self::from_u32(*self as u32 + 1)?)
    }
}

fn printnext<T: Nextable + std::fmt::Debug>(value : T) {
    println!("next of {:?} is {:?}",value, value.gimme_next());
}

fn es1() {
    let x = 5;
    printnext(x);
    let s = 's';
    printnext(s);

    let x = 11;
    printnext(x);
    let s = 'f';
    printnext(s);
}

// es 2
use std::str::Chars;
struct ConsIter<'a> {
    iter: Chars<'a>
}

struct Wrapper{
    inner : String
}

impl Wrapper {
    fn iter(&self) -> ConsIter {
        ConsIter {
            iter : self.inner.chars(),
        }
    }
}

impl<'a> Iterator for ConsIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.iter.next() {
            // Check if the character is ASCII and not a vowel
            if c.is_ascii() && !"aeiouAEIOU".contains(c.to_ascii_lowercase()) {
                return Some(c);  // Return the character if it's valid
            }
        }
        None
    }
}

fn es2() {
    let w = Wrapper { inner: "another day, another hangover!".to_string() };
    for (n, c) in w.iter().enumerate() {
        print!("{n}{c}");
    }
    println!();
    let w = Wrapper { inner: "AAAAAAAAAAAAOOOOUUUUUF!".to_string() };
    for (n, c) in w.iter().enumerate() {
        print!("{n}{c}");
    }
    println!();

    let w = Wrapper { inner: "wait, did someone said waffles?".to_string() };
    for (n, c) in w.iter().enumerate() {
        print!("{}{}", c, (0..n).map(|_| c).collect::<String>());
    }
    println!();
    let w = Wrapper { inner: "ôops this letter shouldn't be hēre¡".to_string() };
    for (n, c) in w.iter().enumerate() {
        print!("{}{}", c, (0..n).map(|_| c).collect::<String>());
    }
    println!();
}

// es 3

fn basicbox_sum(vs : Vec<String>) -> Vec<Box<usize>> {
    // let mut vlen = Vec::new();
    // let mut sum = 0;
    // vs.iter().for_each(|c| {
    //     sum += c.len();
    //     vlen.push(Box::new(c.len()))
    // });
    // vlen.push(Box::new(sum));
    // vlen
    let mut sum = 0;
    let mut vlen : Vec<Box<usize>> = vs.iter().map(|s| {
        let len = s.len();
        sum += len;
        Box::new(len)
    }).collect();
    vlen.push(Box::new(sum));
    vlen
}

fn es3() {
    let s = vec!["asd".to_string(), "where".to_string(), "what".to_string()];
    println!("boxed s: {:?}", basicbox_sum(s));

    let s = vec!["nope".to_string(), "game".to_string(), "bananas".to_string()];
    println!("boxed s: {:?}", basicbox_sum(s));
}

// es 4
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

/*I NEED RC<REFCELL<>> because */

#[derive(Debug)]
struct MasterClock {
    clock : Rc<RefCell<usize>>
}

#[derive(Debug)]
struct SlaveClock {
    clock : Rc<RefCell<usize>>
}

impl MasterClock {
    fn new() -> Self {
        MasterClock { clock : Rc::new(RefCell::new(0)) }
    }

    fn tick(&mut self) {
        *self.clock.borrow_mut() += 1;
    }

    fn get_slave(&self) -> SlaveClock {
        SlaveClock {
            clock : Rc::clone(&self.clock)
        }
    }
}

impl SlaveClock {
    fn get_clock(&self) -> usize {
        *self.clock.borrow()
    }
}

fn es4() {
    let mut mc = MasterClock::new();
    println!("{:?}",mc);

    let mut mc = MasterClock::new();
    mc.tick();
    mc.tick();
    println!("{:?}",mc);

    let mut mc = MasterClock::new();
    let sc1 = mc.get_slave();
    println!("{:?}",sc1);

    let mut mc = MasterClock::new();
    let sc2 = mc.get_slave();

    mc.tick();
    mc.tick();
    mc.tick();

    let sc1 = mc.get_slave();
    println!("{}",sc1.get_clock());

    mc.tick();
    mc.tick();

    println!("{}",sc2.get_clock());
    println!("{}",sc1.get_clock());

    let mut mc = MasterClock::new();
    mc.tick();
    mc.tick();

    let sc1 = mc.get_slave();

    println!("{}",sc1.get_clock());

    let sc2 = mc.get_slave();

    mc.tick();
    mc.tick();

    println!("{}",sc2.get_clock());
    println!("{}",sc1.get_clock());
}

// es 5

mod finance {
    // pub use wallet_1::Wallet1;
    // pub use wallet_2::Wallet2;
    pub type Wallet1 = wallet_1::Wallet1;
    pub type Wallet2 = wallet_2::Wallet2;
    pub mod wallet_1 {
        #[derive(Debug)]
        pub struct Wallet {
            pub euro : f32
        }
        pub use Wallet as Wallet1;
    }

    pub mod wallet_2 {
        #[derive(Debug)]
        pub struct Wallet {
            pub euro : u32,
            pub cents : u8
        }
        pub use Wallet as Wallet2;
    }

    pub fn compare_wallet(first : &Wallet1, second : &Wallet2) -> bool {
        let snd_money = second.euro as f32 + (second.cents as f32 / 10f32);
        first.euro > snd_money
    }
}

// use finance::{compare_wallet, wallet_1, wallet_2};
fn es5() {
    let w1 = finance::wallet_1::Wallet{
        euro: 100.
    };
    println!("{:?}",w1);

    let w2 = finance::wallet_2::Wallet{
        euro: 199,
        cents: 50
    };
    println!("{:?}",w2);

    let w1 = finance::wallet_1::Wallet{
        euro: 100.
    };
    let w2 = finance::Wallet2{
        euro: 90,
        cents: 50
    };
    println!("{}",finance::compare_wallet(&w1,&w2));
}

// es 6

#[derive(Debug)]
pub struct Content{
    pub i:i32,
    pub s:String
}
impl Content {
    pub fn new(i: i32, s: String) -> Content {
        Content { i, s }
    }
}

#[derive(Debug)]
struct Node<T> {
    elem: T,
    left: TreeLink<T>,
    center: TreeLink<T>,
    right: TreeLink<T>,
}
impl <T>Node<T>{
    pub fn new(elem:T) -> Node<T> {
        Node{elem,left:None,center:None,right:None}
    }
}
#[derive(Debug)]
pub struct Tree<T> {
    root: TreeLink<T>,
}
type TreeLink<T> = Option<Box<Node<T>>>;

impl<T: Ord> Tree<T> {
    fn new() -> Self {
        Tree {
            root : None
        }
    }

    fn add_recursive(node : &mut Box<Node<T>>, new_node : Box<Node<T>>) {
        if node.elem == new_node.elem {
            match &mut node.center {
                Some(center_node) => {
                    Self::add_recursive(center_node, new_node)
                },
                None => node.center = Some(new_node)
            }
        } else if new_node.elem < node.elem {
            match &mut node.left {
                Some(left_node) => {
                    Self::add_recursive(left_node, new_node)
                },
                None => node.left = Some(new_node)
            }
        } else if new_node.elem > node.elem {
            match &mut node.right {
                Some(right_node) => {
                    Self::add_recursive(right_node, new_node)
                },
                None => node.right = Some(new_node)
            }
        }
    }

    fn add(&mut self, el : T) {
        let new_node = Box::new(Node::new(el));
        if let Some(root) = &mut self.root {
            Self::add_recursive(root, new_node);
        } else {
            self.root = Some(new_node);
        }
    }



    fn howmany_smaller(&self, el : &T) -> i32 {
        let mut count = 0;
        if let Some(node) = &self.root {
            Self::howmany_recursive(node, el, &mut count);
        }
        count
    }

    fn howmany_recursive(node : &Box<Node<T>>, el : &T, count : &mut i32) {
        if node.elem < *el {
            *count += 1;
            if let Some(left_node) = &node.left {
                Self::howmany_recursive(&left_node, el, count);
            }
            if let Some(center_node) = &node.center {
                Self::howmany_recursive(&center_node, el, count);
            }
            if let Some(right_node) = &node.right {
                Self::howmany_recursive(&right_node, el, count);
            }
        }
    }
}

impl PartialEq<Self> for Content {
    fn eq(&self, other: &Self) -> bool {
        self.s.len() == other.s.len()
    }
}

impl PartialOrd for Content {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.s.len() < other.s.len() {
            return Some(Ordering::Less);
        } else if self.s.len() == other.s.len() {
            return Some(Ordering::Equal);
        } else if self.s.len() > other.s.len() {
            return Some(Ordering::Greater);
        };
        None
    }
}

fn es6() {
    let mut t = Tree::new();
    println!("{:?}",t);
    let e0 = Content::new(10,"asd".to_string());
    let e1 = Content::new(10,"who".to_string());
    let e2 = Content::new(11,"oneasd".to_string());
    println!("{}", e1==e2);
    println!("{}", e1==e0);
    println!("{}", e1<e2);
    println!("{}", e1>e2);
    let e1 = 10;
    let e2 = 11;
    let e3 = 8;
    let e4 = 19;
    let e5 = 45;
    t.add(e1); t.add(e2); t.add(e3); t.add(e4); t.add(e5);
    println!("{:?}",t);

    let mut t = Tree{
        root: Some(Box::new(Node{
            elem: 10, //Content::new(10,"what".to_string()),
            left: Some(Box::new(Node{
                elem: 11, //Content::new(11,"who".to_string()),
                left: None,
                center: Some(Box::new(Node{
                    elem: 19, //Content::new(19,"ten".to_string()),
                    left: None,
                    center: None,
                    right: None
                })),
                right: None
            })),
            center: Some(Box::new(Node{
                elem: 15, //Content::new(15,"zips".to_string()),
                left: None,
                center: None,
                right: None
            })),
            right: Some(Box::new(Node{
                elem: 88, //Content::new(88,"whose".to_string()),
                left: None,
                center: None,
                right: None
            }))
        }))
    };
    let e56 = 4; //Content::new(45,"gips".to_string());
    let e57 = 40; //Content::new(45,"naasdasdsad".to_string());
    println!("{}", t.howmany_smaller(&e56));
    println!("{}", t.howmany_smaller(&e57));
}

pub fn midterm2_2024() {
    es1();
    es2();
    es3();
    es4();
    es5();
    es6();
}