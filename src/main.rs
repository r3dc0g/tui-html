mod tuihtml;

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::List,
};

use std::io::stdout;

use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {

    let html = r#"
        <p> Hello, World! </p>
        <ul>
           <li> Item 1 </li>
           <li> Item 2 </li>
           <li> Item 3 </li>
        </ul>
        <a href="https://www.google.com"> Click me! </a>
    "#;

    let tui = tuihtml::parse_html(html)?;

    run_tui(tui)?;

    Ok(())
}

fn run_tui(widget: List) -> Result<(), Box<dyn Error>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;


    loop {

        terminal.draw(|f| {
            let size = f.size();
            f.render_widget(widget.clone(), size);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    terminal.clear()?;
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
