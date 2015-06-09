mod rooms;

use std::io;


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
    let rooms = rooms::Rooms::new();
    let mut room = rooms.first_room();
    room.print_description_on_entrance();

    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command).ok().expect("Failed to read command!");

        if command.starts_with("n") {
            room = move_to_next_room_if_possible(room, room.north(), "north");
        } else if command.starts_with("s") {
            room = move_to_next_room_if_possible(room, room.south(), "south");
        } else if command.starts_with("e") {
            room = move_to_next_room_if_possible(room, room.east(), "east");
        } else if command.starts_with("w") {
            room = move_to_next_room_if_possible(room, room.west(), "west");
        } else if command.starts_with("q") {
            break;
        } else {
            println!("Huh?");
        }
    }
}

