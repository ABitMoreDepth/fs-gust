mod components;

use crate::components::{Container, Label, Render, TextBox};

fn main() {
    let mut page = Container::new("Sample Page");

    page.add(Label::new("Hello"));
    page.add(Label::new("World"));
    page.add(TextBox::new("Some input", "input", "Please type something"));

    let mut sub_page = Container::new("Sample sub_page");

    sub_page.add(Label::new("This is neat!"));
    sub_page.add(Label::new("Wouldn't you say?"));
    sub_page.add(TextBox::new("Done now?", "input", "Enter yes to stop"));

    page.add(sub_page);

    page.render().unwrap();
}
