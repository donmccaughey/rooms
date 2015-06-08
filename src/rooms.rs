use std::fmt;
use std::ptr;


pub struct Room {
    pub name: String,
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
    pub fn new(name: &str, description: &str) -> Room {
        Room {
            name: name.to_string(),
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
        write!(formatter, "{}", self.name)
    }
}


pub struct Rooms {
    pub vec: Vec<Box<Room>>,
}


impl Rooms {
    pub fn build() -> Box<Rooms> {
        let mut timmys_bedroom = Box::new(
            Room::new("Timmy's bedroom", "A young boy's bedroom"));
        let mut upstairs_hallway = Box::new(Room::new("upstairs hallway", "A hallway"));
        let mut sallys_bedroom = Box::new(
            Room::new("Sally's bedroom", "A teenage girl's bedroom"));
        let mut master_bedroom = Box::new(
            Room::new("Master bedroom", "A bedroom with a king sized bed"));
        let mut kids_bathroom = Box::new(
            Room::new("Kid's bathroom", "A messy bathroom with towels on the floor"));
        let mut parents_bathroom = Box::new(
            Room::new("Parent's bathroom", "A bathroom with a shower"));
        
        unsafe {
            timmys_bedroom.door_north_leads_to(&mut *upstairs_hallway);
            timmys_bedroom.door_east_leads_to(&mut *kids_bathroom);
            sallys_bedroom.door_west_leads_to(&mut *upstairs_hallway);
            sallys_bedroom.door_south_leads_to(&mut *kids_bathroom);
            master_bedroom.door_south_leads_to(&mut *upstairs_hallway);
            master_bedroom.door_west_leads_to(&mut *parents_bathroom);
        }

        Box::new(Rooms { 
            vec: vec!(timmys_bedroom, upstairs_hallway, sallys_bedroom, master_bedroom,
                      kids_bathroom, parents_bathroom),
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

