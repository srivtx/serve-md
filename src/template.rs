use crate::markdown::{RenderedMarkdown, TocEntry};
use crate::resolve::encode_url_path;
use crate::scan::DirEntry;

const SEARCH_JS: &str = r#"
(function() {
    const input = document.getElementById('search-input');
    const results = document.getElementById('search-results');
    if (!input || !results) return;

    let debounce;
    input.addEventListener('input', function() {
        clearTimeout(debounce);
        const q = input.value.trim();
        if (q.length < 2) {
            results.innerHTML = '';
            results.style.display = 'none';
            return;
        }
        debounce = setTimeout(function() {
            fetch('/api/search?q=' + encodeURIComponent(q))
                .then(r => r.json())
                .then(data => {
                    if (data.results.length === 0) {
                        results.innerHTML = '<div class="search-no-results">No results</div>';
                    } else {
                        results.innerHTML = data.results.map(r =>
                            '<a href="' + r.url + '" class="search-result">' +
                            '<div class="search-result-title">' + (r.title || r.url) + '</div>' +
                            '<div class="search-result-preview">' + r.preview + '</div>' +
                            '</a>'
                        ).join('');
                    }
                    results.style.display = 'block';
                })
                .catch(() => {
                    results.innerHTML = '';
                    results.style.display = 'none';
                });
        }, 150);
    });

    document.addEventListener('click', function(e) {
        if (!input.contains(e.target) && !results.contains(e.target)) {
            results.style.display = 'none';
        }
    });
})();
"#;

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
    <div class="search-bar">
        <input type="text" id="search-input" placeholder="Search docs..." autocomplete="off">
        <div id="search-results"></div>
    </div>
    <div class="container">
        <nav class="toc">
            <div class="toc-header">Contents</div>
            {toc_html}
        </nav>
        <main class="content">
            {content}
        </main>
    </div>
    <script>{JS}</script>
</body>
</html>"##,
        title = title,
        CSS = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/static/style.css")),
        JS = SEARCH_JS,
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
            encode_url_path(&entry.url),
            entry.name
        ));
    }

    for entry in &files {
        items.push_str(&format!(
            r#"<li><a href="{}" class="dir-entry file">&#128196; {}</a></li>"#,
            encode_url_path(&entry.url),
            entry.name
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
