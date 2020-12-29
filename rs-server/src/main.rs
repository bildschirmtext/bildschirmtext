mod cept;

use cept::*;

fn main() {
    let mut cept = Cept::new();
    cept.add_str(&"hello");
    println!("{:?}", cept.data());
}
