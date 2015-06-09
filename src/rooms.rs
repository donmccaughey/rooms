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
        let mut timmys_bedroom = Box::new(
            Room::new("Timmy's bedroom", "a young boy's bedroom"));
        let mut upstairs_hallway = Box::new(
            Room::new("upstairs hallway", "a hallway"));
        let mut sallys_bedroom = Box::new(
            Room::new("Sally's bedroom", "a teenage girl's bedroom"));
        let mut master_bedroom = Box::new(
            Room::new("master bedroom", "a bedroom with a king sized bed"));
        let mut kids_bathroom = Box::new(
            Room::new("kid's bathroom", "a messy bathroom with towels on the floor"));
        let mut parents_bathroom = Box::new(
            Room::new("parent's bathroom", "a bathroom with a shower"));

        let mut stairway = Box::new(
            Room::new("stairway", "a stairway"));

        let mut downstairs_hallway = Box::new(
            Room::new("downstairs hallway", "a hallway"));
        let mut livingroom = Box::new(
            Room::new("livingroom", "a large room with a comfy sofa and a big TV"));
        let mut kitchen = Box::new(
            Room::new("kitchen", "a large room with a tile floor that smells like food"));
        
        unsafe {
            timmys_bedroom.door_north_leads_to(&mut *upstairs_hallway);
            timmys_bedroom.door_east_leads_to(&mut *kids_bathroom);
            sallys_bedroom.door_west_leads_to(&mut *upstairs_hallway);
            sallys_bedroom.door_south_leads_to(&mut *kids_bathroom);
            master_bedroom.door_south_leads_to(&mut *upstairs_hallway);
            master_bedroom.door_west_leads_to(&mut *parents_bathroom);
            upstairs_hallway.door_west_leads_to(&mut *stairway);

            stairway.door_north_leads_to(&mut *downstairs_hallway);
            downstairs_hallway.door_north_leads_to(&mut *livingroom);
            downstairs_hallway.door_east_leads_to(&mut *kitchen);
        }

        Box::new(Rooms { 
            vec: vec!(timmys_bedroom, upstairs_hallway, sallys_bedroom, master_bedroom,
                      kids_bathroom, parents_bathroom, stairway, downstairs_hallway,
                      livingroom, kitchen),
        })
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn first_room(&self) -> &Room {
        match self.vec.first() {
            None => panic!("First room is missing!"),
            Some(room) => room,
        }
    }
}

