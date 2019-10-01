use std::sync::mpsc;
use std::thread;
use std::time::Duration;

struct Burger {
	id: u8,
}

fn producer(tx: mpsc::Sender<Burger>) {
	for count in 1..=10 {
		let burger = Burger { id: count };
		thread::sleep(Duration::from_secs(1));
		println!("Cooked burger {}.", burger.id);
		tx.send(burger).unwrap();
	}
	println!("No more burgers.");
}

fn waiter(rx: mpsc::Receiver<Burger>, ty: mpsc::Sender<Burger>) {
	for received in rx {
		println!("Got a burger");
		ty.send(received);
	}
}

fn consumer(rx: mpsc::Receiver<Burger>) {
	for received in rx {
		println!("I ate burger {}!", received.id);
	}
}

fn main() {
	let (producer_tx, prducer_rx) = mpsc::channel();
	let (waiter_tx, waiter_rx) = mpsc::channel();

	let thread_handle = thread::spawn(move || producer(producer_tx));
	let waiter_handle = thread::spawn(move || waiter(prducer_rx, waiter_tx));
	let consumer_handle = thread::spawn(move || consumer(waiter_rx));

	consumer_handle.join().unwrap();
	thread_handle.join().unwrap();
	waiter_handle.join().unwrap();
}
