use std::{collections::HashMap, fmt};

use super::{
    node::{Link, Node},
    prefix_code::PrefixCode,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TreeCreationError {
    MissingFrequencies,
}

impl fmt::Display for TreeCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for TreeCreationError {}

pub struct Tree {
    node: Link,
    prefix_codes: Option<HashMap<u8, PrefixCode>>,
}

impl Tree {
    pub fn from_frequencies(frequencies: &HashMap<u8, u64>) -> Result<Self, TreeCreationError> {
        let mut nodes = Vec::new();

        if frequencies.is_empty() {
            return Err(TreeCreationError::MissingFrequencies);
        }

        for (&character, &frequency) in frequencies {
            nodes.push(Node::new(frequency, Some(character)));
        }

        while nodes.len() > 1 {
            nodes.sort_by(|a, b| b.cmp(a));

            let first = nodes.pop().unwrap();
            let second = nodes.pop().unwrap();

            let node = Node::new_with_childs(first.value() + second.value(), None, first, second);
            nodes.push(node);
        }

        Ok(Self {
            node: Box::new(nodes.pop().unwrap()),
            prefix_codes: None,
        })
    }

    fn add_codes(node: &Node, code: PrefixCode, map: &mut HashMap<u8, PrefixCode>) {
        if node.is_leaf() {
            map.insert(node.character().unwrap(), code);
            return;
        }

        Self::add_codes(node.left_child().unwrap(), code.add_zero(), map);
        Self::add_codes(node.right_child().unwrap(), code.add_one(), map);
    }

    pub fn get_prefix_codes(&mut self) -> &HashMap<u8, PrefixCode> {
        if self.prefix_codes.is_some() {
            return self.prefix_codes.as_ref().unwrap();
        }

        let mut prefix_codes = HashMap::new();
        Self::add_codes(&self.node, PrefixCode::default(), &mut prefix_codes);
        self.prefix_codes = Some(prefix_codes);

        self.prefix_codes.as_ref().unwrap()
    }
}
