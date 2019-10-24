pub struct Room<'a>{
    pub name: String,    
    pub doors: Vec<Door<'a>>,
}

impl<'a> Room<'a>{
    pub fn new(name: String, doors: Vec<Door<'a>>) -> Room <'a>{
        Room {
            name: name,
            doors: doors,
        }
    }
}

pub struct Door<'a>{
    pub name: &'a str,
    pub direction: &'a str
}

impl<'a> Door<'a>{
    pub fn new(name: &'a str, direction: &'a str) -> Door<'a> {
        Door {
            name: name,
            direction: direction,
        }
    }
}
