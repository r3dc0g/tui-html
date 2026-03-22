use std::collections::HashMap;
use insta::assert_snapshot;
use ratatui::{Terminal, backend::TestBackend, style::{Modifier, Stylize}, text::{Line, Span}, widgets::Paragraph};

use crate::tuihtml::{html::*, tokenizer::{HTMLTokenizer, Token}};

#[derive(Default,Debug, Clone, PartialEq)]
enum ProcessState {
    #[default]
    Text,
    Style,
    Heading,
    Image,
    OrderedList,
    UnorderedList,
    ListItem,
}

#[derive(Debug)]
struct ProcessStateMap {
    map: HashMap<HTMLTag, ProcessState>
}

impl ProcessStateMap {
    fn new() -> Self {
        ProcessStateMap {
            map: HashMap::from([
                (HTMLTag::P, ProcessState::Text),
                (HTMLTag::HEAD, ProcessState::Text),
                (HTMLTag::BODY, ProcessState::Text),
                (HTMLTag::TITLE, ProcessState::Style),
                (HTMLTag::META, ProcessState::Style),
                (HTMLTag::LINK, ProcessState::Style),
                (HTMLTag::STYLE, ProcessState::Style),
                (HTMLTag::DIV, ProcessState::Style),
                (HTMLTag::SPAN, ProcessState::Style),
                (HTMLTag::H1, ProcessState::Heading),
                (HTMLTag::H2, ProcessState::Heading),
                (HTMLTag::H3, ProcessState::Heading),
                (HTMLTag::H4, ProcessState::Heading),
                (HTMLTag::H5, ProcessState::Heading),
                (HTMLTag::H6, ProcessState::Heading),
                (HTMLTag::P, ProcessState::Text),
                (HTMLTag::A, ProcessState::Style),
                (HTMLTag::BOLD, ProcessState::Style),
                (HTMLTag::BR, ProcessState::Text),
                (HTMLTag::HR, ProcessState::Text),
                (HTMLTag::IMG, ProcessState::Image),
                (HTMLTag::LABEL, ProcessState::Text),
                (HTMLTag::TABLE, ProcessState::Text),
                (HTMLTag::TR, ProcessState::Text),
                (HTMLTag::TH, ProcessState::Text),
                (HTMLTag::TD, ProcessState::Text),
                (HTMLTag::UL, ProcessState::UnorderedList),
                (HTMLTag::OL, ProcessState::OrderedList),
                (HTMLTag::LI, ProcessState::ListItem),
                (HTMLTag::TEXTAREA, ProcessState::Text),
                (HTMLTag::SOURCE, ProcessState::Text),
                (HTMLTag::NAV, ProcessState::Style),
                (HTMLTag::HEADER, ProcessState::Style),
                (HTMLTag::FOOTER, ProcessState::Style),
                (HTMLTag::SECTION, ProcessState::Style),
                (HTMLTag::ARTICLE, ProcessState::Text),
                (HTMLTag::ASIDE, ProcessState::Text),
                (HTMLTag::MAIN, ProcessState::Style),
                (HTMLTag::FIGURE, ProcessState::Image),
                (HTMLTag::FIGCAPTION, ProcessState::Text),
                (HTMLTag::STRONG, ProcessState::Style),
                (HTMLTag::EM, ProcessState::Style),
                (HTMLTag::CODE, ProcessState::Style),
                (HTMLTag::PRE, ProcessState::Text),
                (HTMLTag::BLOCKQUOTE, ProcessState::Text),
                (HTMLTag::CITE, ProcessState::Text),
                (HTMLTag::ABBR, ProcessState::Text),
                (HTMLTag::TIME, ProcessState::Text),
                (HTMLTag::DATA, ProcessState::Text),
                (HTMLTag::METER, ProcessState::Text),
                (HTMLTag::DETAILS, ProcessState::Text),
                (HTMLTag::SUMMARY, ProcessState::Text),
                (HTMLTag::DIALOG, ProcessState::Text),
                (HTMLTag::CANVAS, ProcessState::Image),
                (HTMLTag::SVG, ProcessState::Image),
                (HTMLTag::MATH, ProcessState::Text),
                (HTMLTag::SMALL, ProcessState::Style),
                (HTMLTag::AREA, ProcessState::Style),
                (HTMLTag::COL, ProcessState::Text),
                (HTMLTag::COLGROUP, ProcessState::Text),
                (HTMLTag::CAPTION, ProcessState::Text),
            ])
        }
    }

    fn get_state(&self, tag: HTMLTag) -> ProcessState {
        return self.map.get(&tag).unwrap_or(&ProcessState::Text).clone();
    }
}

fn parse_html(html: String) -> Vec<Token> {

    let mut tokens = Vec::new();
    let mut tokenizer = HTMLTokenizer::new(html.chars());
    tokenizer.init();

    let mut current_token = tokenizer.next();
    tokens.push(current_token.clone());


    while current_token != Token::EOF {
        current_token = tokenizer.next();
        tokens.push(current_token.clone());
    }

    tokens
}

