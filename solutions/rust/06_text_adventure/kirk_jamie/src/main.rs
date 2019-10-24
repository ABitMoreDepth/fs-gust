use std::io;
mod person;
mod room;

use crate::room::{Room,Door};
use crate::person::{Person};


fn go(direction: &str, player: &Person, dungeon: &Vec<Room> ) {
    // match direction.to_lowercase().trim() {
    //         "north" => change_room(direction),
    //         "south" => println!("You Went South"),
    //         "west" => println!("You Went West"),
    //         "east" => println!("You Went East"),
    //         _ => println!("Cannot do that: {}", direction),
    //     }s
    let mut rooms = IntoIterator::into_iter(dungeon);
    let room = rooms.find(|&room| room.name == player.current_room).unwrap();
    let mut destinations = IntoIterator::into_iter(room.doors);
    let destination = destinations.find(move |&door_dir| door_dir.direction == direction).unwrap();

    println!("{}",room.doors[0].name);
    println!("{}", destination.direction);

}


fn main() {
    let mut name = String::new(); 
        println!("What is your name: ");
        io::stdin().read_line(&mut name)
            .ok()
            .expect("failed to read line");
    let mut player = Person::new(name, "North".to_string());
    println!("Hi {}", player.name);

    let mut dungeon = Vec::new();
    dungeon.push(Room::new("North".to_string(), vec![Door::new("West","West"), Door::new("South","South")]));
    dungeon.push(Room::new("West".to_string(), vec![Door::new("North","North")]));
    dungeon.push(Room::new("East".to_string(), vec![Door::new("South","South")]));
    dungeon.push(Room::new("South".to_string(), vec![Door::new("East","East"), Door::new("North","North")]));
    loop {
        let mut command = String::new(); 
        println!("Enter your choice");
        io::stdin().read_line(&mut command)
            .ok()
            .expect("failed to read line");

        let commands:Vec<&str> = command.split_whitespace().collect();
        let action = commands.get(0).unwrap_or(&"").to_lowercase();
        let object = commands.get(1).unwrap_or(&"");

        match action.trim() {
            "go" => go(object,&player, &dungeon),
            "quit" => break,
            _ => println!("Cannot do that: {}", object),
        }
    }
}
