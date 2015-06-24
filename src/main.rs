extern crate getopts;

mod rooms;

use std::env;
use std::io;
use getopts::Options;


fn move_to_next_room_if_possible<'a>(room: &'a rooms::Room, 
                                     next_room: Option<&'a rooms::Room>, 
                                     direction: &str) 
                                     -> &'a rooms::Room
{
    match next_room {
        Some(room) => {
            room.print_description_on_entrance();
            room
        },
        None => {
            println!("There is no door to the {}!", direction);
            room
        },
    }
}


fn main() {
    let mut options = Options::new();
    options.optflag("h", "help", "print this help message");

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let matches = match options.parse(&args[1..]) {
        Ok(matches) => matches,
        Err(error) => { panic!(error.to_string()) },
    };

    if matches.opt_present("h") {
        let brief = format!("Usage: {} [roomsfile]", program);
        print!("{}", options.usage(&brief));
        return; 
    }

    let rooms = match matches.free.first() {
        Some(roomsfile) => {
            rooms::Rooms::read(roomsfile)
                         .unwrap_or_else(|e| panic!("{}", e))
        },
        None => rooms::Rooms::new(),
    };
    
    let mut room = rooms.first_room();
    room.print_description_on_entrance();

    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command).ok().expect("command");

        if command.is_empty() {
            continue;
        }

        let ch = command.chars().next().unwrap();
        match ch {
            'n' => room = move_to_next_room_if_possible(room, room.north(), "north"),
            's' => room = move_to_next_room_if_possible(room, room.south(), "south"),
            'e' => room = move_to_next_room_if_possible(room, room.east(), "east"),
            'w' => room = move_to_next_room_if_possible(room, room.west(), "west"),
            'q' => break,
            _ => println!("Huh?"),
        }
    }
}

