use std::collections::HashMap;

use ratatui::{style::Modifier};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HtmlTag {
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
    U,
    S,
    DEL,
    BR,
    HR,
    IMG,
    LABEL,
    TABLE,
    TR,
    TH,
    TD,
    UL,
    OL,
    LI,
    TEXTAREA,
    IFRAME,
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
    I,
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
    AREA,
    COL,
    COLGROUP,
    CAPTION,
    THEAD,
    TBODY,
    TFOOT,
    UNKNOWN,
}

impl HtmlTag {
    pub fn from_string(string: &str) -> HtmlTag {
        match string.to_lowercase().as_str() {
            "html" => HtmlTag::HTML,
            "head" => HtmlTag::HEAD,
            "body" => HtmlTag::BODY,
            "title" => HtmlTag::TITLE,
            "meta" => HtmlTag::META,
            "link" => HtmlTag::LINK,
            "style" => HtmlTag::STYLE,
            "div" => HtmlTag::DIV,
            "span" => HtmlTag::SPAN,
            "h1" => HtmlTag::H1,
            "h2" => HtmlTag::H2,
            "h3" => HtmlTag::H3,
            "h4" => HtmlTag::H4,
            "h5" => HtmlTag::H5,
            "h6" => HtmlTag::H6,
            "p" => HtmlTag::P,
            "a" => HtmlTag::A,
            "b" => HtmlTag::BOLD,
            "u" => HtmlTag::U,
            "s" => HtmlTag::S,
            "del" => HtmlTag::DEL,
            "br" => HtmlTag::BR,
            "hr" => HtmlTag::HR,
            "img" => HtmlTag::IMG,
            "label" => HtmlTag::LABEL,
            "table" => HtmlTag::TABLE,
            "tr" => HtmlTag::TR,
            "th" => HtmlTag::TH,
            "td" => HtmlTag::TD,
            "ul" => HtmlTag::UL,
            "ol" => HtmlTag::OL,
            "li" => HtmlTag::LI,
            "textarea" => HtmlTag::TEXTAREA,
            "iframe" => HtmlTag::IFRAME,
            "source" => HtmlTag::SOURCE,
            "nav" => HtmlTag::NAV,
            "header" => HtmlTag::HEADER,
            "footer" => HtmlTag::FOOTER,
            "section" => HtmlTag::SECTION,
            "article" => HtmlTag::ARTICLE,
            "aside" => HtmlTag::ASIDE,
            "main" => HtmlTag::MAIN,
            "figure" => HtmlTag::FIGURE,
            "figcaption" => HtmlTag::FIGCAPTION,
            "strong" => HtmlTag::STRONG,
            "em" => HtmlTag::EM,
            "i" => HtmlTag::I,
            "code" => HtmlTag::CODE,
            "pre" => HtmlTag::PRE,
            "blockquote" => HtmlTag::BLOCKQUOTE,
            "cite" => HtmlTag::CITE,
            "abbr" => HtmlTag::ABBR,
            "time" => HtmlTag::TIME,
            "data" => HtmlTag::DATA,
            "progress" => HtmlTag::PROGRESS,
            "meter" => HtmlTag::METER,
            "details" => HtmlTag::DETAILS,
            "summary" => HtmlTag::SUMMARY,
            "dialog" => HtmlTag::DIALOG,
            "canvas" => HtmlTag::CANVAS,
            "svg" => HtmlTag::SVG,
            "math" => HtmlTag::MATH,
            "template" => HtmlTag::TEMPLATE,
            "slot" => HtmlTag::SLOT,
            "output" => HtmlTag::OUTPUT,
            "fieldset" => HtmlTag::FIELDSET,
            "legend" => HtmlTag::LEGEND,
            "datalist" => HtmlTag::DATALIST,
            "optgroup" => HtmlTag::OPTGROUP,
            "small" => HtmlTag::SMALL,
            "area" => HtmlTag::AREA,
            "col" => HtmlTag::COL,
            "colgroup" => HtmlTag::COLGROUP,
            "caption" => HtmlTag::CAPTION,
            "thead" => HtmlTag::THEAD,
            "tbody" => HtmlTag::TBODY,
            "tfoot" => HtmlTag::TFOOT,
            _ => HtmlTag::UNKNOWN
        }
    }

    pub fn to_modifiers(&self) -> Vec<Modifier> {
        match self {
            HtmlTag::BOLD |
            HtmlTag::STRONG |
            HtmlTag::H1 |
            HtmlTag::H2 |
            HtmlTag::H3 |
            HtmlTag::H4 |
            HtmlTag::H5 |
            HtmlTag::H6 => vec![Modifier::BOLD],

            HtmlTag::EM | HtmlTag::I => vec![Modifier::ITALIC],

            HtmlTag::U | HtmlTag::A => vec![Modifier::UNDERLINED],

            HtmlTag::S | HtmlTag::DEL => vec![Modifier::CROSSED_OUT],

            _ => vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlElement {
    pub tag: HtmlTag,
    pub attributes: HashMap<String, String>,
    pub closing: bool
}

impl HtmlElement {
    pub fn new(tag: HtmlTag, attributes: HashMap<String, String>, closing: bool) -> Self {
        HtmlElement {
            tag,
            attributes,
            closing
        }
    }
}
