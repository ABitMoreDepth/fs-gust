#[macro_use]
extern crate derive_new;

mod world_building;
use std::io::{self, BufRead};
use itertools::Itertools;
use crate::world_building::{World, Room, Item, Player};

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

    let player = Player::new("John Smith".to_string());    
    let mut done = false;
    let stdin = io::stdin();

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

    //let player_location = world.get_player_room();
    // let result = player_location.take_item(&mut player, "stick".to_string());
    // match result {
    //     Ok(output) => println!("{}", output),
    //     Err(output) => println!("{}", output),
    // }

    // println!("{}", world.get_location_description());

    // world.move_player(&"North".to_string()).unwrap();

    // println!("{}", world.get_location_description());

    // match world.move_player(&"North".to_string()) {
    //     Ok(move_msg) => println!("{}", move_msg),
    //     Err(err_msg) => println!("{}", err_msg),
    // }

    //let player_location = world.get_player_room();
    // let result = player_location.take_item(&mut player, "stone".to_string());
    // match result {
    //     Ok(output) => println!("{}", output),
    //     Err(output) => println!("{}", output),
    //}
}