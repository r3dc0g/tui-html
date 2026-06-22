use ratatui::{style::{Modifier, Style, Stylize}, text::{Line, Span}, widgets::{Paragraph, Wrap}};
use crate::tuihtml::{html::*, tokenizer::{HtmlTokenizer, Token}, widget::{HtmlWidget}};

#[derive(Default, Debug, Clone, PartialEq)]
pub enum ListState {
    #[default]
    Ordered,
    Unordered
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct StyleContext {
    list_state: Vec<ListState>,
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

    pub fn remove_modifiers(&mut self, tag: HtmlTag) {
        let modifiers = tag.to_modifiers();
        for modifier in modifiers {
            if let Some(i) = self.active_modifiers.iter().position(|m| *m == modifier) {
                self.active_modifiers.remove(i);
            }

        }
    }

    pub fn add_modifiers(&mut self, tag: HtmlTag) {
        for modifier in tag.to_modifiers() {
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

    pub fn styled(&self) -> bool {
        if self.active_modifiers.is_empty() && self.active_styles.is_empty() {
            return false
        }

        true
    }
}

pub fn get_html_style<'a>(tag: &HtmlTag, mut spans: Vec<Span<'a>>, context: &StyleContext) -> (Vec<Span<'a>>, Vec<Line<'a>>) {
    const HR_WIDTH: usize = 120;

    match tag {
        HtmlTag::H1 => {
            let spans_width: usize = spans.iter().map(|span| span.width()).collect::<Vec<usize>>().iter().sum();
            let header_spacing = vec![Span::from(" ".repeat(spans_width / 2))];
            let header_line = Line::from([header_spacing.clone(),
                spans.clone(),
                header_spacing].concat());
            let overscore_line = Line::from(Span::from("\u{00AF}".repeat(spans_width * 2)));

            let lines = vec![header_line.centered(), overscore_line.centered()];
            (Vec::new(), lines)
        },
        HtmlTag::HR => {
            let overscore_line = Line::from(Span::from("\u{00AF}".repeat(HR_WIDTH)));
            let underscore_line = Line::from(Span::from("\u{005F}".repeat(HR_WIDTH)));

            let lines = vec![underscore_line.centered(), overscore_line.centered()];
            (spans, lines)
        },
        HtmlTag::LI => {
            match context.list_state.last() {
                Some(last_state) => {
                    match last_state {
                        ListState::Ordered => {
                            let list_item = Line::from(
                                [vec![Span::from(format!(
                                        "{}{}. ",
                                        " ".repeat(2 * context.list_state.len()),
                                        context.list_index.last()
                                        .unwrap_or(&0_usize)
                                        .clone()
                                ))],
                                spans.clone()].concat()
                            );
                            (Vec::new(), vec![list_item])
                        },
                        ListState::Unordered => {
                            let list_item = Line::from(
                                [vec![Span::from(format!(
                                        "{}\u{2022} ",
                                        " ".repeat(2 * context.list_state.len()),
                                ))],
                                spans.clone()].concat()
                            );
                            (Vec::new(), vec![list_item])
                        }
                    }
                },
                None => {
                    (spans, Vec::new())
                }
            }
        },
        HtmlTag::A => {
            spans.push(Span::from(format!("[{}]", context.link_index)).bold());
            (spans, Vec::new())
        },
        HtmlTag::H2 => {
            let spans_width: usize = spans.iter().map(|span| span.width()).collect::<Vec<usize>>().iter().sum();
            let above_line = Line::from(Span::from("\u{00A0}".repeat(spans_width)));
            let header_line = Line::from(spans.clone());

            let lines = vec![above_line, header_line.underlined()];
            (Vec::new(), lines)
        }
        HtmlTag::H3 |
        HtmlTag::H4 |
        HtmlTag::H5 |
        HtmlTag::H6 => {
            let spans_width: usize = spans.iter().map(|span| span.width()).collect::<Vec<usize>>().iter().sum();
            let above_line = Line::from(Span::from("\u{00A0}".repeat(spans_width)));
            let header_line = Line::from(spans.clone());

            let lines = vec![above_line, header_line];
            (Vec::new(), lines)
        },
        HtmlTag::NAV |
        HtmlTag::DIV |
        HtmlTag::OL |
        HtmlTag::UL |
        HtmlTag::P => {
            match spans.is_empty() {
                true => {
                    (spans, Vec::new())
                },
                false => {
                    let line = Line::from(spans.clone());
                    (Vec::new(), vec![line])
                }
            }
        },
        _ => (spans, Vec::new())
    }
}

pub fn is_self_closing(tag: &HtmlTag) -> bool {
    match tag {
        HtmlTag::HR => true,
        HtmlTag::BR => true,
        _ => false
    }
}

fn parse_html(html: String) -> Vec<Token> {

    let mut tokens = Vec::new();
    let mut tokenizer = HtmlTokenizer::new(html.chars());

    let mut current_token = tokenizer.next();
    tokens.push(current_token.clone());


    while current_token != Token::Eof {
        current_token = tokenizer.next();
        tokens.push(current_token.clone());
    }

    tokens
}

pub fn construct_widget<'a>(html: String) -> HtmlWidget<'a> {

    let tokens = parse_html(html);
    let mut lines: Vec<Line<'a>> = Vec::new();
    let mut spans: Vec<Span<'a>> = Vec::new();
    let mut style_context = StyleContext::new();
    let mut element_stack: Vec<HtmlElement> = Vec::new();

    for token in tokens {

        // Skip whitespace tokens when not inside a styled context or a
        // block-level text container. In a TUI we may not know what styling
        // applies, so we collapse extraneous whitespace for clarity —
        // except inside <p>, <div>, <section>, and <article> where spacing
        // is semantically meaningful.
        if Token::is_whitespace(&token) &&
            !style_context.styled() &&
            (element_stack.last().is_none_or(|el| el.tag != HtmlTag::P && el.tag != HtmlTag::DIV && el.tag != HtmlTag::SECTION && el.tag != HtmlTag::ARTICLE)) {
            continue;
        }

        match token {
            Token::Element(element) => {
                match &element.closing {
                    true => {
                        if let Some(removed_element) = element_stack.pop() {
                            let (new_spans, new_lines) = get_html_style(&removed_element.tag, spans, &style_context);
                            lines = [lines, new_lines].concat();
                            spans = new_spans;

                            style_context.remove_modifiers(removed_element.tag);

                            match &element.tag {
                                HtmlTag::OL | HtmlTag::UL => {
                                    lines.push(Line::raw("\u{00A0}"));
                                    style_context.list_state.pop();
                                    style_context.list_index.pop();
                                },
                                HtmlTag::P => {
                                    lines.push(Line::raw("\u{00A0}"));
                                },
                                _ => {}
                            }
                        }
                    }
                    false => {
                        match &element.tag {
                            HtmlTag::OL => {
                                style_context.list_state.push(ListState::Ordered);
                                style_context.list_index.push(0);
                            },
                            HtmlTag::UL => {
                                style_context.list_state.push(ListState::Unordered);
                                style_context.list_index.push(0);
                            }
                            HtmlTag::LI => {
                                if let Some(index) = style_context.list_index.pop() {
                                    let new_index = index + 1;
                                    style_context.list_index.push(new_index);
                                }
                            },
                            HtmlTag::A => {
                                style_context.link_index += 1;
                                if let Some(link) = element.attributes.get("href") {
                                    style_context.link_list.push(link.to_owned());
                                }
                            }
                            HtmlTag::IMG => {
                                style_context.img_index += 1;
                                if let Some(image) = element.attributes.get("src") {
                                    style_context.img_list.push(image.to_owned());
                                }
                            }
                            _ => {}
                        }

                        match is_self_closing(&element.tag) {
                            true => {
                                let (new_spans, new_lines) = get_html_style(&element.tag, spans, &style_context);
                                lines = [lines, new_lines].concat();
                                spans = new_spans;
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
            Token::Eof => {
                if !spans.is_empty() {
                    lines.push(Line::from(spans.clone()));
                    spans.clear();
                }
            }
        }
    }

    HtmlWidget {
        paragraph: Paragraph::new(lines).wrap(Wrap { trim: false }),
        links: style_context.link_list,
        images: style_context.img_list
    }
}

mod test {
    #![allow(dead_code)]
    use std::collections::HashMap;

    use insta::assert_snapshot;
    use ratatui::{Terminal, backend::TestBackend};

    use crate::tuihtml::{html::{HtmlElement, HtmlTag}, parser::{construct_widget, parse_html}, tokenizer::Token, widget::HtmlWidget};


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
            Token::Eof
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
            Token::Element(HtmlElement { tag: HtmlTag::HTML, attributes: HashMap::new(), closing: false }),
            Token::Element(HtmlElement { tag: HtmlTag::BODY, attributes: HashMap::new(), closing: false }),
            Token::Element(HtmlElement { tag: HtmlTag::H1, attributes: HashMap::new(), closing: false }),
            Token::Text("Title".into()),
            Token::Element(HtmlElement { tag: HtmlTag::H1, attributes: HashMap::new(), closing: true }),
            Token::Element(HtmlElement { tag: HtmlTag::P, attributes: HashMap::new(), closing: false }),
            Token::Element(HtmlElement { tag: HtmlTag::BOLD, attributes: HashMap::new(), closing: false }),
            Token::Text("Hello".into()),
            Token::Element(HtmlElement { tag: HtmlTag::BOLD, attributes: HashMap::new(), closing: true }),
            Token::Text(" World".into()),
            Token::Element(HtmlElement { tag: HtmlTag::P, attributes: HashMap::new(), closing: true }),
            Token::Element(HtmlElement { tag: HtmlTag::A, attributes: HashMap::from([("href".into(), "https://www.google.com".into()); 1]), closing: false}),
            Token::Text("Google".into()),
            Token::Element(HtmlElement { tag: HtmlTag::A, attributes: HashMap::new(), closing: true }),
            Token::Element(HtmlElement { tag: HtmlTag::IMG, attributes: HashMap::from([("src".into(), "/home/garrett/Documents/image.jpg".into()); 1]), closing: false }),
            Token::Element(HtmlElement { tag: HtmlTag::IMG, attributes: HashMap::new(), closing: true }),
            Token::Element(HtmlElement { tag: HtmlTag::BODY, attributes: HashMap::new(), closing: true }),
            Token::Element(HtmlElement { tag: HtmlTag::HTML, attributes: HashMap::new(), closing: true }),
            Token::Eof
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
            Token::Element(HtmlElement { tag: HtmlTag::H1, attributes: HashMap::new(), closing: false }),
            Token::Text("Title".into()),
            Token::Element(HtmlElement { tag: HtmlTag::H1, attributes: HashMap::new(), closing: true }),
            Token::Element(HtmlElement { tag: HtmlTag::P, attributes: HashMap::new(), closing: false }),
            Token::Element(HtmlElement { tag: HtmlTag::BOLD, attributes: HashMap::new(), closing: false }),
            Token::Text("Hello".into()),
            Token::Element(HtmlElement { tag: HtmlTag::BOLD, attributes: HashMap::new(), closing: true }),
            Token::Text(" World".into()),
            Token::Element(HtmlElement { tag: HtmlTag::P, attributes: HashMap::new(), closing: true }),
            Token::Element(HtmlElement { tag: HtmlTag::A, attributes: HashMap::from([("href".into(), "https://www.google.com".into()); 1]), closing: false}),
            Token::Text("Google".into()),
            Token::Element(HtmlElement { tag: HtmlTag::A, attributes: HashMap::new(), closing: true }),
            Token::Element(HtmlElement { tag: HtmlTag::IMG, attributes: HashMap::from([("src".into(), "/home/garrett/Documents/image.jpg".into()); 1]), closing: false }),
            Token::Element(HtmlElement { tag: HtmlTag::IMG, attributes: HashMap::new(), closing: true }),
            Token::Eof
        ]))
    }

    #[test]
    fn images_are_properly_indexed_and_stored() {
        let html = r#"
        <h1>Title</h1>
        <p><b>Hello</b> World</p>
        <a href="https://www.google.com">Google</a>
        <img src="/home/garrett/Documents/image1.jpg"></img>
        <img src="/home/garrett/Documents/image2.jpg"></img>
        <img src="/home/garrett/Documents/image3.jpg"></img>
    "#;

        let widget = construct_widget(html.into());

        assert!(widget.images.len() == 3);
        assert!(widget.images[0] == "/home/garrett/Documents/image1.jpg" );
        assert!(widget.images[1] == "/home/garrett/Documents/image2.jpg" );
        assert!(widget.images[2] == "/home/garrett/Documents/image3.jpg" );
    }

    #[test]
    fn test_render() {
        if let Ok(mut terminal) = Terminal::new(TestBackend::new(80, 20)) {
            terminal
                .draw(|frame| {

                    let html = r#"
 <p>Breht reads and reacts to "A Letter To The American People" written by Masoud Pezeschkian, the Iranian president, as a ground invasion of some sort seems imminent. Check out our new design in collaboration with <em>Goods for the People</em> <a href="https://goodsforthepeople.com/">HERE</a></p> <p> </p> <p> -------------------------------------------------------------------------------</p> <p>Support Rev Left and get bonus episodes on <a href= "https://www.patreon.com/revleftradio">Patreon</a></p> <p>Make a one-time donation to Rev Left at <a href= "https://buymeacoffee.com/revleftradio">BuyMeACoffee.com/revleftradio</a></p> <p>Follow RLR on IG <a href= "https://www.instagram.com/rev_left_radio_official/" rel= "noopener noreferrer nofollow">HERE</a></p> <p>Learn more about Rev Left <a href= "https://revleftradio.com/">HERE</a></p>
 "#;
                    let html_widget = HtmlWidget::new(String::from(html));
                    frame.render_widget(html_widget.get_paragraph(), frame.area());
                })
            .unwrap();
            assert_snapshot!(terminal.backend());
        }
    }
}

