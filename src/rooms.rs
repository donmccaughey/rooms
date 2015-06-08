use std::fmt;


pub struct Room {
    pub description: String,
    pub north: Option<usize>,
    pub south: Option<usize>,
    pub east: Option<usize>,
    pub west: Option<usize>,
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
}


impl fmt::Display for Room {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.description)
    }
}


pub struct Rooms {
    pub vec: Vec<Room>,
}


impl Rooms {
    pub fn build() -> Box<Rooms> {
        let mut rooms = Box::new(Rooms { 
            vec: Vec::new(),
        });

        rooms.vec.push(Room::new("room 1"));
        rooms.vec.push(Room::new("room 2"));
        rooms.vec.push(Room::new("room 3"));

        rooms.vec[0].north = Some(1);
        rooms.vec[1].south = Some(0);

        rooms.vec[1].north = Some(2);
        rooms.vec[2].south = Some(1);

        rooms
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

