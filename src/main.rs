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
<body>
    <h1>Heading Level 1 - Main Title</h1>
    <h2>Heading Level 2 - Section Title</h2>
    <h3>Heading Level 3 - Subsection Title</h3>
    <h4>Heading Level 4 - Minor Heading</h4>
    <h5>Heading Level 5 - Small Heading</h5>
    <h6>Heading Level 6 - Smallest Heading</h6>

    <hr>

    <p>This is a normal paragraph with <b>bold text</b> for emphasis.</p>
    <p>This paragraph contains <i>italicized text</i> for styling.</p>
    <p>You can also use <strong>strong importance</strong> and <em>emphasized text</em>.</p>
    <p>Add <u>underlined text</u> or <s>strikethrough text</s> for different effects.</p>
    <p>Display <code>inline code snippets</code> or <kbd>keyboard input</kbd> for technical content.</p>
    <p><small>Smaller text</small> for fine print, <big>larger text</big> for emphasis, and <mark>marked text</mark> for highlighting.</p>

    <hr>

    <h2>Unordered List</h2>
    <ul>
        <li>First list item</li>
        <li>Second list item</li>
        <ul>
            <li>Nested item 1</li>
            <li>Nested item 2</li>
        </ul>
        <li>Third list item</li>
    </ul>

    <h2>Ordered List</h2>
    <ol>
        <li>First step</li>
        <li>Second step</li>
        <li>Third step</li>
        <ol>
            <li>Nested step A</li>
            <li>Nested step B</li>
        </ol>
    </ol>

    <h2>Mixed List</h2>
    <ol>
        <li>Note 1</li>
        <li>Node 2</li>
        <li>Note 3</li>
        <ul>
            <li>Step 1</li>
            <li>Step 2</li>
        </ul>
        <li>Note 4</li>
    </ol>

    <hr>

    <h2>Block Level Containers</h2>

    <blockquote>
        <p>A blockquote represents a section that is quoted from another source.</p>
        <p>It is typically indented to show it's separate from the main content.</p>
    </blockquote>

    <hr>

    <h2>Inline Elements</h2>
    <p>
        <span>Span is an inline container</span> used to group text without affecting layout.
        You can also use <a href="\#">anchor links</a> for navigation.
        Line breaks <br> can be inserted to force text onto a new line.
        <img src="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 24 24'%3E%3Ccircle cx='12' cy='12' r='10' fill='%23333'/%3E%3C/svg%3E" alt="inline image">
        Images can be placed inline with text.
    </p>

    <hr>

    <h2>Semantic Structure Elements</h2>

    <header>
        <h3>Header Section</h3>
        <p>Introductory content for a section or page.</p>
    </header>

    <nav>
        <h3>Navigation Section</h3>
        <a href="\#">Home</a> | <a href="\#">About</a> | <a href="\#">Contact</a>
    </nav>

    <main>
        <h3>Main Content Area</h3>
        <article>
            <h4>Article Title</h4>
            <p>Independent, self-contained content that could be distributed separately.</p>
            <p>Articles can contain their own headings, paragraphs, and other elements.</p>
        </article>

        <section>
            <h4>Section Title</h4>
            <p>Thematic grouping of content, typically with a heading.</p>
            <p>Sections help organize content into logical parts.</p>
        </section>

        <aside>
            <h4>Aside Content</h4>
            <p>Tangentially related content like sidebars or pull quotes.</p>
            <p>This content is related to the main content but not essential.</p>
        </aside>
    </main>

    <footer>
        <h3>Footer Section</h3>
        <p>Copyright information, author details, and related links.</p>
        <p>Contact: <a href="mailto:example@example.com">example@example.com</a></p>
    </footer>

    <hr>

    <h2>Contact Information</h2>
    <address>
        Written by John Doe<br>
        Visit us at:<br>
        example.com<br>
        Box 564, Disneyland<br>
        USA
    </address>

    <hr>

    <h2>Figures with Captions</h2>
    <figure>
        <img src="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='200' height='100' viewBox='0 0 200 100'%3E%3Crect width='200' height='100' fill='%23cccccc'/%3E%3Ctext x='50%25' y='50%25' text-anchor='middle' dy='.3em' fill='%23333'%3ESample Image%3C/text%3E%3C/svg%3E" alt="sample figure">
        <figcaption>Figure 1: A sample image with a caption.</figcaption>
    </figure>

    <hr>
</body>\n
"#;

//     let html = r#"
// <b> Hello World </b>
//     "#;

//     let html = r#"
// <h1>title</h1>
// <p>
//     <i><b>hello</b> world</i>
// </p>
// <a href="https://www.google.com">google</a>
// <img src="/home/garrett/documents/image.jpg"></img>"#;
    frame.render_widget(construct_widget(String::from(html)), frame.area());
}


