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
    let mut world = let_there_be_light();

    println!("Welcome");

    loop {        
        println!();
        println!("{}", world.get_location_description());
        let player_room = world.get_player_room();
        let exits = player_room.get_exits().join(", ");
        println!("Exits are {}", exits);

        let mut user_input = String::new();        
        match io::stdin().read_line(&mut user_input) {            
            Err(error) => {
                println!("error: {}", error);
                break;
            },
            _ => { }
        }
        
        match perform_action(&mut world, &user_input) {
            Err(err) => {
                println!("{}",err );
                break;
            },
            Ok(output) => println!("{}", output)
        }
    }
}

fn let_there_be_light() -> World {
    shaper_of_worlds!(
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
    )     
}

fn perform_action(world:&mut World, user_intput:&str) -> Result<String, String> {
    let cased = user_intput.to_lowercase().trim().to_string();
    match cased.as_ref() {
        "exit" => Err("Exiting".to_string()),
        _ => world.move_player(&cased)
        }
}