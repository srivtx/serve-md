use ignore::WalkBuilder;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct FileIndex {
    pub files: HashMap<String, PathBuf>,
    pub dirs: HashMap<String, Vec<DirEntry>>,
}

pub struct DirEntry {
    pub name: String,
    pub url: String,
    pub is_dir: bool,
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
            dirs.entry(dir_url.clone()).or_insert_with(Vec::new);

            let parent_url = parent_dir_url(&url_path);
            dirs.entry(parent_url.clone())
                .or_insert_with(Vec::new)
                .push(DirEntry {
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
            dirs.entry(parent_url)
                .or_insert_with(Vec::new)
                .push(DirEntry {
                    name: file_name_from_url(&url),
                    url,
                    is_dir: false,
                });
        }
    }

    Ok(FileIndex { files, dirs })
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
