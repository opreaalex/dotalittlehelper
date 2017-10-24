extern crate dotalittlehelper;

use dotalittlehelper::*;

#[test]
fn read_json_configuration() {
    let json_str = r#"{
        "entities": [{
            "identity": {"id": "test", "name": "Test"},
            "entity_type": "Hero",
            "attributes": [{
                "identity": {
                    "id": "test_attr",
                    "name": "Test Attr"
                },
                "attribute_type": "Base",
                "calculation": null,
                "value": 9001
            }]
        }]
    }"#;
    let test_entity = adapter::read_json_configuration(json_str).remove(0);
    let real_entity = entity::Entity::new(
        entity::Identity::new(
            String::from("test"), String::from("Test")
        ),
        entity::EntityType::Hero,
        vec!(
            entity::Attribute::new(
                entity::Identity::new(
                    String::from("test_attr"), String::from("Test Attr")
                ),
                entity::AttributeType::Base,
                None,
                Some(9001)
            )
        )
    );
    assert_eq!(real_entity, test_entity);
}

#[test]
#[should_panic]
fn read_json_configuration_fail() {
    let json_str = r#"{
        "entitties": [{
            "what": "is this"
        }]
    }"#;
    let test_entities = adapter::read_json_configuration(json_str);
    assert_eq!(0, test_entities.len());
    
    let json_str = r#"{
        "entities": [{
            "identity": {"id": "test", "name": "Test"}
        }]
    }"#;
    adapter::read_json_configuration(json_str);
}

