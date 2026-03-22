pub mod tuihtml;

use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};
use crate::tuihtml::parser::construct_widget;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {

            let html = r#"
<h1>Title</h1>
<p>
    <b>Hello</b> World
</p>
<a href="https://www.google.com">Google</a>
<img src="/home/garrett/Documents/image.jpg"></img>"#;

    frame.render_widget(construct_widget(String::from(html)), frame.area());
}


