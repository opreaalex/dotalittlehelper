use json;

use entity::{Attribute, Entity, EntityType, Identity};

const ERR_MSG_CONF_GENERIC: &'static str = "Wrong json configuration format";

pub fn read_json_configuration(s: &str) -> Vec<Entity> {
    // Some utility functions for creating what we need
    // out of json values
    fn read_identity(j: &json::JsonValue) -> Identity {
        Identity::new(
            String::from(j["id"].as_str().expect(ERR_MSG_CONF_GENERIC)),
            String::from(j["name"].as_str().expect(ERR_MSG_CONF_GENERIC))
        )
    }

    fn read_attribute(j: &json::JsonValue) -> Attribute {
        Attribute::new(
            read_identity(&j["identity"]),
            j["value"].as_i32().expect(ERR_MSG_CONF_GENERIC)
        )
    }

    fn read_entity(j: &json::JsonValue) -> Entity {
        Entity::new(
            read_identity(&j["identity"]),
            read_entity_type(&j["entity_type"]),
            j["attributes"].members().map(|a| read_attribute(&a)).collect()
        )
    }

    fn read_entity_type(j: &json::JsonValue) -> EntityType {
        match j.as_str().expect(ERR_MSG_CONF_GENERIC) {
            "Hero" => EntityType::Hero,
            "Item" => EntityType::Item,
            _ => panic!(ERR_MSG_CONF_GENERIC)
        }
    }
    // End utility functions
    let j = json::parse(s).expect("Unable to parse json configuration");
    j["entities"].members().map(|e| read_entity(&e)).collect()
}

