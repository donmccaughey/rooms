use std::fmt;
use std::ptr;
use std::collections::HashMap;


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

    pub fn doors_description(&self) -> String {
        let mut doors: Vec<&str> = Vec::new();
        if ! self.north.is_null() { doors.push("north"); }
        if ! self.south.is_null() { doors.push("south"); }
        if ! self.east.is_null() { doors.push("east"); }
        if ! self.west.is_null() { doors.push("west"); }

        match doors.len() {
            1 => format!("There is a door to the {}.", doors[0]),
            2 => format!("There are doors to the {} and {}.", doors[0], doors[1]),
            3 => format!("There are doors to the {}, {} and {}.", 
                         doors[0], doors[1], doors[2]),
            4 => format!("There are doors in all directions."),
            _ => panic!("Found {} doors in {}!", doors.len(), self.name),
        }
    }

    pub fn print_description_on_entrance(&self) {
        println!("You find yourself in {}.", self.description);
        println!("{}", self.doors_description());
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
    pub fn new() -> Box<Rooms> {
        let mut rooms = Rooms {
            vec: Vec::new(),
        };

        unsafe {
            let timmys_bedroom = rooms.add_room("Timmy's bedroom", "a young boy's bedroom");
            let upstairs_hallway = rooms.add_room("upstairs hallway", "a hallway");
            (*timmys_bedroom).door_north_leads_to(upstairs_hallway);
            
            let sallys_bedroom = rooms.add_room("Sally's bedroom", 
                                                "a teenage girl's bedroom");
            (*sallys_bedroom).door_west_leads_to(upstairs_hallway);
            
            let kids_bathroom = rooms.add_room("kid's bathroom", 
                                               "a messy bathroom with towels on the floor");
            (*kids_bathroom).door_west_leads_to(timmys_bedroom);
            (*kids_bathroom).door_north_leads_to(sallys_bedroom);

            let master_bedroom = rooms.add_room("master bedroom", 
                                                "a bedroom with a king sized bed");
            (*master_bedroom).door_south_leads_to(upstairs_hallway);

            let parents_bathroom = rooms.add_room("parent's bathroom", 
                                                  "a bathroom with a shower");
            (*parents_bathroom).door_east_leads_to(master_bedroom);

            let stairway = rooms.add_room("stairway", "a stairway");
            (*stairway).door_east_leads_to(upstairs_hallway);

            let downstairs_hallway = rooms.add_room("downstairs hallway", "a hallway");
            (*downstairs_hallway).door_south_leads_to(stairway);

            let livingroom = rooms.add_room("livingroom", 
                                            "a large room with a comfy sofa and a big TV");
            (*livingroom).door_south_leads_to(downstairs_hallway);

            let kitchen = rooms.add_room("kitchen", 
                                         "a large room with a tile floor that smells like food");
            (*kitchen).door_west_leads_to(downstairs_hallway);
        }
        Box::new(rooms)
    }

    fn add_room(&mut self, name: &str, description: &str) -> *mut Room {
        self.vec.push(Box::new(Room::new(name, description)));
        &mut **self.vec.last_mut().unwrap()
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn first_room(&self) -> &Room {
        self.vec.first().expect("first room")
    }
}

