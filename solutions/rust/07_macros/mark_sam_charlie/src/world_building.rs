use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct World {
    pub locations: HashMap<String, Room>,
    pub player_location: String,
}

impl World {
    pub fn add_location(&mut self, location: Room) {
        self.locations.insert(location.id.clone(), location);
    }

    pub fn move_player(&mut self, direction: &String) -> Result<String, String> {
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

    pub fn get_location_description(&self) -> String {
        match self.locations.get(&self.player_location)
        {
            Some(room) => room.description.clone(),
            None => format!("No room called {}", &self.player_location),
        }            
    }

    pub fn get_player_room(&mut self) -> &mut Room {
        self.locations.get_mut(&self.player_location).unwrap()
    }
}

#[derive(Debug, new)]
pub struct Room {
    pub id: String,
    pub description: String,    
    #[new(default)]
    pub exits: HashMap<String, String>,
    #[new(default)]
    pub items: Vec<Item>,
}

impl<'a> Room {
    pub fn add_exit(&mut self, command: String, exit_id: String) {
        self.exits.insert(command, exit_id);
    }
    
    pub fn get_exits(&self) -> impl Iterator<Item = &String> {
        self.exits.keys()
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn take_item(&mut self, player: &mut Player, item_name: String) -> Result<String, String> {
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

#[derive(Default, Debug, new)]
pub struct Player {
    pub name: String,
    #[new(default)]
    pub inventory: Vec<Item>,
}

#[derive(Debug, new)]
pub struct Item {
    pub name: String,
}