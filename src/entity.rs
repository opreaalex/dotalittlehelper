use std::collections::hash_map::{HashMap};

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
    attribute_type: AttributeType,
    calculation: Option<Calculation>,
    value: Option<i32>
}

impl Attribute {
    pub fn new(identity: Identity,
               attribute_type: AttributeType,
               calculation: Option<Calculation>,
               value: Option<i32>) -> Attribute {
        Attribute {
            identity: identity,
            attribute_type: attribute_type,
            calculation: calculation,
            value: value
        }
    }

    pub fn get_type(&self) -> AttributeType {
        self.attribute_type.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AttributeType {
    Base,
    Calculated
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Calculation {
    operands: Vec<Attribute>,
    operation: Operation
}

impl Calculation {
    pub fn new(operands: Vec<Attribute>,
               operation: Operation) -> Calculation {
        Calculation {
            operands: operands,
            operation: operation
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    Add,
    Multiply
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

    pub fn get_attributes(&self) -> Vec<Attribute> {
        self.attributes.iter().map(|(_, v)| v.clone()).collect()
    }
}

