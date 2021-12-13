use std::collections::{hash_map::Entry, HashMap};

use crate::{
    error::{Error, Result},
    room::Room,
};

/// Entry point for smart home control - Home
/// ```
/// use smart_home_lib::{Home, Room};
///
/// let mut home = Home::new("Sweet Home".to_owned());
/// assert_eq!(home.name(), "Sweet Home");
/// assert_eq!(home.room_iter().count(), 0);
///
/// home.add_room(Room::new("Room 1")).unwrap();
/// assert_eq!(home.room_iter_mut().count(), 1);
/// assert!(home.room_iter().any(|room| room.name() == "Room 1"));
///
/// home.del_room("Room 1").unwrap();
/// assert_eq!(home.room_iter().count(), 0);
/// ```
#[derive(Debug)]
pub struct Home {
    /// Name of the Home
    name: String,
    /// List of rooms in the current home
    rooms: HashMap<String, Room>,
}

impl Home {
    /// Construct a new empty home
    pub fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            name: name.into(),
            rooms: Default::default(),
        }
    }

    /// Gets a home name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Add room to the Home
    pub fn add_room(&mut self, room: Room) -> Result<&mut Room> {
        match self.rooms.entry(room.name().to_string()) {
            Entry::Occupied(_) => Err(Error::RoomAlreadyExists(room)),
            Entry::Vacant(entry) => Ok(entry.insert(room)),
        }
    }

    /// Del room from the Home
    pub fn del_room(&mut self, name: &str) -> Option<Room> {
        self.rooms.remove(name)
    }

    /// Get room by name
    pub fn room(&self, name: &str) -> Option<&Room> {
        self.rooms.get(name)
    }

    /// Get iterator over rooms
    pub fn room_iter(&self) -> impl Iterator<Item = &Room> {
        self.rooms.iter().map(|(_, room)| room)
    }

    /// Get mutable iterator over rooms
    pub fn room_iter_mut(&mut self) -> impl Iterator<Item = &mut Room> {
        self.rooms.iter_mut().map(|(_, room)| room)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::room::Room;

    #[test]
    fn example() {
        let mut home = Home::new("Sweet Home");

        assert_eq!(home.name(), "Sweet Home");
        assert_eq!(home.room_iter().count(), 0);
        assert_eq!(home.room_iter_mut().count(), 0);

        home.add_room(Room::new("Room 1")).unwrap();
        assert_eq!(home.room_iter().count(), 1);
        assert_eq!(home.room_iter_mut().count(), 1);
        assert!(home.room_iter().any(|room| room.name() == "Room 1"));

        assert!(matches!(
            home.add_room(Room::new("Room 1")),
            Err(Error::RoomAlreadyExists(_))
        ));

        home.del_room("Room 1");
        assert_eq!(home.room_iter().count(), 0);
        assert_eq!(home.room_iter_mut().count(), 0);
    }

    #[test]
    fn init() {
        let home = Home::new("home");
        assert_eq!(home.name(), "home");
    }

    #[test]
    fn room_management() {
        let mut home = Home::new("home");

        assert_eq!(home.room_iter().count(), 0);
        assert_eq!(home.room("NOT FOUND"), None);

        home.add_room(Room::new("room 1")).unwrap();
        assert_eq!(home.room_iter().count(), 1);
        assert_eq!(home.room("room 1"), Some(&Room::new("room 1")));

        home.add_room(Room::new("room 2")).unwrap();
        assert_eq!(home.room_iter().count(), 2);
        assert_eq!(home.room("room 2"), Some(&Room::new("room 2")));

        let deleted_room = home.del_room("room 1");
        assert_eq!(deleted_room, Some(Room::new("room 1")));
        assert_eq!(deleted_room.as_ref().map(|r| r.name()), Some("room 1"));
        assert_eq!(home.room("room 1"), None);
        assert_eq!(home.room_iter().count(), 1);

        let deleted_room = home.del_room("room 2");
        assert_eq!(deleted_room, Some(Room::new("room 2")));
        assert_eq!(deleted_room.as_ref().map(|r| r.name()), Some("room 2"));
        assert_eq!(home.room("room 2"), None);
        assert_eq!(home.room_iter().count(), 0);
    }
}
