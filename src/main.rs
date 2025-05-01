use std::{collections::HashMap, str::Chars};


// Enum for the different HTML tags
#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    ELEMENT(HTMLElement),
    TEXT(String),
    EOF,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum HTMLTag {
    HTML,
    HEAD,
    BODY,
    TITLE,
    META,
    LINK,
    STYLE,
    DIV,
    SPAN,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    P,
    A,
    BOLD,
    BR,
    HR,
    IMG,
    INPUT,
    BUTTON,
    SELECT,
    OPTION,
    FORM,
    LABEL,
    TABLE,
    TR,
    TH,
    TD,
    UL,
    OL,
    LI,
    SCRIPT,
    TEXTAREA,
    IFRAME,
    VIDEO,
    AUDIO,
    SOURCE,
    NAV,
    HEADER,
    FOOTER,
    SECTION,
    ARTICLE,
    ASIDE,
    MAIN,
    FIGURE,
    FIGCAPTION,
    STRONG,
    EM,
    CODE,
    PRE,
    BLOCKQUOTE,
    CITE,
    ABBR,
    TIME,
    DATA,
    PROGRESS,
    METER,
    DETAILS,
    SUMMARY,
    DIALOG,
    CANVAS,
    SVG,
    MATH,
    TEMPLATE,
    SLOT,
    OUTPUT,
    FIELDSET,
    LEGEND,
    DATALIST,
    OPTGROUP,
    SMALL,
    SUB,
    SUP,
    MARK,
    RUBY,
    RT,
    RP,
    BDI,
    BDO,
    WBR,
    EMBED,
    OBJECT,
    PARAM,
    TRACK,
    MAP,
    AREA,
    COL,
    COLGROUP,
    CAPTION,
    THEAD,
    TBODY,
    TFOOT,
    NOSCRIPT,
    BASE,
    UNKNOWN
}

