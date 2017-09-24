extern crate json;

mod adapter;
mod business;
mod entity;

use std::env;

use adapter::read_json_configuration;
use business::{load_configuration, hero_apply_item};
use entity::{Attribute, Entity, EntityType, Identity};

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
        let item = Entity::new(
            Identity::new(
                String::from("ring_of_protection"),
                String::from("Ring of protection")
            ),
            EntityType::Item,
            vec!(
                Attribute::new(
                    Identity::new(
                        String::from("armor"), String::from("Armor")
                    ),
                    2
                )
            )
        );
        if e.is_type(EntityType::Hero) {
            println!("{:?}", hero_apply_item(e, item));
        } else {
            println!("{:?}", e);
        }
    }
}

