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

pub fn calculate_attribute_value(a: &Attribute, e: &Entity) -> Result<i32, String> {
    match a.get_type() {
        AttributeType::Base => match a.get_value() {
            Some(v) => Ok(v),
            None => Err(String::from("Base attribute must have a value"))
        },
        AttributeType::Calculated => match a.get_calculation() {
            Some(c) => {
                let values: Vec<i32> = c.get_operands().iter()
                    .filter(|o| o.get_value().is_some())
                    .map(|o| o.get_value().unwrap())
                    .collect();
                let mut sum = 0;
                match c.get_operation() {
                    Operation::Add => {
                        for v in values {
                            sum += v;
                        }
                    },
                    Operation::Multiply => {
                        for v in values {
                            sum *= v;
                        }
                    }
                };
                Ok(sum)
            },
            None => Err(String::from("Calculated attribute must have a calculation"))
        }
    }
}

