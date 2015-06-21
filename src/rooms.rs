use std::error;
use std::fmt;
use std::fs;
use std::io;
use std::io::BufRead;
use std::ptr;


fn as_option<'a, T>(p: *const T) -> Option<&'a T> {
    match p.is_null() {
        true => None,
        false => unsafe {
            Some(&*p)
        },
    }
}


pub struct Room {
    pub name: String,
    pub description: String,
    north: *mut Room,
    south: *mut Room,
    east: *mut Room,
    west: *mut Room,
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
        as_option(self.north)
    }

    pub fn south(&self) -> Option<&Room> {
        as_option(self.south)
    }

    pub fn east(&self) -> Option<&Room> {
        as_option(self.east)
    }

    pub fn west(&self) -> Option<&Room> {
        as_option(self.west)
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


#[derive(Debug)]
pub enum RoomsError {
    Io(io::Error),
    InvalidFile(String),
}


impl fmt::Display for RoomsError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RoomsError::Io(ref error) => write!(formatter, "IO error: {}", error),
            RoomsError::InvalidFile(ref message) => write!(formatter, "Invalid file: {}", message),
        }
    }
}


impl error::Error for RoomsError {
    fn description(&self) -> &str {
        match *self {
            RoomsError::Io(ref error) => error.description(),
            RoomsError::InvalidFile(ref message) => &message,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            RoomsError::Io(ref error) => Some(error),
            RoomsError::InvalidFile(_) => None,
        }
    }
}


impl From<io::Error> for RoomsError {
    fn from(error: io::Error) -> RoomsError {
        RoomsError::Io(error)
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

    pub fn read(path: &str) -> Result<Box<Rooms>, RoomsError> {
        let mut rooms = Rooms {
            vec: Vec::new(),
        };
        let file = try!(fs::File::open(path));
        let reader = io::BufReader::new(file);

        for result in reader.lines() {
            let line = try!(result);
            let whitespace: &[_] = &[' ', '\n', '\r', '\t'];
            let trimmed_line = line.trim_matches(whitespace);
            if trimmed_line.is_empty() { continue; }
            if trimmed_line.starts_with("#") { continue; }
            let columns: Vec<&str> = trimmed_line.split(';').collect();
            match columns[0] {
                "room" => {
                    if columns.len() != 3 {
                        let message = format!("Expected 3 columns for room, found {}", columns.len()); 
                        return Err(RoomsError::InvalidFile(message));
                    }
                    let name = columns[1].trim_matches(whitespace);
                    let description = columns[2].trim_matches(whitespace);
                    rooms.add_room(name, description);
                },
                "door" => {
                    if columns.len() != 4 {
                        let message = format!("Expected 4 columns for door, found {}", columns.len());
                        return Err(RoomsError::InvalidFile(message));
                    }
                    let first_room_name = columns[1].trim_matches(whitespace);
                    let direction = columns[2].trim_matches(whitespace);
                    let second_room_name = columns[3].trim_matches(whitespace);

                    unsafe {
                        let first_room: *mut Room; 
                        match rooms.find(first_room_name) {
                            Some(found_room) => first_room = found_room,
                            None => {
                                let message = format!("No room named \"{}\"", first_room_name);
                                return Err(RoomsError::InvalidFile(message));
                            },
                        }
                        let second_room: *mut Room;
                        match rooms.find(second_room_name) {
                            Some(found_room) => second_room = found_room,
                            None => {
                                let message = format!("No room named \"{}\"", second_room_name);
                                return Err(RoomsError::InvalidFile(message));
                            },
                        }
                        if direction == "north" {
                            (*first_room).door_north_leads_to(second_room);
                        } else if direction == "south" {
                            (*first_room).door_south_leads_to(second_room);
                        } else if direction == "east" {
                            (*first_room).door_east_leads_to(second_room);
                        } else if direction == "west" {
                            (*first_room).door_west_leads_to(second_room);
                        } else {
                            let message = format!("Unknown direction \"{}\"", direction);
                            return Err(RoomsError::InvalidFile(message));
                        }
                    }
                },
                _ => {
                    let message = format!("Unexpected row type \"{}\"", columns[0]);
                    return Err(RoomsError::InvalidFile(message));
                }
            }
        }

        if rooms.vec.is_empty() {
            return Err(RoomsError::InvalidFile("No rooms found.".to_string()));
        }
        Ok(Box::new(rooms))
    }

    fn add_room(&mut self, name: &str, description: &str) -> *mut Room {
        self.vec.push(Box::new(Room::new(name, description)));
        &mut **self.vec.last_mut().unwrap()
    }

    unsafe fn find(&mut self, name: &str) -> Option<*mut Room> {
        for i in 0..self.vec.len() {
            let room = self.vec.get_unchecked_mut(i);
            if room.name == name {
                return Some(&mut **room);
            }
        }
        None
    }

    pub fn first_room(&self) -> &Room {
        self.vec.first().expect("first room")
    }
}

