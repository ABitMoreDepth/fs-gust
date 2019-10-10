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