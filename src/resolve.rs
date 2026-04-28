use crate::scan::FileIndex;
use percent_encoding::{percent_decode_str, utf8_percent_encode, AsciiSet, CONTROLS};
use std::path::PathBuf;

#[derive(Debug)]
pub enum ResolveResult {
    Markdown(PathBuf),
    Static(PathBuf),
    Directory(String),
    Redirect(String),
    NotFound,
}

const PATH_SEGMENT_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'?')
    .add(b'`')
    .add(b'{')
    .add(b'}');

pub fn decode_url(url: &str) -> String {
    percent_decode_str(url).decode_utf8_lossy().to_string()
}

pub fn encode_url_path(path: &str) -> String {
    path.split('/')
        .map(|segment| {
            if segment.is_empty() {
                String::new()
            } else {
                utf8_percent_encode(segment, PATH_SEGMENT_ENCODE_SET).to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("/")
}

pub fn resolve(index: &FileIndex, incoming: &str) -> ResolveResult {
    let url_path = decode_url(incoming);
    let url_path = if url_path.is_empty() {
        String::from("/")
    } else {
        url_path
    };

    if let Some(stem) = url_path.strip_suffix(".md") {
        let canonical = if stem.is_empty() { "/" } else { stem };
        if index.files.contains_key(canonical)
            || index.dirs.contains_key(&format!("{}/", canonical))
        {
            return ResolveResult::Redirect(canonical.to_string());
        }
    }

    if url_path.ends_with('/') {
        if index.dirs.contains_key(&url_path) {
            let base = url_path.trim_end_matches('/');
            let readme = format!("{}/README", base);
            let index_md = format!("{}/index", base);

            if let Some(path) = index.files.get(&readme) {
                return ResolveResult::Markdown(path.clone());
            }
            if let Some(path) = index.files.get(&index_md) {
                return ResolveResult::Markdown(path.clone());
            }
            return ResolveResult::Directory(url_path.to_string());
        }
        return ResolveResult::NotFound;
    }

    if let Some(path) = index.files.get(&url_path) {
        let is_md = path.extension().map(|e| e == "md").unwrap_or(false);
        if is_md {
            return ResolveResult::Markdown(path.clone());
        } else {
            return ResolveResult::Static(path.clone());
        }
    }

    let with_md = format!("{}.md", url_path);
    if let Some(path) = index.files.get(&with_md) {
        return ResolveResult::Markdown(path.clone());
    }

    let as_dir = format!("{}/", url_path);
    if index.dirs.contains_key(&as_dir) {
        return ResolveResult::Redirect(as_dir);
    }

    ResolveResult::NotFound
}
