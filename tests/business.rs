extern crate dotalittlehelper;

use dotalittlehelper::*;

#[test]
fn hero_apply_item_fail() {
    let hero = entity::Entity::new(
        entity::Identity::new(
            String::from("test"), String::from("Test")
        ),
        entity::EntityType::Hero,
        vec!(
            entity::Attribute::new(
                entity::Identity::new(
                    String::from("strenght"), String::from("Strenght")
                ),
                9001
            )
        )
    );
    let item = entity::Entity::new(
        entity::Identity::new(
            String::from("item"), String::from("Item")
        ),
        entity::EntityType::Item,
        vec!(
            entity::Attribute::new(
                entity::Identity::new(
                    String::from("inteligence"), String::from("Inteligence")
                ),
                9001
            )
        )
    );
    let result = business::hero_apply_item(item, hero);
    assert_eq!(true, result.is_err());
}

