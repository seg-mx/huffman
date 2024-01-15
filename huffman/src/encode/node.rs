use std::cmp::Ordering;

pub type Link = Box<Node>;

#[derive(Default)]
pub struct Node {
    value: u64,
    character: Option<u8>,
    left_child: Option<Link>,
    right_child: Option<Link>,
}

impl Node {
    pub fn new(value: u64, character: Option<u8>) -> Self {
        Self {
            value,
            character,
            ..Self::default()
        }
    }

    pub fn new_with_childs(
        value: u64,
        character: Option<u8>,
        left_child: Self,
        right_child: Self,
    ) -> Self {
        Self {
            value,
            character,
            left_child: Some(Box::new(left_child)),
            right_child: Some(Box::new(right_child)),
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn character(&self) -> Option<u8> {
        self.character
    }

    pub fn left_child(&self) -> Option<&Self> {
        self.left_child.as_ref().map(|node| node.as_ref())
    }

    pub fn right_child(&self) -> Option<&Self> {
        self.right_child.as_ref().map(|node| node.as_ref())
    }

    pub fn is_leaf(&self) -> bool {
        self.left_child.is_none() && self.right_child.is_none()
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
