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
    pub fn get_html_element(token: Token) -> Option<HTMLElement> {
        match token {
            Token::Element(e) => Some(e),
            Token::Text(_) => None,
            Token::NewLine => None,
            Token::EOF => None,
        }
    }

    pub fn get_text(token:Token) -> Option<String> {
        match token {
            Token::Element(_) => None,
            Token::Text(s) => Some(s),
            Token::NewLine => None,
            Token::EOF => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HTMLTokenizer<'a> {
    html_pos: Chars<'a>,
    next_char: char,
}

impl<'a> HTMLTokenizer<'a> {

    pub fn new(html: Chars<'a>) -> HTMLTokenizer<'a> {
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
                    return Token::Element(self.capture_element())
                }
                return Token::Text(lexeme);
            }

            if self.next_char == '\n' {
                if lexeme.is_empty() {
                    self.next_char();
                    return Token::NewLine;
                }
                return Token::Text(lexeme);
            }

            lexeme.push(self.next_char);
            self.next_char();
        }

        if lexeme.is_empty() {
            return Token::EOF;
        }

        lexeme.push(self.next_char);
        lexeme.push(self.html_pos.clone().last().unwrap());
        return Token::Text(lexeme);
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

            if self.next_char == '>' {
                return attributes;
            }

            match state {
                AttributeState::KEY => {
                    if self.next_char == '=' {
                        state = AttributeState::VALUE;
                        self.next_char();
                        self.next_char();
                    }
                    else {
                        key.push(self.next_char);
                        self.next_char();
                    }
                },
                AttributeState::VALUE => {
                    if self.next_char == '\"' {
                        state = AttributeState::KEY;
                        attributes.insert(key.clone(), value.clone());
                        self.next_char();
                        self.consume_whitespace();
                        key = String::new();
                        value = String::new();
                    }
                    else {
                        value.push(self.next_char);
                        self.next_char();
                    }
                }
            }
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

