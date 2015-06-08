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
        match room.north() {
            None => {
                println!("No door to the north");
                break;
            },
            Some(next_room) => {
                room = next_room;
                println!("  Entering {}", room);
            },
        }
    }
    println!("Reached the end, turning around.");

    println!("Walking south:");
    loop {
        match room.south() {
            None => {
                println!("No door to the south");
                break;
            },
            Some(next_room) => {
                room = next_room;
                println!("  Entering {}", room);
            },
        }
    }
    println!("Done!");
}

