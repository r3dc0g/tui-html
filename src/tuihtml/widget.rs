use ratatui::widgets::Paragraph;

use crate::tuihtml::parser::construct_widget;

#[derive(Default)]
pub struct HtmlWidget<'a> {
    pub paragraph: Paragraph<'a>,
    pub links: Vec<String>,
    pub images: Vec<String>,
}

impl<'a> HtmlWidget<'a> {

    pub fn new(html: String) -> Self {
        construct_widget(html)
    }

    pub fn get_paragraph(&self) -> Paragraph<'_> {
        self.paragraph.clone()
    }

    pub fn get_links(&self) -> Vec<String> {
        self.links.clone()
    }

    pub fn get_images(&self) -> Vec<String> {
        self.images.clone()
    }
}
