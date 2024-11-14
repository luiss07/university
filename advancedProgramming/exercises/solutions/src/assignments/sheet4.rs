use std::fmt::{write, Debug, Display, Formatter};

fn find_equal<'a, 'b>(s1 : &'a str, s2 : &'b str) -> Option<(&'a str, &'b str)> {
    for i in 0..s1.len() {
        for j in 0..s1.len() {
            if i < s1.len()-1 && j < s2.len()-1 {
                if s1[i..i+2] == s2[j..j+2] {
                    return Some((&s1[i..i + 2], &s2[j..j + 2]))
                }
            }
        }
    }
    None
}

fn random_letter() -> char {
    let mut n = rand::random::<u8>();
    n = n % 26 + 'a' as u8;
    n as char
}
fn random_string(len: usize) -> String {
    let mut s = String::with_capacity(len);
    for _ in 0..len{
        s.push(random_letter());
    }
    s
}

fn lucky_slice(input_str : &str) -> Option<&str> {
    let rng_str = random_string(input_str.len());
    println!("Rng string {rng_str}");
    let res = find_equal(input_str, &rng_str);
    match res {
        Some((s1, s2)) => Some(s1),
        None => None
    }
}

// #[derive(Debug)]
struct Person<'a> {
    name : String,
    father : Option<&'a Person<'a>>,
    mother : Option<&'a Person<'a>>
}

impl Debug for Person<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<'a> Person<'a> {
    fn new(name : &'a str, father : Option<&'a Person<'a>>, mother : Option<&'a Person<'a>>) -> Person<'a> {
        Person{ name: name.to_string(), father, mother}
    }

    fn find_relatives_recursive_string<'b>(p : &Person, generations : u32, vp : &'b mut Vec<String>) -> &'b mut Vec<String> {
        vp.push(p.name.clone());
        if generations > 0 {
            if let Some(mother) = p.mother {
                Self::find_relatives_recursive_string(mother, generations-1, vp);
            }
            if let Some(father) = p.father {
                Self::find_relatives_recursive_string(father, generations-1, vp);
            }
        }
        vp
    }

    pub fn find_relatives_string(&self, generations : u32) -> Vec<String> {
        let mut vp = Vec::new();
        Self::find_relatives_recursive_string(&self, generations, &mut vp);
        vp
    }

    fn find_relatives_recursive_person<'b>(&'b self, generations : u32, vp : &mut Vec<&'b Self>){
        vp.push(self);
        if generations > 0 {
            if let Some(mother) = self.mother {
                mother.find_relatives_recursive_person(generations-1, vp);
            }
            if let Some(father) = self.father {
                father.find_relatives_recursive_person(generations-1, vp);
            }
        }
    }

    pub fn find_relatives_person(&self, generations : u32) -> Vec<&Person> {
        let mut vp = Vec::new();
        self.find_relatives_recursive_person(generations, &mut vp);
        vp
    }

    fn find_roots_recursive<'b>(&'b self, vp : &mut Vec<&'b Person<'b>>) {
        match (self.father, self.mother) {
            (None, None) => vp.push(self),
            (father, None) => {
                vp.push(self);
                father.unwrap().find_roots_recursive(vp)
            },
            (None, mother) => {
                vp.push(self);
                mother.unwrap().find_roots_recursive(vp)
            },
            (father, mother) => {
                father.unwrap().find_roots_recursive(vp);
                mother.unwrap().find_roots_recursive(vp);
            }
        }
    }

    pub fn find_roots(&self) -> Vec<&Person>{
        let mut vp = Vec::new();
        self.find_roots_recursive(&mut vp);
        vp
    }
}

// CORRECT THE CODE
struct ImportantExcerpt<'a> {
    part: &'a str,
}
// 'a : 'b needed because we return the &str contained into self, thus we need 'a to outlive 'b
impl<'a : 'b, 'b> ImportantExcerpt<'a> { //THIS LINE
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str
    {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// ES 4
struct DubleRef<'r, 's : 'r, T> {
    r: &'r T,
    s: &'s T
}

// ES 5

trait Split<'a> {
    type ReturnType;
    fn split_trait(&'a self) -> (Self::ReturnType, Self::ReturnType);
}

impl<'a> Split<'a> for String {
    type ReturnType = &'a str;
    fn split_trait(&'a self) -> (Self::ReturnType, Self::ReturnType) {
        let mid = self.len() / 2;
        (&self[..mid], &self[mid..])
    }
}

impl<'a> Split<'a> for &'a [i32] {
    type ReturnType = &'a [i32];
    fn split_trait(&'a self) -> (Self::ReturnType, Self::ReturnType) {
        let mid = self.len() / 2;
        (&self[..mid], &self[mid..])
    }
}

