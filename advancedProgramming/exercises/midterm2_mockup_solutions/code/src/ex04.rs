// Write a struct `BinIter` that implements `Iterator` over `bool`s.
// - `BinIter` must have a function `new` that takes as input `n` the number and `l` the length.
// - The iterator must yield bits according to the binary form of `n`, after returning the `l`-th bit the iterator stops.
// - The bits yielded must be in "little-endian" order, so the most significant bit must be yielded last.

struct BinIter {
    n: u128,
    l: usize
}

impl BinIter {
    fn new(n: u128, l: usize) -> Self {
        Self { n, l }
    }
}

impl Iterator for BinIter {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let o = Some(self.n % 2 != 0);
        self.n >>= 1;
        if self.l == 0 {
            None
        } else {
            self.l -= 1;
            o
        }
    }
}

// ---

#[test]
fn test() {
    for n in BinIter::new(4641312598359562305508665788689819792, 128) {
        print!("{}", if n { 1 } else { 0 })
    }
    println!();
    for n in BinIter::new(18956403638425120546, 64) {
        print!("{}", if n { 1 } else { 0 })
    }
    println!();
    for n in BinIter::new(15, 4) {
        print!("{}", if n { 1 } else { 0 })
    }
}
/*
00001001000101000010010100111011011100010100010101010110001111010100110110100100100010010011011100000110010001111011111011000000
0100010011110110000000101111111000110101001101010100100011100000
1111
 */

#[test]
fn test2() {
    for n in BinIter::new(1568948940, 16) {
        print!("{}", if n { "*" } else { "_" })
    }
    println!();
    for n in BinIter::new(8978540, 16) {
        print!("{}", if n { "*" } else { "_" })
    }
    println!();
    for n in BinIter::new(375, 9) {
        print!("{}", if n { "*" } else { "_" })
    }
}
/*
__**__**_*____*_
__**_**_________
***_***_*
 */
