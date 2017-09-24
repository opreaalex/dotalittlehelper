use std::fs::File;
use std::io::prelude::*;

use entity::{Entity, EntityType};

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

pub fn hero_apply_item(hero: Entity, item: Entity) -> Result<Entity, String> {
    if !hero.is_type(EntityType::Hero) && 
        !item.is_type(EntityType::Item) {
        return Result::Err(String::from(
            "Must provide Hero and Item arguments, in that order"
        ));
    }
    Result::Ok(hero + item)
}

