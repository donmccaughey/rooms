mod rooms;


fn main() {
  let rooms = rooms::Rooms::build();
  println!("There are {} rooms to explore.", rooms.len());

  for room in rooms.room_refs.iter() {
    println!("  {}", *room.borrow());
  }

  println!("Walking north:");
  let mut weak_room_ref = rooms.main_room_ref();
  loop {
    let room_ref = weak_room_ref.upgrade().unwrap();
    println!("  In {}", *room_ref.borrow());
    match room_ref.borrow().north {
      None => {
        println!("No more rooms!");
        break;
      },
      Some(ref north_room_ref) => weak_room_ref = north_room_ref.clone(),
    }
  }

  let northmost_room_ref = match weak_room_ref.upgrade() {
    None => panic!("Room has been deallocated"),
    Some(room_ref) => room_ref,
  };
  println!("Walking south from {}:", *northmost_room_ref.borrow());
  loop {
    let room_ref = weak_room_ref.upgrade().unwrap();
    println!("  In {}", *room_ref.borrow());
    match room_ref.borrow().south {
      None => {
        println!("No more rooms!");
        break;
      },
      Some(ref south_room_ref) => weak_room_ref = south_room_ref.clone(),
    }
  }
}

