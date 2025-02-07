use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use bimap::BiHashMap;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Type {
    Bool,
    Double,
    Float,
    Int16,
    Int32,
    Int64,
    Int8,
    String,
    Uint16,
    Uint32,
    Uint64,
    Uint8,
    Array(Box<Type>),
    Table(String), // the String here is the name of the type referred to.
    Variant(Vec<Type>),
    Tuple(Vec<Type>),
    PrototypeReference(String), // the string here is the name of the prototype type being referenced
    PrototypeReference2(Vec<String>), // This is just a specialization of the PrototypeReference type which allows multiple types to be references
    PrototypeReferenceBase(String), // This is just a specialization of the PrototypeReference type which allows any prototypes inheriting from a class to be used.
    StringLiteral(String), // this is the only exception in the defaults setting system. This value cannot be changed and is always the literal provided.

    // specifically for tables which are different in a specific use context, but don't have a different name
    TableWithOverrides(String, HashMap<String, Value>),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum Value {
    Bool(bool),
    Double(f64),
    Float(f32),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int8(u8),
    String(String),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Uint8(u8),
    Array(Vec<Value>),
    Table(HashMap<String, Value>),
    Variant(Box<Value>),
    Tuple(Vec<Value>),
    PrototypeReference(String), // the string here is the name of the prototype being referenced (which is part of the type of prototypes that is named in the table definition).
    Nil, // this is used as the value for any unset field (specifically important for optional fields which do not have default values)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Table {
    pub attributes: HashMap<String, Type>,

    // if a field isn't in here, it's considered required. To make an optional field which doesn't have a default value, set its default value to Value::Nil
    pub defaults: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct TypeSet {
    pub types: HashMap<String, Type>,

    // Mapping prototype types to the actual value for the prototype type field
    pub prototypes: BiHashMap<String, String>,

    pub prototype_abstractness: HashMap<String, bool>,

    pub prototype_inheritance: HashMap<String, Option<String>>,
}

impl TypeSet {
    pub fn new() -> Self {
        Self {
            types: Default::default(),
            prototypes: Default::default(),
            prototype_abstractness: Default::default(),
            prototype_inheritance: Default::default(),
        }
    }

    pub fn add_type(&mut self, name: String, t: Type) {
        self.types.insert(name, t);
    }

    pub fn setup_prototype(&mut self, type_name: String, prototype_name: String, is_abstract: bool, parent_type: Option<String>) {
        self.prototypes.insert(type_name.clone(), prototype_name);
        self.prototype_abstractness.insert(type_name.clone(), is_abstract);
        self.prototype_inheritance.insert(type_name.clone(), parent_type);
    }
}

impl Table {
    pub fn new() -> Self {
        Self {
            attributes: Default::default(),
            defaults: Default::default(),
        }
    }

    pub fn add_required_field(&mut self, name: impl AsRef<str>, t: Type) {
        self.attributes.insert(String::from(name.as_ref()), t);
    }

    pub fn add_optional_field(&mut self, name: impl AsRef<str>, t: Type)  {
        self.attributes.insert(String::from(name.as_ref()), t);
    }

    pub fn add_field(&mut self, name: impl AsRef<str>, t: Type, default: Value) {
        let name = String::from(name.as_ref());
        self.attributes.insert(name.clone(), t);
        self.defaults.insert(name, default);
    }

    pub fn add_literal_field(&mut self, name: impl AsRef<str>, literal_value: impl AsRef<str>) {
        self.attributes.insert(String::from(name.as_ref()), Type::StringLiteral(String::from(literal_value.as_ref())));
    }
}