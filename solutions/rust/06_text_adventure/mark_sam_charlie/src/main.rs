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
            },
            None => Err(format!("{} is not a valid direction", direction))
        }
    }

    fn get_location_description(&self) -> String {
        self.locations.get(&self.player_location).unwrap().description.clone()
    }
}

#[derive(Debug)]
struct Room{
    description: String,
    id: String,
    exits: HashMap<String, String>,
}

impl<'a> Room {
    fn new(id: String, description: String) -> Room {
        Room {
            description,
            id,
            exits: HashMap::new() 
        }
    }

    fn add_exit(&mut self, command: String, exit_id: String) {
        self.exits.insert(command, exit_id);
    }
    
    fn get_exits(&self) -> impl Iterator<Item = &String> {
        self.exits.values()
    }
}


#[derive(Default, Debug)]
struct Player {
    name: String,
}

impl Player {
    fn new(name: String) -> Player {
        Player {
            name: name
        }
    }
}

fn main() {
    let mut world = World::default();
    let mut room_a = Room::new("A".to_string(), "This is A".to_string());
    let room_b = Room::new("B".to_string(), "This is B".to_string());
    room_a.add_exit("North".to_string(), "B".to_string());
    world.add_location(room_a);
    world.add_location(room_b);

    let player = Player::new("John Smith".to_string());

    world.player_location = "A".to_string();

    println!("{}", world.get_location_description());

    world.move_player(&"North".to_string()).unwrap();

    println!("{}", world.get_location_description());

    match world.move_player(&"North".to_string()) {
        Ok(move_msg) => println!("{}", move_msg),
        Err(err_msg) => println!("{}", err_msg),
    }
}
