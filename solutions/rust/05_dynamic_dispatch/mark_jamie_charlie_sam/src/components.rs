type RenderResult = Result<(), &'static str>;

pub trait Render {
    fn render(&self) -> RenderResult;
}

pub struct Label<'a> {
    text: &'a str,
}

impl<'a> Label<'a> {
    pub fn new(text: &'a str) -> Label {
        Label { text: text }
    }
}

impl Render for Label<'_> {
    fn render(&self) -> RenderResult {
        println!("{}", self.text);
        Ok(()) // Return an empty Ok
    }
}

pub struct TextBox<'a> {
    heading: &'a str,
    value: &'a str,
    placeholder_text: &'a str,
}

impl<'a> TextBox<'a> {
    pub fn new(heading: &'a str, value: &'a str, placeholder_text: &'a str) -> TextBox<'a> {
        TextBox {
            heading: heading,
            value: value,
            placeholder_text: placeholder_text,
        }
    }
}

impl Render for TextBox<'_> {
    fn render(&self) -> RenderResult {
        println!(
            "{} - {} - {}",
            self.heading, self.placeholder_text, self.value
        );
        Ok(()) // Return an empty Ok
    }
}

pub struct Container<'a> {
    contents: Vec<Box<dyn Render + 'a>>,
    name: &'a str,
}

impl<'a> Container<'a> {
    pub fn new(name: &'a str) -> Container {
        Container {
            name: name,
            contents: Vec::new(),
        }
    }

    pub fn add(&mut self, renderable: impl Render + 'a) {
        let boxed = Box::new(renderable);
        self.contents.push(boxed);
    }
}

impl Render for Container<'_> {
    fn render(&self) -> RenderResult {
        println!("{}", self.name);
        for item in self.contents.iter() {
            item.render()?;
        }

        Ok(())
    }
}
