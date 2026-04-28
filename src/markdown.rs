use pulldown_cmark::{html, Event, Options, Parser, Tag};

pub struct RenderedMarkdown {
    pub html: String,
    pub toc: Vec<TocEntry>,
    pub title: Option<String>,
}

pub struct TocEntry {
    pub level: u8,
    pub text: String,
    pub id: String,
}

pub fn render_markdown(content: &str) -> RenderedMarkdown {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let mut toc = Vec::new();
    let mut title = None;
    let mut in_heading = false;
    let mut current_level = 0u8;
    let mut current_text = String::new();

    for event in Parser::new_ext(content, options) {
        match event {
            Event::Start(Tag::Heading(level, _, _)) => {
                in_heading = true;
                current_level = level as u8;
                current_text.clear();
            }
            Event::End(Tag::Heading(_, _, _)) => {
                in_heading = false;
                let id = slugify(&current_text);
                if title.is_none() && current_level == 1 {
                    title = Some(current_text.clone());
                }
                toc.push(TocEntry {
                    level: current_level,
                    text: current_text.clone(),
                    id,
                });
            }
            Event::Text(text) if in_heading => {
                current_text.push_str(&text);
            }
            Event::Code(code) if in_heading => {
                current_text.push_str(&code);
            }
            _ => {}
        }
    }

    let parser = Parser::new_ext(content, options).map(|event| match event {
        Event::Html(text) => Event::Text(pulldown_cmark::CowStr::from(escape_html(&text))),
        other => other,
    });
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    RenderedMarkdown {
        html: html_output,
        toc,
        title,
    }
}

fn slugify(text: &str) -> String {
    text.to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && c != ' ', "")
        .replace(' ', "-")
}

fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
