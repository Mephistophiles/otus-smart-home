use super::Room;

use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
};

impl PartialEq for Room {
    fn eq(&self, rhs: &Self) -> bool {
        self.name == rhs.name
    }
}
impl Eq for Room {}
impl Hash for Room {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.name.hash(state);
    }
}

impl Borrow<str> for Room {
    fn borrow(&self) -> &str {
        &self.name
    }
}
