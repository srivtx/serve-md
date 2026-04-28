use ignore::WalkBuilder;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct FileIndex {
    pub files: HashMap<String, PathBuf>,
    pub dirs: HashMap<String, Vec<DirEntry>>,
    pub search_index: SearchIndex,
}

pub struct DirEntry {
    pub name: String,
    pub url: String,
    pub is_dir: bool,
}

pub struct SearchIndex {
    /// word -> list of (url, title, score)
    pub index: HashMap<String, Vec<SearchDoc>>,
}

#[derive(Clone)]
pub struct SearchDoc {
    pub url: String,
    pub title: String,
    pub preview: String,
}

pub fn scan_directory(root: &Path) -> anyhow::Result<FileIndex> {
    let root = root.canonicalize()?;
    let mut files = HashMap::new();
    let mut dirs: HashMap<String, Vec<DirEntry>> = HashMap::new();

    dirs.insert("/".to_string(), Vec::new());

    let walker = WalkBuilder::new(&root)
        .hidden(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .follow_links(false)
        .build();

    for entry in walker {
        let entry = entry?;
        let path = entry.path();

        if path == root.as_path() {
            continue;
        }

        if is_hidden(path) {
            continue;
        }

        let relative = path.strip_prefix(&root)?;
        let url_path = path_to_url(relative);

        if path.is_dir() {
            let dir_url = format!("{}/", url_path);
            dirs.entry(dir_url.clone()).or_default();

            let parent_url = parent_dir_url(&url_path);
            dirs.entry(parent_url.clone()).or_default().push(DirEntry {
                name: file_name_from_url(&url_path),
                url: dir_url,
                is_dir: true,
            });
        } else if path.is_file() {
            let url = if path.extension() == Some(std::ffi::OsStr::new("md")) {
                url_path
                    .strip_suffix(".md")
                    .map(String::from)
                    .unwrap_or_else(|| url_path.clone())
            } else {
                url_path.clone()
            };

            files.insert(url.clone(), path.to_path_buf());

            let parent_url = parent_dir_url(&url_path);
            dirs.entry(parent_url).or_default().push(DirEntry {
                name: file_name_from_url(&url),
                url,
                is_dir: false,
            });
        }
    }

    let mut search_index = SearchIndex {
        index: HashMap::new(),
    };

    for (url, path) in &files {
        if path.extension() == Some(std::ffi::OsStr::new("md")) {
            if let Ok(content) = std::fs::read_to_string(path) {
                let title = extract_title(&content);
                let preview = content.lines().next().unwrap_or("").to_string();
                let words = tokenize(&content);
                for word in words {
                    let docs = search_index.index.entry(word).or_default();
                    if !docs.iter().any(|d| d.url == *url) {
                        docs.push(SearchDoc {
                            url: url.clone(),
                            title: title.clone(),
                            preview: preview.clone(),
                        });
                    }
                }
            }
        }
    }

    Ok(FileIndex {
        files,
        dirs,
        search_index,
    })
}

fn path_to_url(relative: &Path) -> String {
    let mut s = String::new();
    for component in relative.components() {
        if let std::path::Component::Normal(os) = component {
            s.push('/');
            s.push_str(&os.to_string_lossy());
        }
    }
    if s.is_empty() {
        s.push('/');
    }
    s
}

fn parent_dir_url(url: &str) -> String {
    if let Some(pos) = url.rfind('/') {
        if pos == 0 {
            "/".to_string()
        } else {
            format!("{}/", &url[..pos])
        }
    } else {
        "/".to_string()
    }
}

fn file_name_from_url(url: &str) -> String {
    url.rfind('/')
        .map(|pos| &url[pos + 1..])
        .unwrap_or(url)
        .to_string()
}

fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}

fn extract_title(content: &str) -> String {
    content
        .lines()
        .find(|line| line.starts_with("# "))
        .map(|line| line.trim_start_matches("# ").trim().to_string())
        .unwrap_or_else(|| "Untitled".to_string())
}

fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && c != ' ', " ")
        .split_whitespace()
        .filter(|word| word.len() > 2)
        .map(String::from)
        .collect()
}
