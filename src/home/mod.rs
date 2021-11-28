use std::collections::HashSet;

use crate::{
    error::{Error, Result},
    room::Room,
};

/// Entry point for smart home control - Home
#[derive(Debug)]
pub struct Home {
    /// Name of the Home
    name: String,
    /// List of rooms in the current home
    rooms: HashSet<Room>,
}

impl Home {
    /// Construct a new empty home
    /// ```
    /// use otus_smart_home::Home;
    ///
    /// let home = Home::new("Sweet Home".to_owned());
    /// assert_eq!(home.name(), "Sweet Home");
    /// assert_eq!(home.rooms().count(), 0);
    /// ```
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
    pub fn add_room(&mut self, room: Room) -> Result<()> {
        if self.rooms.contains(room.name()) {
            Err(Error::RoomAlreadyExists(room))
        } else {
            self.rooms.insert(room);
            Ok(())
        }
    }

    /// Del room from the Home
    pub fn del_room(&mut self, name: &str) -> Option<Room> {
        self.rooms.take(name)
    }

    /// Get room by name
    pub fn room(&self, name: &str) -> Option<&Room> {
        self.rooms.get(name)
    }

    /// Gets a list of rooms in the home
    pub fn rooms(&self) -> impl Iterator<Item = &Room> {
        self.rooms.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::room::Room;

    #[test]
    fn example() {
        let mut home = Home::new("Sweet Home");

        assert_eq!(home.name(), "Sweet Home");
        assert_eq!(home.rooms().count(), 0);

        home.add_room(Room::new("Room 1")).unwrap();
        assert_eq!(home.rooms().count(), 1);
        assert!(home.rooms().any(|room| room.name() == "Room 1"));

        assert!(matches!(
            home.add_room(Room::new("Room 1")),
            Err(Error::RoomAlreadyExists(_))
        ));

        home.del_room("Room 1");
        assert_eq!(home.rooms().count(), 0);
    }

    #[test]
    fn init() {
        let home = Home::new("home");
        assert_eq!(home.name(), "home");
    }

    #[test]
    fn room_management() {
        let mut home = Home::new("home");

        assert_eq!(home.rooms().count(), 0);
        assert_eq!(home.room("NOT FOUND"), None);

        home.add_room(Room::new("room 1")).unwrap();
        assert_eq!(home.rooms().count(), 1);
        assert_eq!(home.room("room 1"), Some(&Room::new("room 1")));

        home.add_room(Room::new("room 2")).unwrap();
        assert_eq!(home.rooms().count(), 2);
        assert_eq!(home.room("room 2"), Some(&Room::new("room 2")));

        let deleted_room = home.del_room("room 1");
        assert_eq!(deleted_room, Some(Room::new("room 1")));
        assert_eq!(deleted_room.as_ref().map(|r| r.name()), Some("room 1"));
        assert_eq!(home.room("room 1"), None);
        assert_eq!(home.rooms().count(), 1);

        let deleted_room = home.del_room("room 2");
        assert_eq!(deleted_room, Some(Room::new("room 2")));
        assert_eq!(deleted_room.as_ref().map(|r| r.name()), Some("room 2"));
        assert_eq!(home.room("room 2"), None);
        assert_eq!(home.rooms().count(), 0);
    }
}
