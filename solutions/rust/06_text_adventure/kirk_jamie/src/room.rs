pub struct Room{
    pub name: String,    
    pub doors: Vec<'a&str>,
}

impl Room{
    pub fn new(name: String, doors: Vec<'a&str>) -> Room {
        Room {
            name: name,
            doors: doors,
        }
    }
}