use std::collections::LinkedList;

impl<'a> Split<'a> for LinkedList<f64> {
    type ReturnType = LinkedList<f64>;
    fn split_trait(&'a self) -> (Self::ReturnType, Self::ReturnType) {
        let mut left = self.clone();
        let right = left.split_off(left.len()/2);
        (left,right)
    }
}

// es 7
use std::ops::{Add,Sub};

#[derive(Copy, Clone)]
struct Point {
    x : f32,
    y : f32
}

#[derive(Copy, Clone)]
struct Circle {
    center : Point,
    radius : f32
}

#[derive(Copy, Clone)]
struct Rectangle {
    top_left : Point,
    bottom_right : Point
}

impl Default for Point {
    fn default() -> Self {
        Point{x:0.0,y:0.0}
    }
}

impl Default for Circle {
    fn default() -> Self {
        Circle{center : Point::default(), radius : 1.0}
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self{top_left : Point{x:-1.,y:1.}, bottom_right : Point{x:1.,y:-1.}}
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point{
            x : self.x + rhs.x,
            y : self.y + rhs.y
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Point{
            x : self.x - rhs.x,
            y : self.y - rhs.y
        }
    }
}

struct Area{
    area : f32
}

impl Default for Area {
    fn default() -> Self {
        Self{area : 0.}
    }
}

impl Display for Area{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Area is {} cm^2", self.area)
    }
}

trait GetArea {
    fn get_area(&self) -> Area;
}

impl GetArea for Point {
    fn get_area(&self) -> Area {
        Area::default()
    }
}

impl GetArea for Circle {
    fn get_area(&self) -> Area {
        Area{
            area : self.radius.powi(2)*std::f32::consts::PI
        }
    }
}

impl GetArea for Rectangle {
    fn get_area(&self) -> Area {
        let p = self.bottom_right - self.top_left;
        let area = (p.y * p.x).abs();
        Area{area}
    }
}

impl Add<Area> for Area {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Area{
            area : self.area + rhs.area
        }
    }
}

impl Add<&dyn GetArea> for Area {
    type Output = Area;
    fn add(self, rhs: &dyn GetArea) -> Self::Output {
        Area{
            area: self.area + rhs.get_area().area
        }
    }
}

fn sum_area(slice : &[&dyn GetArea]) -> Area {
    let mut area = Area::default();
    for e in slice {
        area = area + *e;
    }
    area
}

// es 7

fn skip_prefix<'a>(telephone_number : &'a str, prefix : &str) -> &'a str {
    let p_len = prefix.len();
    if telephone_number[..p_len].contains(prefix) {
        return &telephone_number[p_len..]
    }
    telephone_number
}

// ES 8

struct Chair<'a> {
    color : &'a str,
    quantity : &'a usize
}

struct Wardrobe<'a> {
    color : &'a str,
    quantity : &'a usize
}

trait Object {
    fn build(&self) -> &str;
    fn get_quantity(&self) -> String;
}

impl<'a> Object for Chair<'a> {
    fn build(&self) -> &str {
        "Chair has been built"
    }
    fn get_quantity(&self) -> String {
        format!("We have {} chairs", self.quantity)
    }
}

impl<'a> Object for Wardrobe<'a> {
    fn build(&self) -> &str {
        "Wardrobe has been built"
    }
    fn get_quantity(&self) -> String {
        format!("We have {} wardrobe", self.quantity)
    }
}

impl<'a> Display for Chair<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.quantity {
            0 => write!(f, "We have 0 chairs"),
            _ => write!(f, "We have {} chairs of color {}", self.quantity, self.color)
        }
    }
}

impl<'a> Display for Wardrobe<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.quantity {
            0 => write!(f, "We have 0 wardrobe"),
            _ => write!(f, "We have {} wardrobe of color {}", self.quantity, self.color)
        }
    }
}

