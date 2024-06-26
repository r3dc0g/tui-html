// Takes in RAW HTML and returns a representation as a list of ratatui widgets

use std::error::Error;
use ratatui::style::{Modifier, Style, Styled, Stylize};
use ratatui::widgets::List;
use ratatui::text::{Text, Line};
use html_parser::{Dom, Element, Node};

pub fn parse_html(html: &str) -> Result<List, Box<dyn Error>> {

    let dom = Dom::parse(html)?;
    let children = dom.children;

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


    let list = List::new(elements);

    Ok(list)
}

fn handle_listitems(children: Vec<Node>) -> Text<'static> {

    if children.is_empty() {
        return Text::from(" ");
    }

    let mut lines = vec![];

    for child in children.into_iter() {
        if let Some(element) = child.element() {
            if element.name == "li" {
                lines.push(Line::from(format!("• {}", element.name)));
            }
        }
    }

    let text = Text::from(lines.to_owned());
    text
}

fn handle_style(element: Element) -> Text<'static> {
    let text = match element.name.as_str() {
        "b" | "strong" => Text::from(element.children.get(0).clone().unwrap().text().unwrap_or(" ").to_string())
                            .style(Style::default().add_modifier(Modifier::BOLD)),

        "i" | "em" => Text::from(element.children.get(0).clone().unwrap().text().unwrap_or(" ").to_string())
                            .style(Style::default().add_modifier(Modifier::ITALIC)),

        "u" => Text::from(element.children.get(0).clone().unwrap().text().unwrap_or(" ").to_string())
                            .style(Style::default().add_modifier(Modifier::UNDERLINED)),

        "s" | "strike" => Text::from(element.children.get(0).clone().unwrap().text().unwrap_or(" ").to_string())
                            .style(Style::default().add_modifier(Modifier::CROSSED_OUT)),

        _ => Text::from(" ")
    };

    text
}

fn handle_element(element: Element) -> Text<'static> {

    let text = match element.name.as_str() {
        "p" => {
            if element.children.get(0).unwrap().text().is_some() {
                Text::from(element.children.get(0).clone().unwrap().text().unwrap_or(" ").to_string())
            }
            else {
                handle_style(element)
            }
        },

        "img" => Text::from("Image").style(Style::default().bold()),

        "ul" => handle_listitems(element.clone().children),

        "a" => {
            if element.children.get(0).unwrap().text().is_some() {
                Text::from(element.children.get(0).clone().unwrap().text().unwrap_or(" ").to_string())
            }
            else {
                handle_style(element)
            }
        }

        "h1" | "h2" | "h3" | "h4" => {
            if element.children.get(0).unwrap().text().is_some() {
                Text::from(element.children.get(0).clone().unwrap().text().unwrap_or(" ").to_string())
                    .style(Style::default().add_modifier(Modifier::BOLD))
            }
            else {
                handle_style(element)
            }

        }

        "br" => Text::from("\n"),

        _ => Text::from(" ")
    };

    text

}


