pub mod tuihtml;

use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};
use crate::tuihtml::widget::HtmlWidget;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {

    let html = r##"
<header>
    <h1>Garrett's Tech Blog</h1>
    <h3><em>A weekly dose of Rust, terminals, and open source</em></h3>
</header>

<hr>

<main>
    <article>
        <h2>Building an HTML Renderer for the Terminal</h2>
        <p>
            <em>By Garrett &mdash; <time datetime="2026-06-21">June 21, 2026</time></em>
        </p>
        <p>
            Ever wanted to <b>render HTML</b> directly in your terminal?
            It turns out you <u>can</u>&mdash;with a little help from
            <a href="https://ratatui.rs">Ratatui</a> and some creative use of
            Unicode box-drawing characters.
        </p>
        <p>
            The approach is <strong>surprisingly</strong> straightforward:
        </p>
        <ol>
            <li>Tokenize the raw HTML string into <code>Token</code>s</li>
            <li>Parse tokens into a flat list while tracking styling state</li>
            <li>Convert <em>styled spans</em> into Ratatui <code>Line</code>s</li>
            <li><s>Profit</s> <b>Render to the terminal!</b></li>
        </ol>
        <p>
            One gotcha: <del>nesting depth must be tracked manually</del> the
            parser uses an <abbr title="Last In First Out">LIFO</abbr> element
            stack to handle nested tags. This means
            <strong>bold inside italic <em>inside <u>underline</u></em></strong>
            all composes correctly.
        </p>
        <blockquote>
            <p>
                <em>"The terminal is not a limitation&mdash;it&apos;s a <u>canvas</u>."</em>
                <br>
                &mdash; <cite>Anonymous TUI Enthusiast</cite>
            </p>
        </blockquote>
        <figure>
            <img src="/assets/terminal-screenshot.png"></img>
            <figcaption>
                <small>A screenshot of the TUI HTML renderer in action.</small>
            </figcaption>
        </figure>
    </article>

    <hr>

    <article>
        <h2>Why RSS Still Matters in <data value="2026">2026</data></h2>
        <p>
            <em>By Garrett &mdash; <time datetime="2026-06-18">June 18, 2026</time></em>
        </p>
        <p>
            <span>RSS feeds</span> are the <strong>original decentralized
            web</strong>. No algorithms, no ads&mdash;just
            <u>content you choose</u>. Here is why I still use RSS:
        </p>
        <ul>
            <li>You own your reading list&mdash;no platform lock-in</li>
            <li>Full-text or summaries&mdash;<em>your</em> choice</li>
            <li>Works offline with any feed reader</li>
            <li><strong>Zero</strong> tracking by default</li>
        </ul>
        <p>
            If you are new to RSS, check out
            <a href="https://miniflux.app">Miniflux</a> or
            <a href="https://newsblur.com">NewsBlur</a> for a great
            self-hosted or managed experience.
        </p>
        <aside>
            <p>
                <b>Tip:</b> Pair your RSS reader with a
                <a href="https://readwise.io">read-later</a> service to build a
                personal knowledge base over time. <em>Your future self will
                thank you.</em>
            </p>
        </aside>
        <p>
            <small><em>Disclaimer: This post was <s>not</s> sponsored by Big
            RSS. <u>All opinions are my own.</u></em></small>
        </p>
    </article>
</main>

<hr>

<footer>
    <nav>
        <p>
            <b>Garrett&apos;s Tech Blog</b> &bull; Published weekly &bull;
            <a href="https://example.com/feed.xml">RSS Feed</a> &bull;
            <a href="https://example.com">Homepage</a>
        </p>
    </nav>
    <p>
        <img src="/assets/logo.png"></img>
        <small>Copyright &copy; 2026. <u>All wrongs reserved.</u></small>
    </p>
</footer>
"##;
    let html_widget = HtmlWidget::new(String::from(html));

    loop {
        terminal.draw(|f| render(f, &html_widget))?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame, html_widget: &HtmlWidget) {

    frame.render_widget(html_widget.get_paragraph(), frame.area());
}


