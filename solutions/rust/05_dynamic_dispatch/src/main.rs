mod components;

use crate::components::{Label, Render, TextBox};

fn renderer(components: &Vec<Box<dyn Render>>) -> Result<(), String> {
	for component in components.iter() {
		component.render()?;
	}

	Ok(())
}

fn main() {
	let components: Vec<Box<dyn Render>> = vec![
		Box::new(Label::new("hello")),
		Box::new(Label::new("world!")),
		Box::new(TextBox::new("Some input", "input", "Please type something")),
	];

	renderer(&components).unwrap();
}
