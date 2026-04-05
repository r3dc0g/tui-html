use ratatui::{style::{Modifier, Style, Stylize}, text::{Line, Span}, widgets::{Paragraph, Wrap}};
use crate::tuihtml::{html::*, tokenizer::{HTMLTokenizer, Token}};

#[derive(Default,Debug, Clone, PartialEq)]
pub enum ListState {
    #[default]
    ORDERED,
    UNORDERED
}

#[derive(Default,Debug, Clone, PartialEq)]
pub struct StyleContext {
    list_state: Vec<Option<ListState>>,
    list_index: Vec<usize>,
    link_list: Vec<String>,
    link_index: usize,
    img_list: Vec<String>,
    img_index: usize,
    active_modifiers: Vec<Modifier>,
    active_styles: Vec<Style>,
}

impl StyleContext {
    pub fn new() -> Self {
        Self {
            list_state: Vec::new(),
            list_index: Vec::new(),
            link_list: Vec::new(),
            link_index: 0,
            img_list: Vec::new(),
            img_index: 0,
            active_modifiers: Vec::new(),
            active_styles: Vec::new(),
        }
    }

    pub fn remove_modifiers(&mut self, tag: HTMLTag) {
        let modifiers = tag.to_modifers();
        for modifier in modifiers {
            match self.active_modifiers.iter().position(|m| *m == modifier) {
                Some(i) => {
                    self.active_modifiers.remove(i);
                },
                _ => {}
            }

        }
    }

    pub fn add_modifiers(&mut self, tag: HTMLTag) {
        for modifier in tag.to_modifers() {
            self.active_modifiers.push(modifier);
        }
    }

    pub fn construct_span<'a>(&self, text: String) -> Span<'a> {
        let mut span = Span::from(text);

        for modifier in &self.active_modifiers {
            span.style = span.style.add_modifier(*modifier);
        }

        for style in &self.active_styles {
            span.style = span.style.patch(*style);
        }

        span
    }
}

pub fn get_html_style<'a>(tag: &HTMLTag, spans: &mut Vec<Span<'a>>, context: &StyleContext) -> Vec<Line<'a>> {
    const HR_WIDTH: usize = 120;

    match tag {
        HTMLTag::H1 => {
            let spans_width: usize = spans.iter().map(|span| span.width()).collect::<Vec<usize>>().iter().sum();
            let header_spacing = vec![Span::from(" ".repeat(spans_width / 2))];
            let header_line = Line::from(vec![
                header_spacing.clone(),
                spans.clone(),
                header_spacing
            ].concat());
            let overscore_line = Line::from(Span::from("\u{00AF}".repeat(spans_width * 2)));

            spans.clear();
            vec![header_line.centered(), overscore_line.centered()]
        },
        HTMLTag::HR => {
            let overscore_line = Line::from(Span::from("\u{00AF}".repeat(HR_WIDTH)));
            let underscore_line = Line::from(Span::from("\u{005F}".repeat(HR_WIDTH)));

            vec![underscore_line.centered(), overscore_line.centered()]
        },
        HTMLTag::LI => {
            match context.list_state.last() {
                Some(last_state) => {
                    match last_state {
                        Some(state) => {
                            match state {
                                ListState::ORDERED => {
                                    let list_item = Line::from(
                                        vec![
                                            vec![Span::from(format!(
                                                "{}{}. ",
                                                " ".repeat(2 * context.list_state.len()),
                                                context.list_index.last()
                                                                  .unwrap_or(&usize::from(0 as usize))
                                                                  .clone()
                                            ))],
                                            spans.clone()
                                        ].concat()
                                    );
                                    spans.clear();
                                    vec![list_item]
                                },
                                ListState::UNORDERED => {
                                    let list_item = Line::from(
                                        vec![
                                            vec![Span::from(format!(
                                                "{}\u{2022} ",
                                                " ".repeat(2 * context.list_state.len()),
                                            ))],
                                            spans.clone()
                                        ].concat()
                                    );
                                    spans.clear();
                                    vec![list_item]
                                }
                            }
                        },
                        None => {
                            Vec::new()
                        }
                    }
                },
                None => {
                    Vec::new()
                }
            }
        },
        HTMLTag::A => {
            spans.push(Span::from(format!("[{}]", context.link_index)).bold());
            Vec::new()
        },
        HTMLTag::H2 => {
            let spans_width: usize = spans.iter().map(|span| span.width()).collect::<Vec<usize>>().iter().sum();
            let above_line = Line::from(Span::from("\u{00A0}".repeat(spans_width)));
            let header_line = Line::from(spans.clone());

            spans.clear();
            vec![above_line, header_line.underlined() ]
        }
        HTMLTag::H3 |
        HTMLTag::H4 |
        HTMLTag::H5 |
        HTMLTag::H6 => {
            let spans_width: usize = spans.iter().map(|span| span.width()).collect::<Vec<usize>>().iter().sum();
            let above_line = Line::from(Span::from("\u{00A0}".repeat(spans_width)));
            let header_line = Line::from(spans.clone());

            spans.clear();
            vec![above_line, header_line]
        },
        HTMLTag::NAV |
        HTMLTag::DIV |
        HTMLTag::OL |
        HTMLTag::UL |
        HTMLTag::P => {
            match spans.is_empty() {
                true => {
                    Vec::new()
                },
                false => {
                    let line = Line::from(spans.clone());
                    spans.clear();
                    vec![line]
                }
            }
        },
        _ => Vec::new()
    }
}

