extern crate dotalittlehelper;

use dotalittlehelper::datatypes::*;

fn main() {
    let luna = Hero {
        id: 1,
        name: String::from("Luna"),
        stats: 32
    };
    println!("{:?}", luna);
}

