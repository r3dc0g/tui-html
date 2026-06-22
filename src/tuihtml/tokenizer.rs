use std::{collections::HashMap, str::Chars};

use crate::tuihtml::html::{HtmlElement, HtmlTag};

// Enum for the different HTML tags
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Element(HtmlElement),
    Text(String),
    NewLine,
    Eof,
}

impl Token {
    pub fn get_html_element(token: &Token) -> Option<HtmlElement> {
        match token {
            Token::Element(e) => Some(e.clone()),
            Token::Text(_) => None,
            Token::NewLine => None,
            Token::Eof => None,
        }
    }

    pub fn get_text(token: &Token) -> Option<String> {
        match token {
            Token::Element(_) => None,
            Token::Text(s) => Some(s.clone()),
            Token::NewLine => None,
            Token::Eof => None,
        }
    }

    pub fn is_whitespace(token: &Token) -> bool {
        match token {
            Token::Text(s) => {
                for char in s.chars() {
                    if !char.is_whitespace() {
                        return false;
                    }
                }
                true
            }
            _ => false
        }
    }
}

#[derive(Debug, Clone)]
pub struct HtmlTokenizer<'a> {
    html_pos: Chars<'a>,
    next_char: Option<char>,
}

impl<'a> HtmlTokenizer<'a> {

    pub fn new(html: Chars<'a>) -> HtmlTokenizer<'a> {
        let mut tokenizer = HtmlTokenizer {
            html_pos: html,
            next_char: None,
        };

        tokenizer.init();
        tokenizer
    }

    pub fn init(&mut self) {
        self.next_char = self.html_pos.next();
        self.consume_whitespace();
    }

    pub fn next(&mut self) -> Token  {
        let mut lexeme = String::new();

        loop {
            match self.next_char {
                Some('<') => {
                    if lexeme.is_empty() {
                        self.next_char();
                        return Token::Element(self.capture_element())
                    }
                    // Leave '<' unconsumed as next_char so the next call
                    // to next() can begin parsing the element.
                    return Token::Text(lexeme);
                },
                Some('\n') | Some('\r') => {
                    if lexeme.is_empty() {
                        self.consume_whitespace();
                        return Token::NewLine;
                    }
                    return Token::Text(lexeme);
                },
                None => {
                    break;
                },
                _ => {
                    if let Some(char) = self.next_char {
                        lexeme.push(char);
                    }
                    self.next_char();
                }
            }
        }

        if lexeme.is_empty() {
            Token::Eof
        }
        else {
            Token::Text(lexeme)
        }
    }

    fn capture_element(&mut self) -> HtmlElement {
        let mut closing_tag = false;
        let mut tag = String::new();
        let mut attributes = HashMap::new();

        loop {

            if self.next_char.is_some_and(|c| c.is_whitespace()) && !closing_tag {
                self.consume_whitespace();
                attributes = self.capture_tag_attributes();
            }

            match self.next_char {
                Some('/') => {
                    closing_tag = true;
                    self.next_char();
                },
                Some('>') => {
                    self.next_char();
                    // Only collapse trailing newlines/whitespace after '>'.
                    // In a TUI context we may not know what styling applies,
                    // so we trim for clarity rather than preserving all whitespace.
                    if self.next_char.is_some_and(|c| c == '\n' || c == '\r' ) {
                        self.consume_whitespace();
                    }
                    return HtmlElement::new(HtmlTag::from_string(&tag), attributes, closing_tag);
                },
                None => {
                    break;
                }
                _ => {}
            }

            if let Some(char) = self.next_char {
                tag.push(char);
            }
            self.next_char();
        }

        HtmlElement::new(HtmlTag::from_string(&tag), attributes, closing_tag)

    }

    fn capture_tag_attributes(&mut self) -> HashMap<String, String> {
        let mut attributes = HashMap::new();
        let mut key = String::new();
        let mut value = String::new();

        #[derive(PartialEq)]
        enum AttributeState {
            Key,
            Value
        }

        let mut state = AttributeState::Key;

        while self.next_char.is_some() {
            match self.next_char {
                Some('>') => {
                    return attributes;
                },
                Some('=') => {
                    if state == AttributeState::Key {
                        state = AttributeState::Value;
                        self.next_char();           // advance past '='
                        self.consume_whitespace();
                        self.next_char();           // skip the opening quote so value content is
                                                    // captured directly; the closing quote will be
                                                    // caught by the quote arm below
                    }
                },
                Some('\"') | Some('\'') => {
                    if state == AttributeState::Value {
                        state = AttributeState::Key;
                        attributes.insert(key.clone(), value.clone());
                        self.next_char();
                        self.consume_whitespace();
                        key = String::new();
                        value = String::new();
                    }
                }
                None => {
                    break;
                }
                _ => {
                    match state {
                        AttributeState::Key => {
                            if self.next_char.is_some_and(|c| !c.is_whitespace()) {
                                if let Some(char) = self.next_char {
                                    key.push(char);
                                }
                            }
                        },
                        AttributeState::Value => {
                            if let Some(char) = self.next_char {
                                value.push(char);
                            }
                        }
                    }
                    self.next_char();
                }
            }
        }

        attributes
    }

    fn next_char(&mut self) {
        self.next_char = self.html_pos.next();
    }

    fn consume_whitespace(&mut self) {
        while self.next_char.is_some_and(|c| c.is_whitespace()) {
            self.next_char();
        }
    }
}

