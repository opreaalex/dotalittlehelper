extern crate dotalittlehelper;

use dotalittlehelper::*;

#[test]
fn add_two_entities() {
    let first = entity::Entity::new(
        entity::Identity::new(
            String::from("test"), String::from("Test")
        ),
        entity::EntityType::Hero,
        vec!(entity::Attribute::new(
            entity::Identity::new(
                String::from("strenght"), String::from("Strenght")
            ),
            9001
        ))
    );
    let second = entity::Entity::new(
        entity::Identity::new(
            String::from("test2"), String::from("Test2")
        ),
        entity::EntityType::Item,
        vec!(entity::Attribute::new(
            entity::Identity::new(
                String::from("agility"), String::from("Agility")
            ),
            9001
        ))
    );
    let expected = entity::Entity::new(
        entity::Identity::new(
            String::from("test"), String::from("Test")
        ),
        entity::EntityType::Hero,
        vec!(entity::Attribute::new(
            entity::Identity::new(
                String::from("strenght"), String::from("Strenght")
            ),
            9001
        ), entity::Attribute::new(
            entity::Identity::new(
                String::from("agility"), String::from("Agility")
            ),
            9001
        ))
    );
    assert_eq!(expected, first + second);
}

