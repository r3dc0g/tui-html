use ratatui::widgets::Paragraph;

use crate::tuihtml::parser::construct_widget;

pub struct HtmlWidget<'a> {
    paragraph: Paragraph<'a>,
    links: Vec<String>,
}

impl HtmlWidget<'_> {
    pub fn new(html: String) -> Self {
        let (paragraph, links) = construct_widget(html);
        Self {
            paragraph,
            links
        }
    }

    pub fn get_paragraph(&self) -> Paragraph<'_> {
        self.paragraph.clone()
    }

    pub fn get_links(&self) -> Vec<String> {
        self.links.clone()
    }
}