pub fn is_self_closing(tag: &HTMLTag) -> bool {
    match tag {
        HTMLTag::HR => true,
        HTMLTag::BR => true,
        _ => false
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

pub fn construct_widget<'a>(html: String) -> Paragraph<'a> {

    let tokens = parse_html(html);
    let mut lines: Vec<Line<'a>> = Vec::new();
    let mut spans: Vec<Span<'a>> = Vec::new();
    let mut style_context = StyleContext::new();
    let mut element_stack: Vec<HTMLElement> = Vec::new();

    for token in tokens {
        // println!("{:?}", token);
        match token {
            Token::Element(element) => {
                match &element.closing {
                    true => {
                        if let Some(removed_element) = element_stack.pop() {
                            lines = vec![
                                lines,
                                get_html_style(&removed_element.tag, &mut spans, &style_context)
                            ].concat();

                            style_context.remove_modifiers(removed_element.tag);

                            match &element.tag {
                                HTMLTag::OL | HTMLTag::UL => {
                                    style_context.list_state.pop();
                                    style_context.list_index.pop();
                                },
                                _ => {}
                            }
                        }
                    }
                    false => {
                        match &element.tag {
                            HTMLTag::OL => {
                                style_context.list_state.push(Some(ListState::ORDERED));
                                style_context.list_index.push(0);
                            },
                            HTMLTag::UL => {
                                style_context.list_state.push(Some(ListState::UNORDERED));
                                style_context.list_index.push(0);
                            }
                            HTMLTag::LI => {
                                if let Some(index) = style_context.list_index.pop() {
                                    let new_index = index + 1;
                                    style_context.list_index.push(new_index);
                                }
                            }
                            HTMLTag::A => {
                                style_context.link_index += 1;
                            }
                            _ => {}
                        }

                        match is_self_closing(&element.tag) {
                            true => {
                                lines = vec![
                                    lines,
                                    get_html_style(&element.tag, &mut spans, &style_context)
                                ].concat();
                            },
                            false => {
                                style_context.add_modifiers(element.tag.clone());
                                element_stack.push(element);
                            }
                        }
                    }
                }
            },
            Token::Text(text) => {
                spans.push(style_context.construct_span(text));
            },
            Token::NewLine => {
                if !spans.is_empty() {
                    lines.push(Line::from(spans.clone()));
                    spans.clear();
                }
            }
            Token::EOF => {
                if !spans.is_empty() {
                    lines.push(Line::from(spans.clone()));
                    spans.clear();
                }
            }
        }
    }

    Paragraph::new(lines).wrap(Wrap { trim: false })
}

mod test {
    #![allow(dead_code)]
    use std::collections::HashMap;

    use insta::assert_snapshot;
    use ratatui::{Terminal, backend::TestBackend};

    use crate::tuihtml::{html::{HTMLElement, HTMLTag}, parser::{construct_widget, parse_html}, tokenizer::Token};


    #[test]
    fn parse_html_returns_only_text() {

        let html = r#"
Title
Hello World
Google"#;

        let tokens = parse_html(html.into());

        assert_eq!(tokens, Vec::from([
            Token::Text("Title".into()),
            Token::NewLine,
            Token::Text("Hello World".into()),
            Token::NewLine,
            Token::Text("Google".into()),
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
    fn test_render() {
        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
        terminal
            .draw(|frame| {

                let html = r#"
<h2>Ordered List</h2>
<ol>
    <li>First step</li>
    <li>Second step</li>
    <li>Third step</li>
    <ol>
        <li>Nested step A</li>
        <li>Nested step B</li>
    </ol>
</ol>

<h2>Mixed List</h2>
<ol>
    <li>Note 1</li>
    <li>Node 2</li>
    <li>Note 3</li>
    <ul>
        <li>Step 1</li>
        <li>Step 2</li>
    </ul>
    <li>Note 4</li>
</ol>
"#;

                frame.render_widget(construct_widget(String::from(html)), frame.area());
            })
            .unwrap();
        assert_snapshot!(terminal.backend());
    }
}