fn apply_style<'a>(element: HTMLElement, text: String) -> Span<'a> {
    match element.tag {
        HTMLTag::BOLD | HTMLTag::H1 | HTMLTag::H2 | HTMLTag::H3 | HTMLTag::H4 | HTMLTag::H5 | HTMLTag::H6 => {
            return Span::from(text).add_modifier(Modifier::BOLD);
        },
        HTMLTag::EM => {
            return Span::from(text).add_modifier(Modifier::ITALIC);
        },
        HTMLTag::A => {
            return Span::from(text).add_modifier(Modifier::UNDERLINED);
        }
        _ => {
            return Span::from(text);
        }
    }
}

pub fn construct_widget<'a>(html: String) -> Paragraph<'a> {

    let tokens = parse_html(html);
    let mut lines: Vec<Line<'a>> = Vec::new();
    let mut spans: Vec<Span<'a>> = Vec::new();
    let state_map = ProcessStateMap::new();
    let mut state_stack: Vec<(HTMLElement, ProcessState)> = Vec::new();
    let mut list_indent = 0;
    let mut list_enumerance = 1;
    let mut is_ordered = false;
    let mut is_unordered = false;

    for token in tokens {
        match token {
            Token::Element(element) => {
                if element.closing {
                    if let Some(state) = state_stack.last() {
                        let process_state = state.1.clone();
                        match process_state {
                            ProcessState::Heading => {
                                if !spans.is_empty() {
                                    lines.push(Line::from(spans.clone()));
                                    spans.clear();
                                }
                            },
                            _ => {},
                        }
                    }

                }
                else {
                    state_stack.push((element.clone(), state_map.get_state(element.tag.clone())));
                    if element.tag == HTMLTag::OL {
                        is_ordered = true;
                        list_indent += 1;
                    }
                    if element.tag == HTMLTag::UL {
                        is_unordered = true;
                        list_indent += 1;
                    }
                }
            },
            Token::Text(text) => {
                if let Some(state) = state_stack.last() {
                    let process_state = state.1.clone();
                    match process_state {
                        ProcessState::Heading => {
                            spans.push(apply_style(state.0.clone(), text));
                        },
                        ProcessState::Style => {
                            spans.push(apply_style(state.0.clone(), text));
                        },
                        ProcessState::Text => {
                            spans.push(Span::from(text));
                        },
                        ProcessState::Image => {
                            // Need a separate Image cache to render the image
                            todo!();
                        },
                        ProcessState::ListItem => {
                            let indent = String::from("\t".repeat(list_indent));
                            if is_ordered {
                                spans.push(Span::from(format!("{}{}. {}\n", indent, list_enumerance, text)));
                                list_enumerance += 1;
                            }
                            if is_unordered {
                                spans.push(Span::from(format!("{}• {}\n", indent, text)));
                            }
                        },
                        _ => {}
                    }
                }
                else {
                    spans.push(Span::from(text));
                }
            },
            Token::NewLine => {
                if !spans.is_empty() {
                    lines.push(Line::from(spans.clone()));
                    spans.clear();
                }
            }
            Token::EOF => {
                state_stack.pop();
                if !spans.is_empty() {
                    lines.push(Line::from(spans.clone()));
                    spans.clear();
                }
            }
        }
    }

    Paragraph::new(lines)
}


#[test]
fn parse_html_returns_only_text() {

    let html = r#"
Title
Hello World
Google"#;

    let tokens = parse_html(html.into());

    assert_eq!(tokens, Vec::from([
            Token::Text("Title\nHello World\nGoogle".into()),
            Token::EOF
        ]))
}

