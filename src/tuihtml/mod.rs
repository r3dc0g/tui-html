// Takes in RAW HTML and returns a representation as a list of ratatui widgets

use std::error::Error;
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::widgets::List;
use ratatui::text::{Text, Line, Span};
use html_parser::{Dom, Element, Node};

fn handle_listitems(children: Vec<Node>) -> Text<'static> {

    if children.is_empty() {
        return Text::from(" ");
    }

    let mut lines = vec![];

    for child in children.into_iter() {
        if let Some(element) = child.element() {
            if element.name == "li" {
                lines.push(Line::from(handle_children_children(element.children.clone())));
            }
        }
    }

    let text = Text::from(lines.to_owned());
    text
}

fn handle_style(element: Element) -> Span<'static> {

    let text = match element.name.as_str() {
        "b" | "strong" =>{
            Span::from(element.children.get(0).clone().unwrap().text().unwrap_or(" ").to_string())
                .style(Style::default().add_modifier(Modifier::BOLD))
        }

        "i" | "em" => Span::from(element.children.get(0).clone().unwrap().text().unwrap_or(" ").to_string())
                            .style(Style::default().add_modifier(Modifier::ITALIC)),

        "u" => Span::from(element.children.get(0).clone().unwrap().text().unwrap_or(" ").to_string())
                            .style(Style::default().add_modifier(Modifier::UNDERLINED)),

        "s" | "strike" => Span::from(element.children.get(0).clone().unwrap().text().unwrap_or(" ").to_string())
                            .style(Style::default().add_modifier(Modifier::CROSSED_OUT)),

        "a" => Span::from(element.children.get(0).clone().unwrap().text().unwrap_or(" ").to_string())
                .style(Style::default().add_modifier(Modifier::UNDERLINED)),

        _ => Span::from(" ")
    };

    text
}

fn handle_children_children(children: Vec<Node>) -> Vec<Span<'static>> {

    let mut elements: Vec<Span> = vec![];

    for child in children.into_iter() {
        if let Some(element) = child.element() {
            elements.push(Span::from(handle_style(element.clone())));
        }
        else {
            elements.push(Span::from(child.text().unwrap_or(" ").to_string()));
        }
    }

    elements
}


fn handle_element(element: Element) -> Text<'static> {

    if element.children.is_empty() {
        return Text::from(" ");
    }


    let text = match element.name.as_str() {
        "p" => {

            if element.children.len() > 1 {
                return Text::from(Line::from(handle_children_children(element.children)));
            }

            if element.children.get(0).unwrap().text().is_some() {
                Text::from(element.children.get(0).clone().unwrap().text().unwrap_or(" ").to_string())
            }
            else {
                Text::from(Line::from(handle_children_children(element.children)))
            }
        },

        "img" => Text::from("Image").style(Style::default().bold()),

        "ul" => handle_listitems(element.clone().children),

        "h1" | "h2" | "h3" | "h4" => {
            if element.children.get(0).unwrap().text().is_some() {
                Text::from(element.children.get(0).clone().unwrap().text().unwrap_or(" ").to_string())
                    .style(Style::default().add_modifier(Modifier::BOLD))
            }
            else {
                Text::from(Line::from(handle_children_children(element.children)))
            }

        }

        "br" => Text::from("\n"),

        "a" | "b" | "strong" | "i" | "em" | "u" | "s" | "strike" => Text::from(handle_style(element)),

        _ => {

            if element.children.len() > 1 {
                return Text::from(Line::from(handle_children_children(element.children)));
            }

            Text::from(" ")
        }
    };

    text

}

fn handle_children(children: Vec<Node>) -> Vec<Text<'static>> {

    let mut elements: Vec<Text> = vec![];

    for child in children.into_iter() {
        match child {
            text if child.text().is_some()  => {
                elements.push(Text::from(text.text().unwrap().to_string()));
            },

            element if child.element().is_some() => {
                elements.push(handle_element(element.element().unwrap().clone()));
            },

            comment if child.comment().is_some() => {
                elements.push(Text::from(comment.comment().unwrap().to_string()));
            },

            _ => {
                elements.push(Text::from("\n"));
            }
        };

    }

    elements
}

pub fn parse_html(html: &str) -> Result<List, Box<dyn Error>> {

    let dom = Dom::parse(html)?;
    let children = dom.children;

    let elements = handle_children(children);

    let list = List::new(elements);

    Ok(list)
}


