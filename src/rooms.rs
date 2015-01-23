use std::cell::RefCell;
use std::fmt;
use std::rc::{Rc, Weak};


type RoomRef = Rc<RefCell<Room>>;
type WeakRoomRef = Weak<RefCell<Room>>;


pub struct Room {
  pub description: String,
  pub north: Option<WeakRoomRef>,
  pub south: Option<WeakRoomRef>,
  pub east: Option<WeakRoomRef>,
  pub west: Option<WeakRoomRef>,
}


impl Room {
  pub fn new(description: &str) -> Room {
    Room {
      description: description.to_string(),
      north: None,
      south: None,
      east: None,
      west: None,
    }
  }

  pub fn new_ref(description: &str) -> RoomRef {
    Rc::new(RefCell::new(Room::new(description)))
  }
}

impl fmt::Show for Room {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(formatter, "{}", self.description)
  }
}

impl fmt::String for Room {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(formatter, "{}", self.description)
  }
}

trait RoomBuilder {
  fn link_north(&self, room_ref: &RoomRef);
  fn link_south(&self, room_ref: &RoomRef);
  fn link_east(&self, room_ref: &RoomRef);
  fn link_west(&self, room_ref: &RoomRef);
}

impl RoomBuilder for RoomRef {
  fn link_north(&self, room_ref: &RoomRef) {
    self.borrow_mut().north = Some(room_ref.clone().downgrade());
    room_ref.borrow_mut().south = Some(self.clone().downgrade());
  }

  fn link_south(&self, room_ref: &RoomRef) {
    self.borrow_mut().south = Some(room_ref.clone().downgrade());
    room_ref.borrow_mut().north = Some(self.clone().downgrade());
  }

  fn link_east(&self, room_ref: &RoomRef) {
    self.borrow_mut().east = Some(room_ref.clone().downgrade());
    room_ref.borrow_mut().west = Some(self.clone().downgrade());
  }

  fn link_west(&self, room_ref: &RoomRef) {
    self.borrow_mut().west = Some(room_ref.clone().downgrade());
    room_ref.borrow_mut().east = Some(self.clone().downgrade());
  }
}


pub struct Rooms {
  pub room_refs: Vec<RoomRef>,
}


impl Rooms {
  pub fn build() -> Rooms {
    let mut rooms = Rooms { 
      room_refs: Vec::new(),
    };

    rooms.room_refs.push(Room::new_ref("room 1"));
    rooms.room_refs.push(Room::new_ref("room 2"));
    rooms.room_refs.push(Room::new_ref("room 3"));

    rooms.room_refs[0].link_north(&rooms.room_refs[1]);
    rooms.room_refs[1].link_north(&rooms.room_refs[2]);

    rooms
  }

  pub fn len(&self) -> usize {
    self.room_refs.len()
  }

  pub fn main_room_ref(&self) -> WeakRoomRef {
    self.room_refs[0].clone().downgrade()
  }
}

