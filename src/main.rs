extern crate json;

mod adapter;
mod business;
mod entity;

use std::env;

use adapter::read_json_configuration;
use business::{load_configuration, calculate_attribute_value};

fn main() {
    let conf_file = env::args().nth(1)
        .expect("You must provide a json config file");
    println!("Using configuration file: {:?}", conf_file);
    let entities = load_configuration(
        conf_file.as_str(),
        &read_json_configuration
    );
    println!("Loaded the following entities:");
    for e in entities {
        for a in e.get_attributes() {
            calculate_attribute_value(&a, &e);
        }
    }
}

