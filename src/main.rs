
enum ContentType {
    Heading(String),
    Paragraph(String),
    Link(String, String), // URL, Text
}

struct HtmlElement {
    content_type: ContentType,
}

impl HtmlElement {
    fn new(content_type: ContentType) -> Self {
        HtmlElement { content_type }
    }

    fn render(&self) -> String {
        match &self.content_type {
            ContentType::Heading(text) => format!("<h1>{}</h1>", text),
            ContentType::Paragraph(text) => format!("<p>{}</p>", text),
            ContentType::Link(href, text) => format!("<a href='{}'>{}</a>", href, text),
        }
    }
}

struct HtmlPage {
    title: String,
    elements: Vec<HtmlElement>,
}

impl HtmlPage {
    fn new(title: String) -> Self {
        HtmlPage {
            title,
            elements: Vec::new(),
        }
    }

    fn add_element(&mut self, element: HtmlElement) {
        self.elements.push(element);
    }

    fn generate(&self) -> String {
        let mut html = String::from("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str(&format!("<title>{}</title>\n", self.title));
        html.push_str("</head>\n<body>\n");

        for element in &self.elements {
            html.push_str(&element.render());
            html.push('\n');
        }

        html.push_str("</body>\n</html>");
        html
    }
}


use std::fs::File;
use std::io::{self, Write, stdin, stdout};

fn read_user_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    let mut page = HtmlPage::new(read_user_input("Enter the title of your page:"));
    
    loop {
        println!("\nWhat would you like to add to your page?");
        println!("1: Heading");
        println!("2: Paragraph");
        println!("3: Link");
        println!("4: Generate Page and Exit");

        let choice = read_user_input("Choose an option (1-4):");

        match choice.as_str() {
            "1" => {
                let heading = read_user_input("Enter heading text:");
                page.add_element(HtmlElement::new(ContentType::Heading(heading)));
            },
            "2" => {
                let paragraph = read_user_input("Enter paragraph text:");
                page.add_element(HtmlElement::new(ContentType::Paragraph(paragraph)));
            },
            "3" => {
                let text = read_user_input("Enter link text:");
                let url = read_user_input("Enter link URL:");
                page.add_element(HtmlElement::new(ContentType::Link(url, text)));
            },
            "4" => break,
            _ => println!("Invalid choice, please enter a number between 1 and 4."),
        }
    }

    let html = page.generate();
    let mut file = File::create("interactive_output.html").expect("Could not create file");
    writeln!(file, "{}", html).expect("Could not write to file");

    println!("HTML page generated successfully.");
}