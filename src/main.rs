pub mod tuihtml;

use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};
use crate::tuihtml::{parser::construct_widget, widget::HTMLWidget};

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

//             let html = r#"
// <body>
//     <h1>Heading Level 1 - Main Title</h1>
//     <h2>Heading Level 2 - Section Title</h2>
//     <h3>Heading Level 3 - Subsection Title</h3>
//     <h4>Heading Level 4 - Minor Heading</h4>
//     <h5>Heading Level 5 - Small Heading</h5>
//     <h6>Heading Level 6 - Smallest Heading</h6>
//
//     <hr>
//
//     <p>This is a normal paragraph with <b>bold text</b> for emphasis.</p>
//     <p>This paragraph contains <i>italicized text</i> for styling.</p>
//     <p>You can also use <strong>strong importance</strong> and <em>emphasized text</em>.</p>
//     <p>Add <u>underlined text</u> or <s>strikethrough text</s> for different effects.</p>
//     <p>Display <code>inline code snippets</code> or <kbd>keyboard input</kbd> for technical content.</p>
//     <p><small>Smaller text</small> for fine print, <big>larger text</big> for emphasis, and <mark>marked text</mark> for highlighting.</p>
//
//     <hr>
//
//     <h2>Unordered List</h2>
//     <ul>
//         <li>First list item</li>
//         <li>Second list item</li>
//         <ul>
//             <li>Nested item 1</li>
//             <li>Nested item 2</li>
//         </ul>
//         <li>Third list item</li>
//     </ul>
//
//     <h2>Ordered List</h2>
//     <ol>
//         <li>First step</li>
//         <li>Second step</li>
//         <li>Third step</li>
//         <ol>
//             <li>Nested step A</li>
//             <li>Nested step B</li>
//         </ol>
//     </ol>
//
//     <h2>Mixed List</h2>
//     <ol>
//         <li>Note 1</li>
//         <li>Node 2</li>
//         <li>Note 3</li>
//         <ul>
//             <li>Step 1</li>
//             <li>Step 2</li>
//         </ul>
//         <li>Note 4</li>
//     </ol>
//
//     <hr>
//
//     <h2>Block Level Containers</h2>
//
//     <blockquote>
//         <p>A blockquote represents a section that is quoted from another source.</p>
//         <p>It is typically indented to show it's separate from the main content.</p>
//     </blockquote>
//
//     <hr>
//
//     <h2>Inline Elements</h2>
//     <p>
//         <span>Span is an inline container</span> used to group text without affecting layout.
//         You can also use <a href="\#">anchor links</a> for navigation.
//         Line breaks <br> can be inserted to force text onto a new line.
//         <img src="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='16' height='16' viewBox='0 0 24 24'%3E%3Ccircle cx='12' cy='12' r='10' fill='%23333'/%3E%3C/svg%3E" alt="inline image">
//         Images can be placed inline with text.
//     </p>
//
//     <hr>
//
//     <h2>Semantic Structure Elements</h2>
//
//     <header>
//         <h3>Header Section</h3>
//         <p>Introductory content for a section or page.</p>
//     </header>
//
//     <nav>
//         <h3>Navigation Section</h3>
//         <a href="\#">Home</a> | <a href="\#">About</a> | <a href="\#">Contact</a>
//     </nav>
//
//     <main>
//         <h3>Main Content Area</h3>
//         <article>
//             <h4>Article Title</h4>
//             <p>Independent, self-contained content that could be distributed separately.</p>
//             <p>Articles can contain their own headings, paragraphs, and other elements.</p>
//         </article>
//
//         <section>
//             <h4>Section Title</h4>
//             <p>Thematic grouping of content, typically with a heading.</p>
//             <p>Sections help organize content into logical parts.</p>
//         </section>
//
//         <aside>
//             <h4>Aside Content</h4>
//             <p>Tangentially related content like sidebars or pull quotes.</p>
//             <p>This content is related to the main content but not essential.</p>
//         </aside>
//     </main>
//
//     <footer>
//         <h3>Footer Section</h3>
//         <p>Copyright information, author details, and related links.</p>
//         <p>Contact: <a href="mailto:example@example.com">example@example.com</a></p>
//     </footer>
//
//     <hr>
//
//     <h2>Contact Information</h2>
//     <address>
//         Written by John Doe<br>
//         Visit us at:<br>
//         example.com<br>
//         Box 564, Disneyland<br>
//         USA
//     </address>
//
//     <hr>
//
//     <h2>Figures with Captions</h2>
//     <figure>
//         <img src="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='200' height='100' viewBox='0 0 200 100'%3E%3Crect width='200' height='100' fill='%23cccccc'/%3E%3Ctext x='50%25' y='50%25' text-anchor='middle' dy='.3em' fill='%23333'%3ESample Image%3C/text%3E%3C/svg%3E" alt="sample figure">
//         <figcaption>Figure 1: A sample image with a caption.</figcaption>
//     </figure>
//
//     <hr>
// </body>\n
// "#;

    let html = r#"
