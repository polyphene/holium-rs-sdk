//! Key trees are responsible for recursively holding structure keys used as inputs in transformations

#[derive(Debug)]
pub struct KeyNode {
    pub value: Option<&'static str>,
    pub children: &'static [KeyNode]
}
