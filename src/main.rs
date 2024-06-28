mod tuihtml;

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::Paragraph};
use tuihtml::parse_html;
use std::io::stdout;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {

    let html = r#"
        <p>Hey everyone! This is the list of all the changes we've done to our projects and apps during the month of January. We'll also be reporting in our on position in the world, and on our future plans.</p>

        <h2>Summary Of Changes</h2>

        <ul>
            <li><b>100r.co</b>, added <a href='https://100r.co/site/electrical_refit.html' target='_blank'>electrical refit</a>(portal for AC & DC electrical refit), and updated <a href='https://100r.co/site/lpg_refit.html' target='_blank'>LPG refit</a>.</li>
            <li><b><a href='https://100r.co/site/wiktopher.html' target='_blank'>Wiktopher</a></b>, corrected chapter 8, and nearly done with lexicon.</li>
            <li><b><a href='https://100r.co/site/weathering_software_winter.html' target='_blank'>Weathering software winter</a></b>, <a href='https://guide.handmade-seattle.com/c/2022/weathering-software-winter/' target='_blank'>video</a>(Vimeo) now has closed-captionning (same goes for <a href='https://youtu.be/9TJuOwy4aGA' target='_blank'>YouTube version</a>). Rek spent time cleaning up the auto-generated transcript.</li>
            <li><b><a href='https://100r.co/site/uxn.html' target='_blank'>Uxn</a></b>, Andrew released <a href='https://github.com/randrew/uxn32/releases/tag/2.0' target='_blank'>Uxn32 2.0</a>.</li>
            <li><b><a href='http://wiki.xxiivv.com/site/potato.html' target='_blank'>Potato</a></b>, added updates, potato can now assemble roms from tal files.</li>
        </ul>

        <h3>News</h3>

        <p>This month, we started porting <a href='https://100r.co/site/oquonie.html' target='_blank'>Oquonie</a> to Uxn. This is a long time coming, but we weren't sure if it was possible to do, and we still had a lot to learn before even thinking of taking it on. Now, we think we are ready. We are <a href='https://merveilles.town/@neauoire/109753175335217737' target='_blank'>re-drawing the sprites</a>(Mastodon), and they look amazing. This is an important test for us, and for Uxn.</p>

        <p>Here is a very adorable <a href='https://merveilles.town/@neauoire/109689586631044117' target='_blank'>little Uxn sprite</a> for Potato that comes up when a rom path was mistyped, see it also on the <a href='https://100r.co/site/uxn.html' target='_blank'>Uxn</a> page.</p>

        <h3>Pino book club</h3>

        <p>We're reading <b>The Journey Home: Some Words in Defense of the American West</b> by Edward Abbey.</p>

        <p><a href='https://100r.co/site/log.html#jan2023'>Continue Reading</a></p>
    "#;

    let html_view = parse_html(html)?;

    run_tui(html_view)?;

    Ok(())
}

fn run_tui(widget: Paragraph) -> Result<(), Box<dyn Error>> {
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
