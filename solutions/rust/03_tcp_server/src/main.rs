use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

struct Client {
    name: String,
    stream: TcpStream,
}

impl Client {
    fn new(mut stream: TcpStream) -> Client {
        let mut client_message = String::new();
        println!("Received a new message: {}", client_message);
        stream.read_to_string(&mut client_message).unwrap();
        let client_name: &str = client_message
            .split(" ")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .to_owned();

        Client {
            stream: stream,
            name: client_name.to_string(),
        }
    }

    fn acknowledge(&mut self) -> io::Result<()> {
        let message = format!("Hello, {}", self.name);
        self.stream.write_all(message.as_bytes())
    }
}

// Would be used to hold pairs of clients,
// expand to store game state etc.?
struct Pairing<'a> {
    first_client: &'a Client,
    second_client: &'a Client,
}

fn main() -> io::Result<()> {
    let mut clients: Vec<Client> = Vec::new();

    println!("Binding to port 1234");
    let listener = TcpListener::bind("0.0.0.0:1234")?;

    for stream_result in listener.incoming() {
        println!("Received a new connection!");

        let stream = stream_result.unwrap();
        let mut new_client = Client::new(stream);
        new_client.acknowledge()?;
        clients.push(new_client);
    }
    Ok(())
}
