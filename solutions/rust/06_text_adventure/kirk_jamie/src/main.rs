use std::io;
mod person;
mod room;

use crate::room::{Room};
use crate::person::{Person};

fn go(direction: &str ) {
    match direction.to_lowercase().trim() {
            "north" => println!("You Went North"),
            "south" => println!("You Went South"),
            "west" => println!("You Went West"),
            "east" => println!("You Went East"),
            _ => println!("Cannot do that: {}", direction),
        }
}


fn main() {
    let mut name = String::new(); 
        println!("What is your name: ");
        io::stdin().read_line(&mut name)
            .ok()
            .expect("failed to read line");
    let player = Person::new(name, "North".to_string());
    println!("Hi {}", player.name);


    let mut dungeon = Vec::new();
    dungeon.push(Room::new("North".to_string(), vec!["West", "South"]));
    dungeon.push(Room::new("West".to_string(), vec!["North"]));
    dungeon.push(Room::new("East".to_string(), vec!["South"]));
    dungeon.push(Room::new("South".to_string(), vec!["East", "North"]));
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
            "go" => go(object),
            "quit" => break,
            _ => println!("Cannot do that: {}", object),
        }
    }
}
