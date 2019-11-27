use std::collections::HashMap;

#[derive(Default, Debug)]
struct World {
    locations: HashMap<String, Room>,
    player_location: String,
}

impl World {
    fn add_location(&mut self, location: Room) {
        self.locations.insert(location.id.clone(), location);
    }

    fn move_player(&mut self, direction: &String) -> Result<String, String> {
        // Check whether our current location has an exit that matches direction.
        // If so, set the payers location to the pointed direction.
        // returns a result with Ok(new_location), Err(No Exit)

        let current_location = self.locations.get(&self.player_location).unwrap();

        match current_location.exits.get(direction) {
            Some(room_id) => {
                self.player_location = room_id.clone();
                Ok(format!("You have moved {}", direction))
            }
            None => Err(format!("{} is not a valid direction", direction)),
        }
    }

    fn get_location_description(&self) -> String {
        self.locations
            .get(&self.player_location)
            .unwrap()
            .description
            .clone()
    }

    fn get_player_room(&mut self) -> &mut Room {
        self.locations.get_mut(&self.player_location).unwrap()
    }
}

#[derive(Debug)]
struct Room {
    description: String,
    id: String,
    exits: HashMap<String, String>,
    items: Vec<Item>,
}

impl<'a> Room {
    fn new(id: String, description: String) -> Room {
        Room {
            description,
            id,
            exits: HashMap::new(),
            items: Vec::new(),
        }
    }

    fn add_exit(&mut self, command: String, exit_id: String) {
        self.exits.insert(command, exit_id);
    }

    fn get_exits(&self) -> impl Iterator<Item = &String> {
        self.exits.values()
    }

    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    fn take_item(&mut self, player: &mut Player, item_name: String) -> Result<String, String> {
        match self.items.iter().position(|i| i.name == item_name) {
            Some(index) => {
                let temp = self.items.remove(index);
                player.inventory.push(temp);
                Ok(format!("Picked up {}", item_name))
            }
            None => Err(format!("No item of type {} is present", item_name)),
        }
    }
}

#[derive(Default, Debug)]
struct Player {
    name: String,
    inventory: Vec<Item>,
}

impl Player {
    fn new(name: String) -> Player {
        Player {
            name: name,
            inventory: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Item {
    name: String,
}

impl Item {
    fn new(name: String) -> Item {
        Item { name: name }
    }
}

macro_rules! shaper_of_worlds {
    (
        location = $player:expr,
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
            world.player_location = $player.to_string();
            $(
                let mut room = Room::new($room_name.to_string(), $room_description.to_string());
                $(
                    let item = Item::new($item.to_string());
                    room.add_item(item);
                )*
                $(
                    room.add_exit($dir.to_string(), $dest.to_string());
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
                exits = ["North" => "B"]
            ]
            [
                "B",
                "This is B",
                items = [],
                exits = []
            ]
        ]
    );

    let mut player = Player::new("John Smith".to_string());
    // world.player_location = "A".to_string();

    let player_location = world.get_player_room();
    let result = player_location.take_item(&mut player, "stick".to_string());
    match result {
        Ok(output) => println!("{}", output),
        Err(output) => println!("{}", output),
    }

    println!("{}", world.get_location_description());

    world.move_player(&"North".to_string()).unwrap();

    println!("{}", world.get_location_description());

    match world.move_player(&"North".to_string()) {
        Ok(move_msg) => println!("{}", move_msg),
        Err(err_msg) => println!("{}", err_msg),
    }

    let player_location = world.get_player_room();
    let result = player_location.take_item(&mut player, "stone".to_string());
    match result {
        Ok(output) => println!("{}", output),
        Err(output) => println!("{}", output),
    }
}
