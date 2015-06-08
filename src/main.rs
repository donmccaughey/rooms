mod rooms;


fn main() {
    let rooms = rooms::Rooms::build();
    println!("There are {} rooms to explore.", rooms.len());

    for room in rooms.vec.iter() {
        println!("  {}", room);
    }

    let mut room = rooms.main_room();
    println!("Starting in {}", room);
    println!("Walking north:");
    loop {
        match room.north {
            None => {
                println!("No door to the north");
                break;
            },
            Some(i) => {
                let optional_room = rooms.vec.get(i);
                match optional_room {
                    None => panic!("Missing room at index {}!", i),
                    Some(next_room) => {
                        room = next_room;
                        println!("Entering {}", room)
                    },
                }
            },
        }
    }
    println!("Reached the end.");
}

