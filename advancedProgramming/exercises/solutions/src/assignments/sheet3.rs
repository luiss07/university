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

pub fn sheet3() {
    // SHEET 3

    println!("{}", is_it_luhn(&"4539 3195 0343 6467".to_string()));

    let owner_plate : HashMap<String, String> = HashMap::from([
        ("plate1".to_string(), "owner1".to_string()),
        ("plate2".to_string(), "owner2".to_string()),
        ("plate3".to_string(), "owner2".to_string())
    ]);
    println!("{:?}", owner_plate);

    let mut owner = match recognise_owner(&owner_plate, "plate2".to_string()) {
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