// ES 9
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug)]
enum Role {
    GUEST,
    USER,
    ADMIN
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Permission {
    READ,
    WRITE,
    EXECUTE
}

struct Actions {
    action : String,
    permission : HashMap<Permission, bool>
}

struct User {
    name : String,
    role : Role,
    actions : Vec<Actions>
}

trait Auth {
    fn check_permission(&self, action : &str, permission_type : &Permission) -> bool;
    fn can_write(&self, string : &str) -> bool;
    fn can_read(&self, string : &str) -> bool;
    fn can_execute(&self, string : &str) -> bool;
}

impl Auth for User {
    fn check_permission(&self, action : &str, permission_type : &Permission) -> bool {
        // for a in self.actions {
        //     if a.action == action {
        //         return match a.permission.get(permission_type) {
        //             Some(&v) => v,
        //             None => false
        //         }
        //     }
        // }
        // false
        self.actions.iter().any(|x| {
            x.action == action
                && x.permission.get(permission_type).cloned().unwrap_or(false)
        })
    }
    fn can_write(&self, string: &str) -> bool {
        self.check_permission(string, &Permission::WRITE)
    }
    fn can_read(&self, string: &str) -> bool {
        self.check_permission(string, &Permission::READ)
    }
    fn can_execute(&self, string: &str) -> bool {
        self.check_permission(string, &Permission::EXECUTE)
    }
}

impl Default for Actions {
    fn default() -> Self {
        Self{
            action : "".to_string(),
            permission : HashMap::from([
                (Permission::READ, false),
                (Permission::WRITE, false),
                (Permission::EXECUTE, false)
            ])
        }
    }
}

impl Actions {
    fn new(action : String, read : bool, write : bool, execute : bool) -> Self {
        Self{
            action,
            permission : HashMap::from([
                (Permission::READ, read),
                (Permission::WRITE, write),
                (Permission::EXECUTE, execute)
            ])
        }
    }
}

impl Default for User {
    fn default() -> Self {
        Self{
            name: "Guest".to_string(),
            role: Role::GUEST,
            actions: Vec::new(),
        }
    }
}

impl User {
    fn change_role(&mut self, role : Role) -> Result<(), String> {
        // if self.role == Role::ADMIN {
        //     self.role = role;
        //     Ok(())
        // } else if self.role == Role::USER && role == Role::GUEST {
        //     self.role = role;
        //     Ok(())
        // } else if self.role == Role::GUEST && role == Role::GUEST {
        //     Ok(())
        // } else {
        //     Err("Cannot change to higher role!".to_string())
        // }
        match (&mut self.role, role) {
            (Role::ADMIN, role) => { self.role = role; Ok(()) },
            (Role::USER, Role::GUEST) => { self.role = Role::GUEST; Ok(()) },
            (Role::GUEST, Role::GUEST) => { Ok(()) },
            (_,_) => Err("Cannot change to higher role!".to_string())
        }
    }
}

fn sudo_change_permission(user : &mut User, string : String, permission : Permission, value : bool) {
    user.actions.iter_mut().for_each(|x| {
        if x.action == string {
            x.permission.insert(permission, value);
        }
    })
}

fn es9_test_cases() {
    // Create actions with specific permissions
    let read_action = Actions::new("Read Document".to_string(), true, false, false);
    let write_action = Actions::new("Write Document".to_string(), true, true, false);
    let execute_action = Actions::new("Execute Script".to_string(), true, true, true);

    // Create a user with an initial GUEST role and add actions
    let mut user = User {
        name: "Alice".to_string(),
        role: Role::GUEST,
        actions: vec![read_action, write_action, execute_action],
    };

    // Display initial user role and permissions
    println!("User: {}", user.name);
    println!("Initial Role: {:?}", user.role);
    println!(
        "Can read 'Read Document'?: {}",
        user.can_read("Read Document")
    );
    println!(
        "Can write 'Write Document'?: {}",
        user.can_write("Write Document")
    );
    println!(
        "Can execute 'Execute Script'?: {}",
        user.can_execute("Execute Script")
    );

    // Attempt to change role from GUEST to USER (should fail)
    println!("\nAttempting to change role to USER:");
    match user.change_role(Role::USER) {
        Ok(_) => println!("Role changed successfully to USER."),
        Err(e) => println!("Failed to change role: {}", e),
    }

    // Promote user to ADMIN
    println!("\nPromoting user to ADMIN:");
    user.role = Role::ADMIN;
    println!("Current Role: {:?}", user.role);

    // Now, change role from ADMIN to USER (should succeed)
    println!("\nAttempting to change role from ADMIN to USER:");
    match user.change_role(Role::USER) {
        Ok(_) => println!("Role changed successfully to USER."),
        Err(e) => println!("Failed to change role: {}", e),
    }

    // Verify role is now USER
    println!("Current Role: {:?}", user.role);

    // Check permissions after role change
    println!("\nChecking permissions for actions as USER:");
    println!(
        "Can read 'Read Document'?: {}",
        user.can_read("Read Document")
    );
    println!(
        "Can write 'Write Document'?: {}",
        user.can_write("Write Document")
    );
    println!(
        "Can execute 'Execute Script'?: {}",
        user.can_execute("Execute Script")
    );

    // Use sudo_change_permission to grant EXECUTE permission on "Read Document"
    println!("\nGranting EXECUTE permission on 'Read Document':");
    sudo_change_permission(&mut user, "Read Document".to_string(), Permission::EXECUTE, true);

    // Check permissions after sudo_change_permission
    println!("Checking updated permissions:");
    println!(
        "Can execute 'Read Document' after sudo change?: {}",
        user.can_execute("Read Document")
    );
}

pub fn sheet4() {

    // ES 1
    println!("Slice of length 2: {:?}", find_equal("ciaocgfd", "gfao"));
    println!("Slice of length 2: {:?}", find_equal("tredfgrht", "htrdfght"));
    println!("Slice of length 2: {:?}", lucky_slice("tredfgrht"));

    // ES 2
    let claudioFather = Person::new(
        "Pino",
        None,
        None
    );
    let luigiFather = Person::new(
        "claudio",
        Some(&claudioFather),
        None
    );

    let alessandraFather = Person::new(
        "Beniamino",
        None,
        None
    );
    let linaMother = Person::new(
        "giuseppina",
        None,
        None
    );
    let alessandraMother = Person::new(
        "lina",
        None,
        Some(&linaMother)
    );
    let luigiMother = Person::new(
        "alessandra",
        Some(&alessandraFather),
        Some(&alessandraMother)
    );

    let person1 = Person::new(
        "luigi",
        Some(&luigiFather),
        Some(&luigiMother)
    );
    println!("Relatives: {:?}", person1.find_relatives_person(1));
    println!("Relatives w one parent to None: {:?}", person1.find_roots());

    // ES 3 - CORRECT THE CODE
    // ES 4 - IMPL THIS CODE
    // ES 5
    let s = String::from("ciaocia");
    println!("Splitted string: {:?}", s.split_trait());
    let v = [1,2,3,4,5];
    let slice = v.split_at(4);
    println!("Splitted &[i32]: {:?}", slice.0.split_trait());
    let ll:LinkedList<f64> = LinkedList::from([1.2,2.0,3.0,4.0,5.0,6.0]);
    println!("Splitted &[i32]: {:?}", ll.split_trait());

    // ES 6
    let circle = Circle { center: Point::default(), radius: 2.0 };
    let rectangle = Rectangle {
        top_left: Point { x: -1.0, y: 1.0 },
        bottom_right: Point { x: 2.0, y: -2.0 },
    };
    let shapes: [&dyn GetArea; 2] = [&circle, &rectangle];
    let total_area = sum_area(&shapes);
    assert!((total_area.area - (12.56637 + 9.0)).abs() < 1e-5);
    println!("total_area {}", (total_area.area - (12.56637 + 9.0)).abs());

    // ES 7
    println!("skip_prefix: {}", skip_prefix("ciao38920465", "ciao"));
    println!("skip_prefix: {}", skip_prefix("ciao38920465", "ao"));

    // ES 8
    let quantity = 5;
    let chair = Chair {
        color: "red",
        quantity: &quantity,
    };
    println!("{}", chair.build());
    println!("{}", chair.get_quantity());
    let chair = Chair {
        color: "yellow",
        quantity: &0,
    };
    println!("{}", chair);

    let quantity = 2;
    let wardrobe = Wardrobe {
        color: "white",
        quantity: &quantity,
    };
    println!("{}", wardrobe.build());

    // ES 9
    es9_test_cases();
}