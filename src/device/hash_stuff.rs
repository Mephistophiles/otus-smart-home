use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
};

use super::Device;

impl PartialEq for Device {
    fn eq(&self, rhs: &Self) -> bool {
        self.name == rhs.name
    }
}
impl Eq for Device {}

impl Hash for Device {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.name.hash(state);
    }
}

impl Borrow<str> for Device {
    fn borrow(&self) -> &str {
        &self.name
    }
}
