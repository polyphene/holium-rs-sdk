//! Data trees are responsible for recursively holding holium data. Leaves hold scalar CBOR values
//! while non-leaf nodes point to ordered children.

use crate::internal::key_tree::Node as KeyNode;
use serde::{Deserialize, Serialize};
use serde_cbor::Value as CborValue;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
/// Value held by the leaf of a data tree
pub(crate) enum Value {
    Null,
    Bool(bool),
    Integer(i128),
    Float(f64),
    Bytes(Vec<u8>),
    Text(String),
}

impl Value {
    pub(crate) fn to_cbor(&self) -> CborValue {
        match self {
            Value::Null => CborValue::Null,
            Value::Bool(v) => CborValue::Bool(*v),
            Value::Integer(v) => CborValue::Integer(*v),
            Value::Float(v) => CborValue::Float(*v),
            Value::Bytes(v) => CborValue::Bytes(v.clone()),
            Value::Text(v) => CborValue::Text(v.clone()),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
/// Recursive structure building simple data trees
pub struct Node {
    pub(crate) value: Option<Value>,
    pub(crate) children: Vec<Node>,
}

impl Node {
    /// Create a data tree from a Cbor value
    pub fn new(src_value: CborValue) -> Self {
        fn new_leaf(v: Value) -> Node {
            Node {
                value: Some(v),
                children: vec![],
            }
        }
        fn new_non_leaf(children: Vec<Node>) -> Node {
            Node {
                value: None,
                children,
            }
        }

        match src_value {
            CborValue::Null => new_leaf(Value::Null),
            CborValue::Bool(v) => new_leaf(Value::Bool(v)),
            CborValue::Integer(v) => new_leaf(Value::Integer(v)),
            CborValue::Float(v) => new_leaf(Value::Float(v)),
            CborValue::Bytes(v) => new_leaf(Value::Bytes(v)),
            CborValue::Text(v) => new_leaf(Value::Text(v)),
            CborValue::Tag(_, boxed_value) => Self::new(*boxed_value),
            CborValue::Array(values) => {
                new_non_leaf(values.into_iter().map(|v| Self::new(v)).collect())
            }
            CborValue::Map(tree_map) => {
                new_non_leaf(tree_map.into_values().map(|v| Self::new(v)).collect())
            }
            CborValue::__Hidden => unreachable!(),
        }
    }

    /// Fuse a key tree and a node tree to generate a Cbor structure based on them
    pub fn assign_keys(&self, key_node: &KeyNode) -> CborValue {
        match &self.value {
            Some(value) => value.to_cbor(),
            None => {
                if &key_node.children.len() > &(0usize) {
                    let mut map: BTreeMap<CborValue, CborValue> = BTreeMap::new();

                    for (i, child) in self.children.iter().enumerate() {
                        // We can unwrap here as we will not handle the error more properly later on
                        let key = &key_node.children.get(i).unwrap();
                        map.insert(
                            CborValue::Text(String::from(key.value.unwrap())),
                            child.assign_keys(key),
                        );
                    }

                    CborValue::Map(map)
                } else {
                    let mut cbor_values: Vec<CborValue> = Vec::new();
                    for node in self.children.iter() {
                        cbor_values.push(node.assign_keys(&KeyNode::default()));
                    }
                    CborValue::Array(cbor_values)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::Serialize;
    use serde_cbor::value::to_value;
    use std::collections::BTreeMap;

    use crate::internal::key_tree::GenerateNode;

    use super::*;

    #[test]
    fn can_represent_null_value() {
        assert_eq!(
            Node::new(CborValue::Null),
            Node {
                value: Some(Value::Null),
                children: vec![]
            }
        )
    }

    #[test]
    fn can_represent_boolean_value() {
        assert_eq!(
            Node::new(CborValue::from(true)),
            Node {
                value: Some(Value::Bool(true)),
                children: vec![]
            }
        );
        assert_eq!(
            Node::new(CborValue::from(false)),
            Node {
                value: Some(Value::Bool(false)),
                children: vec![]
            }
        )
    }

    #[test]
    fn can_represent_array() {
        assert_eq!(
            Node::new(CborValue::from(vec![
                CborValue::from(vec![CborValue::Null]),
                CborValue::Null,
            ])),
            Node {
                value: None,
                children: vec![
                    Node {
                        value: None,
                        children: vec![Node {
                            value: Some(Value::Null),
                            children: vec![]
                        }],
                    },
                    Node {
                        value: Some(Value::Null),
                        children: vec![]
                    },
                ],
            }
        )
    }

    #[test]
    fn can_represent_map() {
        let mut tree_map = BTreeMap::new();
        tree_map.insert(CborValue::Null, CborValue::Integer(0));
        assert_eq!(
            Node::new(CborValue::from(tree_map)),
            Node {
                value: None,
                children: vec![Node {
                    value: Some(Value::Integer(0)),
                    children: vec![]
                },],
            }
        )
    }

    #[test]
    fn can_import_tagged_value() {
        assert_eq!(
            Node::new(CborValue::Tag(0, Box::from(CborValue::Null),)),
            Node {
                value: Some(Value::Null),
                children: vec![]
            }
        )
    }

    #[test]
    fn can_assign_map() {
        #[derive(Eq, PartialEq, Serialize)]
        struct Structure {
            key: NestedStructure,
        }
        // This code is generated in the wasm module while compiling
        impl GenerateNode for Structure {
            fn generate_node() -> KeyNode {
                KeyNode {
                    value: None,
                    children: vec![KeyNode {
                        value: Some("key"),
                        children: NestedStructure::generate_node().children,
                    }],
                }
            }
        }

        #[derive(Eq, PartialEq, Serialize)]
        struct NestedStructure {
            key: u8,
        }

        // This code is generated in the wasm module while compiling
        impl GenerateNode for NestedStructure {
            fn generate_node() -> KeyNode {
                KeyNode {
                    value: None,
                    children: vec![KeyNode {
                        value: Some("key"),
                        children: u8::generate_node().children,
                    }],
                }
            }
        }

        let structure = Structure {
            key: NestedStructure { key: 0 },
        };

        let structure_cbor = to_value(structure).unwrap();

        let structure_data = Node::new(structure_cbor.clone());

        let structure_assigned = structure_data.assign_keys(&Structure::generate_node());

        assert_eq!(structure_cbor, structure_assigned);
    }

    #[test]
    fn can_assign_array() {
        #[derive(Eq, PartialEq, Serialize)]
        struct Structure {
            key: Vec<u8>,
        }

        // This code is generated in the wasm module while compiling
        impl GenerateNode for Structure {
            fn generate_node() -> KeyNode {
                KeyNode {
                    value: None,
                    children: vec![KeyNode {
                        value: Some("key"),
                        children: Vec::<u8>::generate_node().children,
                    }],
                }
            }
        }

        let structure = Structure {
            key: vec![0, 1, 2, 3],
        };

        let structure_cbor = to_value(structure).unwrap();

        let structure_data = Node::new(structure_cbor.clone());

        let structure_assigned = structure_data.assign_keys(&Structure::generate_node());

        assert_eq!(structure_cbor, structure_assigned);
    }
}