<p>Hey everyone!</p><p>This is the list of all the changes we've done to our projects during the month of March.</p> <img src="../media/content/videos/march_2026.jpg" alt="the familiar from the game Donsol is sitting on Doctor Claws desk where Mad Cat should be, looking content while being petted by the armored hand of the Inspector Gadget villain" loading="lazy" width="1200" height="679"> <ul> <li><b>Wiki</b>, documented the replacement of Pino's <a href="../site/standing_rigging_replacement.html#mar2026">backstay</a>, and created a project page for <a href="../site/no_bears_none.html">No Bears, None</a>.</li> <li><b><a href="https://kokorobot.ca/site/hakum.html">Hakum</a></b>, completed <a href="https://kokorobot.ca/media/content/hakum_sabo_10.jpg" target="_blank">page 10</a> of <a href="https://kokorobot.ca/site/sabotage_study.html" target="_blank">Sabotage Study</a>.</li> <li><b><a href="../site/rabbit_waves.html">Rabbit Waves</a></b>, added a <a href="https://rabbitwaves.ca/media/comic/coffeep1.jpg" target="_blank">comic page</a>.</li> <li><b><a href="../site/grimgrains.html">Grimgrains</a></b>, updated lactofermentation section about <a href="https://grimgrains.com/site/lactofermentation.html#jars">jars</a>.</li> <li><b><a href="../site/oquonie.html">Oquonie</a></b>, released a new version with lots of tiny optimizations.</li> <li><b><a href="../site/uxn.html">Uxn</a></b>, released a little <a href='https://wiki.xxiivv.com/site/m5' target='_blank'>emulator for the m5 stack</a>.</li> </ul> <p>We spent a big chunk of the month finishing up the new <a href="../site/donsol.html">Donsol</a>. Devine <a href="https://rabbits.srht.site/decadv/" target="_blank">documented</a> the development as part of a spring edition of <a href="https://eli.li/december-adventure" target="_blank">December Adventure</a>. All of the graphics are done and the game is fully playable, all we have left to do is to test for bugs and to write the music. It looks <a href="https://rabbits.srht.site/days/2026/03/29.html" target="_blank">fantastic</a>. We'll let you all know when we have a release date!</p> <p>The goal with <a href="https://rabbitwaves.ca">Rabbit Waves</a> was always to use storytelling to teach analog seafaring skills by making it possible to navigate between different topics by selecting highlighted objects in comic strips. Rek completed the <a href="https://rabbitwaves.ca/media/comic/coffeep1.jpg" target="_blank">first page</a>, featuring a rabbit sailor's morning coffee-making routine while living on a small turnip sailboat at sea. This comic is the first step towards that goal of building a larger world for Rabbit Waves.</p> <p>Spring is here and we are catching up with boat maintenance. While doing checks aboard last fall we noticed that Pino's <a href="../site/standing_rigging_replacement.html#mar2026">backstay</a> had a cut, we replaced the stay this week and plan to do the same for the headstay.</p> <p>This month, we also finished proofreading the revised version of the <a href="../site/victoria_to_sitka_logbook.html">Victoria to sitka logbook</a>, which we will now refer to as <a href="../site/no_bears_none.html">No Bears, None</a>. We still have work to do on it, but we hope to release it later this summer, or early fall. Lastly, we'd like to thank everyone who purchased <a href="../site/store.html">rabbit stickers</a> last month! Devine continues to draw very <a href="https://rabbits.srht.site/days/2026/03/31.html" target="_blank">beautiful spirographs</a> onto each letter.</p> <p><b>Book Club:</b> We are reading <i>Island</i> by Aldous Huxley.</p>
"#;
//     let html = r#"
//  <p>Breht reads and reacts to "A Letter To The American People" written by Masoud Pezeschkian, the Iranian president, as a ground invasion of some sort seems imminent. Check out our new design in collaboration with <em>Goods for the People</em> <a href="https://goodsforthepeople.com/">HERE</a></p> <p> </p> <p> -------------------------------------------------------------------------------</p> <p>Support Rev Left and get bonus episodes on <a href= "https://www.patreon.com/revleftradio">Patreon</a></p> <p>Make a one-time donation to Rev Left at <a href= "https://buymeacoffee.com/revleftradio">BuyMeACoffee.com/revleftradio</a></p> <p>Follow RLR on IG <a href= "https://www.instagram.com/rev_left_radio_official/" rel= "noopener noreferrer nofollow">HERE</a></p> <p>Learn more about Rev Left <a href= "https://revleftradio.com/">HERE</a></p>
// "#;
    let html_widget = HTMLWidget::new(String::from(html));
    frame.render_widget(html_widget.get_paragraph(), frame.area());
}


