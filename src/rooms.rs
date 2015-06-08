use std::fmt;
use std::ptr;


pub struct Room {
    pub description: String,
    north: *mut Room,
    south: *mut Room,
    east: *mut Room,
    west: *mut Room,
}


fn optional_room<'a>(room: *const Room) -> Option<&'a Room> {
    if room.is_null() {
        None
    } else {
        unsafe {
            Some(&*room)
        }
    }
}


impl Room {
    pub fn new(description: &str) -> Room {
        Room {
            description: description.to_string(),
            north: ptr::null_mut(),
            south: ptr::null_mut(),
            east: ptr::null_mut(),
            west: ptr::null_mut(),
        }
    }

    pub fn north(&self) -> Option<&Room> {
        optional_room(self.north)
    }

    pub fn south(&self) -> Option<&Room> {
        optional_room(self.south)
    }

    pub fn east(&self) -> Option<&Room> {
        optional_room(self.east)
    }

    pub fn west(&self) -> Option<&Room> {
        optional_room(self.west)
    }

    unsafe fn door_north_leads_to(&mut self, room: *mut Room) {
        self.north = room;
        (*room).south = self;
    }

    unsafe fn door_south_leads_to(&mut self, room: *mut Room) {
        self.south = room;
        (*room).north = self;
    }

    unsafe fn door_east_leads_to(&mut self, room: *mut Room) {
        self.east = room;
        (*room).west = self;
    }

    unsafe fn door_west_leads_to(&mut self, room: *mut Room) {
        self.west = room;
        (*room).east = self;
    }
}


impl fmt::Display for Room {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.description)
    }
}


pub struct Rooms {
    pub vec: Vec<Box<Room>>,
}


impl Rooms {
    pub fn build() -> Box<Rooms> {
        let mut main_room = Box::new(Room::new("room 1"));
        let mut room2 = Box::new(Room::new("room 2"));
        let mut room3 = Box::new(Room::new("room 3"));
        
        unsafe {
            main_room.door_north_leads_to(&mut *room2);
            room2.door_north_leads_to(&mut *room3);
        }

        Box::new(Rooms { 
            vec: vec!(main_room, room2, room3),
        })
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn main_room(&self) -> &Room {
        match self.vec.first() {
            None => panic!("First room is missing!"),
            Some(room) => room,
        }
    }
}

