type RenderResult = Result<(), &'static str>;

pub trait Render {
    fn render(&self) -> RenderResult;
}

pub struct Label {
    text: &'static str,
}

impl Label {
    pub fn new(text: &'static str) -> Label {
        Label { text: text }
    }
}

impl Render for Label {
    fn render(&self) -> RenderResult {
        println!("{}", self.text);
        Ok(()) // Return an empty Ok
    }
}

pub struct TextBox {
    heading: &'static str,
    value: &'static str,
    placeholder_text: &'static str,
}

impl TextBox {
    pub fn new(
        heading: &'static str,
        value: &'static str,
        placeholder_text: &'static str,
    ) -> TextBox {
        TextBox {
            heading: heading,
            value: value,
            placeholder_text: placeholder_text,
        }
    }
}

impl Render for TextBox {
    fn render(&self) -> RenderResult {
        println!(
            "{} - {} - {}",
            self.heading, self.placeholder_text, self.value
        );
        Ok(()) // Return an empty Ok
    }
}

pub struct Container<'a> {
    Contents: Vec<Box<dyn Render +'a>>,
    Name: &'static str   
}

impl<'a> Container<'a> {
    pub fn new(name: &'static str) -> Container {
        Container {
            Name: name,
            Contents: Vec::new()
        }
    }

    pub fn add(&mut self, renderable: impl Render + 'a) {
        let boxed = Box::new(renderable);
        self.Contents.push(boxed);
    }
}

impl Render for Container<'_> {
    fn render(&self) -> RenderResult {
        println!("{}", self.Name);
        for item in self.Contents.iter() {
            item.render()?;
        }
        
        Ok(())
    }
}