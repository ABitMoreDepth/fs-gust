pub struct Person{
    pub name: String,
    pub current_room: String
}

impl Person{
    pub fn new(name: String, room: String) -> Person {
        Person {
            name: name,
            current_room: room,
        }
    }
}