use crate::markdown::{RenderedMarkdown, TocEntry};
use crate::resolve::encode_url_path;
use crate::scan::DirEntry;

pub fn wrap_html(rendered: &RenderedMarkdown, _path: &str) -> String {
    let title = rendered.title.as_deref().unwrap_or("serve-md");
    let toc_html = build_toc_html(&rendered.toc);

    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <style>{CSS}</style>
</head>
<body>
    <div class="container">
        <nav class="toc">
            <div class="toc-header">Contents</div>
            {toc_html}
        </nav>
        <main class="content">
            {content}
        </main>
    </div>
</body>
</html>"##,
        title = title,
        CSS = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/style.css")),
        toc_html = toc_html,
        content = rendered.html
    )
}

pub fn directory_listing(path: &str, entries: &[DirEntry]) -> String {
    let mut items = String::new();

    if path != "/" {
        let p = path.trim_end_matches('/');
        let parent = if let Some(pos) = p.rfind('/') {
            if pos == 0 {
                "/".to_string()
            } else {
                format!("{}/", &p[..pos])
            }
        } else {
            "/".to_string()
        };
        items.push_str(&format!(
            r#"<li><a href="{}" class="dir-entry parent-dir">../</a></li>"#,
            encode_url_path(&parent)
        ));
    }

    let mut dirs: Vec<_> = entries.iter().filter(|e| e.is_dir).collect();
    let mut files: Vec<_> = entries.iter().filter(|e| !e.is_dir).collect();

    dirs.sort_by(|a, b| a.name.cmp(&b.name));
    files.sort_by(|a, b| a.name.cmp(&b.name));

    for entry in &dirs {
        items.push_str(&format!(
            r#"<li><a href="{}" class="dir-entry dir">&#128193; {}/</a></li>"#,
            encode_url_path(&entry.url), entry.name
        ));
    }

    for entry in &files {
        items.push_str(&format!(
            r#"<li><a href="{}" class="dir-entry file">&#128196; {}</a></li>"#,
            encode_url_path(&entry.url), entry.name
        ));
    }

    let title = if path == "/" {
        "Index"
    } else {
        path.trim_end_matches('/')
    };

    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Directory: {title}</title>
    <style>{CSS}</style>
</head>
<body>
    <div class="container">
        <main class="content">
            <h1>{title}</h1>
            <ul class="dir-list">{items}</ul>
        </main>
    </div>
</body>
</html>"##,
        title = title,
        CSS = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/style.css")),
        items = items
    )
}

fn build_toc_html(toc: &[TocEntry]) -> String {
    if toc.is_empty() {
        return String::new();
    }

    let mut html = String::from("<ul class='toc-list'>");
    for entry in toc {
        html.push_str(&format!(
            r##"<li class="toc-level-{level}"><a href="#{id}">{text}</a></li>"##,
            level = entry.level,
            id = entry.id,
            text = html_escape(&entry.text)
        ));
    }
    html.push_str("</ul>");
    html
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