impl HTMLTag {
    fn from_string(string: String) -> HTMLTag {
        match string.to_lowercase().as_str() {
            "html" => HTMLTag::HTML,
            "head" => HTMLTag::HEAD,
            "body" => HTMLTag::BODY,
            "title" => HTMLTag::TITLE,
            "meta" => HTMLTag::META,
            "link" => HTMLTag::LINK,
            "style" => HTMLTag::STYLE,
            "div" => HTMLTag::DIV,
            "span" => HTMLTag::SPAN,
            "h1" => HTMLTag::H1,
            "h2" => HTMLTag::H2,
            "h3" => HTMLTag::H3,
            "h4" => HTMLTag::H4,
            "h5" => HTMLTag::H5,
            "h6" => HTMLTag::H6,
            "p" => HTMLTag::P,
            "a" => HTMLTag::A,
            "b" => HTMLTag::BOLD,
            "br" => HTMLTag::BR,
            "hr" => HTMLTag::HR,
            "img" => HTMLTag::IMG,
            "input" => HTMLTag::INPUT,
            "button" => HTMLTag::BUTTON,
            "select" => HTMLTag::SELECT,
            "option" => HTMLTag::OPTION,
            "form" => HTMLTag::FORM,
            "label" => HTMLTag::LABEL,
            "table" => HTMLTag::TABLE,
            "tr" => HTMLTag::TR,
            "th" => HTMLTag::TH,
            "td" => HTMLTag::TD,
            "ul" => HTMLTag::UL,
            "ol" => HTMLTag::OL,
            "li" => HTMLTag::LI,
            "script" => HTMLTag::SCRIPT,
            "textarea" => HTMLTag::TEXTAREA,
            "iframe" => HTMLTag::IFRAME,
            "video" => HTMLTag::VIDEO,
            "audio" => HTMLTag::AUDIO,
            "source" => HTMLTag::SOURCE,
            "nav" => HTMLTag::NAV,
            "header" => HTMLTag::HEADER,
            "footer" => HTMLTag::FOOTER,
            "section" => HTMLTag::SECTION,
            "article" => HTMLTag::ARTICLE,
            "aside" => HTMLTag::ASIDE,
            "main" => HTMLTag::MAIN,
            "figure" => HTMLTag::FIGURE,
            "figcaption" => HTMLTag::FIGCAPTION,
            "strong" => HTMLTag::STRONG,
            "em" => HTMLTag::EM,
            "code" => HTMLTag::CODE,
            "pre" => HTMLTag::PRE,
            "blockquote" => HTMLTag::BLOCKQUOTE,
            "cite" => HTMLTag::CITE,
            "abbr" => HTMLTag::ABBR,
            "time" => HTMLTag::TIME,
            "data" => HTMLTag::DATA,
            "progress" => HTMLTag::PROGRESS,
            "meter" => HTMLTag::METER,
            "details" => HTMLTag::DETAILS,
            "summary" => HTMLTag::SUMMARY,
            "dialog" => HTMLTag::DIALOG,
            "canvas" => HTMLTag::CANVAS,
            "svg" => HTMLTag::SVG,
            "math" => HTMLTag::MATH,
            "template" => HTMLTag::TEMPLATE,
            "slot" => HTMLTag::SLOT,
            "output" => HTMLTag::OUTPUT,
            "fieldset" => HTMLTag::FIELDSET,
            "legend" => HTMLTag::LEGEND,
            "datalist" => HTMLTag::DATALIST,
            "optgroup" => HTMLTag::OPTGROUP,
            "small" => HTMLTag::SMALL,
            "sub" => HTMLTag::SUB,
            "sup" => HTMLTag::SUP,
            "mark" => HTMLTag::MARK,
            "ruby" => HTMLTag::RUBY,
            "rt" => HTMLTag::RT,
            "rp" => HTMLTag::RP,
            "bdi" => HTMLTag::BDI,
            "bdo" => HTMLTag::BDO,
            "wbr" => HTMLTag::WBR,
            "embed" => HTMLTag::EMBED,
            "object" => HTMLTag::OBJECT,
            "param" => HTMLTag::PARAM,
            "track" => HTMLTag::TRACK,
            "map" => HTMLTag::MAP,
            "area" => HTMLTag::AREA,
            "col" => HTMLTag::COL,
            "colgroup" => HTMLTag::COLGROUP,
            "caption" => HTMLTag::CAPTION,
            "thead" => HTMLTag::THEAD,
            "tbody" => HTMLTag::TBODY,
            "tfoot" => HTMLTag::TFOOT,
            "noscript" => HTMLTag::NOSCRIPT,
            "base" => HTMLTag::BASE,
            _ => HTMLTag::UNKNOWN
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HTMLElement {
    tag: HTMLTag,
    attributes: HashMap<String, String>,
    closing: bool
}

impl HTMLElement {
    pub fn new(tag: HTMLTag, attributes: HashMap<String, String>, closing: bool) -> Self {
        HTMLElement {
            tag,
            attributes,
            closing
        }
    }
}

#[derive(Debug, Clone)]
struct HTMLTokenizer<'a> {
    html_pos: Chars<'a>,
    next_char: char,
}

impl HTMLTokenizer<'_> {

    pub fn new(html: Chars<'_>) -> HTMLTokenizer {
        HTMLTokenizer {
            html_pos: html,
            next_char: '\0',
        }
    }

    pub fn init(&mut self) {
        self.next_char = self.html_pos.next().unwrap();
        self.consume_whitespace();
    }

    pub fn next(&mut self) -> Token  {
        let mut lexeme = String::new();

        while !self.html_pos.clone().eq(self.html_pos.clone().last()) {
            if self.next_char == '<' {
                if lexeme.is_empty() {
                    self.next_char();
                    return Token::ELEMENT(self.capture_element())
                }
                return Token::TEXT(lexeme);
            }

            lexeme.push(self.next_char);
            self.next_char();
        }

        return Token::EOF;
    }

    fn capture_element(&mut self) -> HTMLElement {
        let mut closing_tag = false;
        let mut title = String::new();
        let mut attributes = HashMap::new();

        while !self.html_pos.clone().eq(self.html_pos.clone().last()) {

            if self.next_char.is_whitespace() && !closing_tag {
                self.consume_whitespace();
                attributes = self.capture_tag_attributes();
            }

            if self.next_char == '/' {
                closing_tag = true;
                self.next_char();
            }

            if self.next_char == '>' {
                self.next_char();
                if self.next_char == '\n' {
                    self.consume_whitespace();
                }
                return HTMLElement::new(HTMLTag::from_string(title), attributes, closing_tag);
            }

            title.push(self.next_char);
            self.next_char();
        }

        return HTMLElement::new(HTMLTag::from_string(title), attributes, closing_tag);

    }

    fn capture_tag_attributes(&mut self) -> HashMap<String, String> {
        let mut attributes = HashMap::new();
        let mut key = String::new();
        let mut value = String::new();

        enum AttributeState {
            KEY,
            VALUE
        }

        let mut state = AttributeState::KEY;

        while !self.html_pos.clone().eq(self.html_pos.clone().last()) {

            if self.next_char != '=' {
                match state {
                    AttributeState::KEY => {
                        key.push(self.next_char);
                    },
                    AttributeState::VALUE => {
                        value.push(self.next_char);
                    }
                }
            }
            else {
                state = AttributeState::VALUE;
            }

            if self.next_char.is_whitespace() {
                attributes.insert(key.clone(), value.clone());
                state = AttributeState::KEY;
                key = String::new();
                value = String::new();
            }

            if self.next_char == '>' {
                attributes.insert(key.clone(), value.clone());
                return attributes;
            }


            self.next_char();
        }

        return attributes;
    }

    fn next_char(&mut self) {
        if !self.html_pos.clone().eq(self.html_pos.clone().last()) {
            self.next_char = self.html_pos.next().unwrap();
        }
    }

    fn consume_whitespace(&mut self) {
        while self.next_char.is_whitespace() && !self.html_pos.clone().eq(self.html_pos.clone().last()) {
            self.next_char();
        }
    }
}

fn main() {

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

    parse_html(html.into());

}

fn parse_html(html: String) {

    let mut tokenizer = HTMLTokenizer::new(html.chars());
    tokenizer.init();

    let mut current_token = tokenizer.next();

    while current_token != Token::EOF {
        println!("{:?}", current_token);

        current_token = tokenizer.next();
    }

    print!("\r");
}
