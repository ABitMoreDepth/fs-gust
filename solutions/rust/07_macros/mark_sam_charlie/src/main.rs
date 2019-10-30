#[macro_use]
extern crate derive_new;

mod world_building;
use std::io::{self};
use itertools::Itertools;
use crate::world_building::{World, Room, Item};

macro_rules! shaper_of_worlds { 
    (
        location = $player_location:expr,
        rooms = [
            $([
                $room_name:expr,
                $room_description:expr, 
                items=[$($item:expr$(,)*)*],
                exits=[$($dir:expr => $dest:expr)*]   
            ])+
        ]
    ) => {
        {
            let mut world = World::default();
            world.player_location = $player_location.to_lowercase().to_string();
            $(
                let mut room = Room::new($room_name.to_lowercase().to_string(), $room_description.to_string());
                $(
                    let item = Item::new($item.to_string());
                    room.add_item(item);
                )*
                $(
                    room.add_exit($dir.to_string(), $dest.to_lowercase().to_string());
                )*
                world.add_location(room);
            )+
            world
        }
    };
}

fn main() {
    let mut world = shaper_of_worlds!(
        location = "A",
        rooms = [
            [
                "A",
                "This is A",
                items = ["Stick", "Stone"],
                exits = ["north" => "B" "west" => "c"]
            ]
            [
                "B",
                "This is B",
                items = [],
                exits = ["south" => "A"]
            ]
            [
                "C",
                "This is C",
                items = [],
                exits = ["east" => "A"]
            ]
        ]
    );
     
    let mut done = false;    

    println!("Welcome");

    while !done {        
        println!("{}",  world.get_location_description());
        let player_room = world.get_player_room();
        let exits = player_room.get_exits().join(", ");
        println!("Exits are {}", exits);

        let mut line = String::new();        
        match io::stdin().read_line(&mut line) {            
            Err(error) => {
                println!("error: {}", error);
                break;
            },
            _ => { }
        }
        
        let cased = line.to_lowercase().trim().to_string();

        match cased.as_ref() {
            "exit" => done = true,
            _ => match world.move_player(&cased) {
                Ok(move_msg) => println!("{}", move_msg),
                Err(err_msg) => println!("{}", err_msg),
            }
        }
    }
}