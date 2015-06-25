mod rooms;
mod rooms_options;

use std::error;
use std::io;
use std::io::Write;
use std::process;


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


fn print_error_and_exit<E: error::Error>(error: E) -> ! {
    writeln!(io::stderr(), "{}", error).unwrap();
    process::exit(2);
}


fn main() {
    let options = rooms_options::RoomsOptions::new();
    let rooms = match options.roomsfile {
        Some(ref roomsfile) => 
            rooms::Rooms::read(&roomsfile)
                         .unwrap_or_else(|e| print_error_and_exit(e)),
        None => rooms::Rooms::new(),
    };
    
    let mut room = rooms.first_room();
    room.print_description_on_entrance();

    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command)
                   .unwrap_or_else(|e| print_error_and_exit(e));

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

