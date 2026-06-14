use std::{collections::HashMap, str::Chars};

use crate::tuihtml::html::{HTMLElement, HTMLTag};

// Enum for the different HTML tags
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Element(HTMLElement),
    Text(String),
    NewLine,
    EOF,
}

impl Token {
    pub fn get_html_element(token: &Token) -> Option<HTMLElement> {
        match token {
            Token::Element(e) => Some(e.clone()),
            Token::Text(_) => None,
            Token::NewLine => None,
            Token::EOF => None,
        }
    }

    pub fn get_text(token: &Token) -> Option<String> {
        match token {
            Token::Element(_) => None,
            Token::Text(s) => Some(s.clone()),
            Token::NewLine => None,
            Token::EOF => None,
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
pub struct HTMLTokenizer<'a> {
    html_pos: Chars<'a>,
    next_char: Option<char>,
}

impl<'a> HTMLTokenizer<'a> {

    pub fn new(html: Chars<'a>) -> HTMLTokenizer<'a> {
        HTMLTokenizer {
            html_pos: html,
            next_char: Some('\0'),
        }
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
                    lexeme.push(self.next_char.unwrap());
                    self.next_char();
                }
            }
        }

        if lexeme.is_empty() {
            Token::EOF
        }
        else {
            Token::Text(lexeme)
        }
    }

    fn capture_element(&mut self) -> HTMLElement {
        let mut closing_tag = false;
        let mut tag = String::new();
        let mut attributes = HashMap::new();

        loop {

            if self.next_char.is_some() && self.next_char.unwrap().is_whitespace() && !closing_tag {
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
                    if let Some('\n') = self.next_char {
                        self.consume_whitespace();
                    }
                    return HTMLElement::new(HTMLTag::from_string(tag), attributes, closing_tag);
                },
                None => {
                    break;
                }
                _ => {}
            }

            tag.push(self.next_char.unwrap());
            self.next_char();
        }

        HTMLElement::new(HTMLTag::from_string(tag), attributes, closing_tag)

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
                        self.next_char();
                        self.consume_whitespace();
                        self.next_char();
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
                            key.push(self.next_char.unwrap());
                        },
                        AttributeState::Value => {
                            value.push(self.next_char.unwrap());
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
        while self.next_char.is_some() && self.next_char.unwrap().is_whitespace() {
            self.next_char();
        }
    }
}

