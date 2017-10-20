use std::fs::File;
use std::io::prelude::*;

use entity::*;

pub fn load_configuration(path: &str,
                          func: &Fn(&str) -> Vec<Entity>) -> Vec<Entity> {
    func(read_file_contents(path).as_str())
}

fn read_file_contents(path: &str) -> String {
    let mut contents = String::new();
    File::open(path)
        .expect(format!("File not found: {}", path).as_str())
        .read_to_string(&mut contents)
        .expect(format!("Unable to read file: {}", path).as_str());
    contents
}

pub fn calculate_attribute_value(e: &Entity, a: &Attribute) -> i32 {
    match a.get_type() {
        AttributeType::Base => println!("This is fixed"),
        AttributeType::Calculated => println!("This is calculated")
    }
    println!("{:?}", e);
    0
}