#[test]
fn parse_html_returns_full_html_dom() {

    let html = r#"
<html>
    <body>
        <h1>Title</h1>
        <p><b>Hello</b> World</p>
        <a href="https://www.google.com">Google</a>
        <img src="/home/garrett/Documents/image.jpg"></img>
    </body>
</html>
    "#;


    let tokens = parse_html(html.into());

    assert_eq!(tokens, Vec::from([
        Token::Element(HTMLElement { tag: HTMLTag::HTML, attributes: HashMap::new(), closing: false }),
        Token::Element(HTMLElement { tag: HTMLTag::BODY, attributes: HashMap::new(), closing: false }),
        Token::Element(HTMLElement { tag: HTMLTag::H1, attributes: HashMap::new(), closing: false }),
        Token::Text("Title".into()),
        Token::Element(HTMLElement { tag: HTMLTag::H1, attributes: HashMap::new(), closing: true }),
        Token::Element(HTMLElement { tag: HTMLTag::P, attributes: HashMap::new(), closing: false }),
        Token::Element(HTMLElement { tag: HTMLTag::BOLD, attributes: HashMap::new(), closing: false }),
        Token::Text("Hello".into()),
        Token::Element(HTMLElement { tag: HTMLTag::BOLD, attributes: HashMap::new(), closing: true }),
        Token::Text(" World".into()),
        Token::Element(HTMLElement { tag: HTMLTag::P, attributes: HashMap::new(), closing: true }),
        Token::Element(HTMLElement { tag: HTMLTag::A, attributes: HashMap::from([("href".into(), "https://www.google.com".into()); 1]), closing: false}),
        Token::Text("Google".into()),
        Token::Element(HTMLElement { tag: HTMLTag::A, attributes: HashMap::new(), closing: true }),
        Token::Element(HTMLElement { tag: HTMLTag::IMG, attributes: HashMap::from([("src".into(), "/home/garrett/Documents/image.jpg".into()); 1]), closing: false }),
        Token::Element(HTMLElement { tag: HTMLTag::IMG, attributes: HashMap::new(), closing: true }),
        Token::Element(HTMLElement { tag: HTMLTag::BODY, attributes: HashMap::new(), closing: true }),
        Token::Element(HTMLElement { tag: HTMLTag::HTML, attributes: HashMap::new(), closing: true }),
        Token::EOF
    ]))
}

#[test]
fn parse_html_returns_partial_html_elements() {

    let html = r#"
        <h1>Title</h1>
        <p><b>Hello</b> World</p>
        <a href="https://www.google.com">Google</a>
        <img src="/home/garrett/Documents/image.jpg"></img>
    "#;


    let tokens = parse_html(html.into());

    assert_eq!(tokens, Vec::from([
        Token::Element(HTMLElement { tag: HTMLTag::H1, attributes: HashMap::new(), closing: false }),
        Token::Text("Title".into()),
        Token::Element(HTMLElement { tag: HTMLTag::H1, attributes: HashMap::new(), closing: true }),
        Token::Element(HTMLElement { tag: HTMLTag::P, attributes: HashMap::new(), closing: false }),
        Token::Element(HTMLElement { tag: HTMLTag::BOLD, attributes: HashMap::new(), closing: false }),
        Token::Text("Hello".into()),
        Token::Element(HTMLElement { tag: HTMLTag::BOLD, attributes: HashMap::new(), closing: true }),
        Token::Text(" World".into()),
        Token::Element(HTMLElement { tag: HTMLTag::P, attributes: HashMap::new(), closing: true }),
        Token::Element(HTMLElement { tag: HTMLTag::A, attributes: HashMap::from([("href".into(), "https://www.google.com".into()); 1]), closing: false}),
        Token::Text("Google".into()),
        Token::Element(HTMLElement { tag: HTMLTag::A, attributes: HashMap::new(), closing: true }),
        Token::Element(HTMLElement { tag: HTMLTag::IMG, attributes: HashMap::from([("src".into(), "/home/garrett/Documents/image.jpg".into()); 1]), closing: false }),
        Token::Element(HTMLElement { tag: HTMLTag::IMG, attributes: HashMap::new(), closing: true }),
        Token::EOF
    ]))
}

#[test]
fn process_state_map_correctly_maps() {
    let map = ProcessStateMap::new();
    let mut stack = Vec::new();
    let mut elements = Vec::new();

    let html = r#"
        <h1>Title</h1>
        <p><b>Hello</b> World</p>
        <a href="https://www.google.com">Google</a>
        <img src="/home/garrett/Documents/image.jpg"></img>
        <ul>
          <li> List Item 1 </li>
          <li> List Item 2 </li>
          <li> List Item 3 </li>
        </ul>
    "#;

    let tokens = parse_html(html.into());

    for token in tokens {

        println!("{:?}", token);

        if let Some(element) = Token::get_html_element(token) {
            let state = map.get_state(element.clone().tag).clone();
            // println!("Current Process State: {:?}", state);

            if stack.last().is_none() {
                stack.push(state.clone());
                elements.push(element.clone());
            }
            else if stack.last().unwrap().clone() != state {
                stack.push(state.clone());
                elements.push(element.clone());
            }
            else {
                if stack.last().unwrap().clone() == ProcessState::Style {
                    stack.pop();
                    // println!("No longer Styling");
                }
                else {
                    stack.pop();
                    for _element in elements.clone().into_iter() {
                        // println!("{:?} ", element);
                    }
                    elements.clear();
                    // println!("New Line");

                }
            }
        }
    }
}

#[test]
fn test_render() {
    let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
    terminal
        .draw(|frame| {

            let html = r#"
<h1>Title</h1>
<p>
    <b>Hello</b> World
</p>
<a href="https://www.google.com">Google</a>
<img src="/home/garrett/Documents/image.jpg"></img>"#;

            frame.render_widget(construct_widget(String::from(html)), frame.area());
        })
        .unwrap();
    assert_snapshot!(terminal.backend());
}
