use std::ops::Add;
use std::collections::hash_map::{HashMap, Entry};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Identity {
    id:   String,
    name: String
}

impl Identity {
    pub fn new(id: String, name: String) -> Identity {
        Identity {
            id: id,
            name: name
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attribute {
    identity: Identity,
    value:    i32
}

impl Attribute {
    pub fn new(identity: Identity, value: i32) -> Attribute {
        Attribute {
            identity: identity,
            value: value
        }
    }
}

impl Add for Attribute {
    type Output = Attribute;

    fn add(self, other: Attribute) -> Attribute {
        if self.identity != other.identity {
            return self;
        }
        Attribute { 
            identity: self.identity,
            value: self.value + other.value
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EntityType {
    Hero,
    Item
}

#[derive(Clone, Debug, PartialEq)]
pub struct Entity {
    identity:    Identity,
    entity_type: EntityType,
    attributes:  HashMap<Identity, Attribute>
}

impl Entity {
    pub fn new(identity: Identity,
               entity_type: EntityType,
               attributes: Vec<Attribute>) -> Entity {
        let mut attr_vec = HashMap::new();
        for a in attributes {
            attr_vec.insert(a.identity.clone(), a);
        }
        Entity {
            identity: identity,
            entity_type: entity_type,
            attributes: attr_vec
        }
    }

    pub fn is_type(&self, t: EntityType) -> bool {
        self.entity_type == t
    }
}

impl Add for Entity {
    type Output = Entity;

    fn add(self, other: Entity) -> Entity {
        let mut attributes = self.attributes;
        for (key, val) in other.attributes {
            match attributes.entry(key) {
                Entry::Occupied(mut e) => {
                    let attr = e.get_mut(); 
                    *attr = attr.clone() + val;
                }, 
                Entry::Vacant(e) => { e.insert(val); }
            }
        }
        Entity {
            identity: self.identity,
            entity_type: self.entity_type,
            attributes: attributes

        }
    }
}

