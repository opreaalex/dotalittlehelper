extern crate dotalittlehelper;

use dotalittlehelper::*;
/*
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
*/

#[test]
fn calculate_attribute_value() {
    let e = entity::Entity::new(
        entity::Identity::new(String::from("hero"), String::from("Hero")),
        entity::EntityType::Hero,
        vec!(
            entity::Attribute::new(
                entity::Identity::new(String::from("base"), String::from("Base")),
                entity::AttributeType::Base,
                None,
                Some(31)
            ),
            entity::Attribute::new(
                entity::Identity::new(String::from("lvl"), String::from("Lvl")),
                entity::AttributeType::Base,
                None,
                Some(10)
            ),
            entity::Attribute::new(
                entity::Identity::new(String::from("calc"), String::from("Calc")),
                entity::AttributeType::Calculated,
                Some(entity::Calculation::new(
                    vec!(
                        entity::Attribute::new(
                            entity::Identity::new(String::from("base"), String::from("Base")),
                            entity::AttributeType::Base,
                            None,
                            Some(31)
                        ),
                        entity::Attribute::new(
                            entity::Identity::new(String::from("lvl"), String::from("Base")),
                            entity::AttributeType::Base,
                            None,
                            Some(10)
                        )
                    ),
                    entity::Operation::Add
                )),
                None
            )
        )
    );
    let i = entity::Identity::new(String::from("calc"), String::from("Calc"));
    let a = e.get_attribute(&i).expect("Unable to find attribute: calc");
    assert_eq!(41, business::calculate_attribute_value(&a, &e).unwrap());
}

