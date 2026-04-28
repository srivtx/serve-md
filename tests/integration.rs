use serve_md::{
    markdown::render_markdown,
    resolve::{resolve, ResolveResult},
    scan::scan_directory,
};
use std::collections::HashSet;
use std::io::Write;

fn create_test_dir() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    let mut readme = std::fs::File::create(root.join("README.md")).unwrap();
    write!(
        readme,
        "# Test Project\n\nThis is the README.\n\n## Section 1\n\nContent here.\n"
    )
    .unwrap();

    std::fs::create_dir(root.join("guide")).unwrap();
    let mut guide = std::fs::File::create(root.join("guide").join("setup.md")).unwrap();
    write!(
        guide,
        "# Setup Guide\n\nRun `serve-md` to start.\n\n- [x] Install\n- [ ] Configure\n"
    )
    .unwrap();

    let mut img = std::fs::File::create(root.join("logo.png")).unwrap();
    img.write_all(&[0x89, 0x50, 0x4E, 0x47]).unwrap();

    let mut hidden = std::fs::File::create(root.join(".secret")).unwrap();
    write!(hidden, "secret").unwrap();

    let mut gitignore = std::fs::File::create(root.join(".gitignore")).unwrap();
    write!(gitignore, ".secret\n").unwrap();

    std::fs::create_dir(root.join(".git")).unwrap();

    dir
}

#[test]
fn test_scan_finds_markdown_and_assets() {
    let dir = create_test_dir();
    let index = scan_directory(dir.path()).unwrap();

    assert!(index.files.contains_key("/README"));
    assert!(index.files.contains_key("/guide/setup"));
    assert!(index.files.contains_key("/logo.png"));

    // .secret should be ignored by gitignore
    assert!(!index.files.contains_key("/.secret"));
    assert!(!index
        .files
        .values()
        .any(|p| p.file_name() == Some(std::ffi::OsStr::new(".secret"))));

    assert!(index.dirs.contains_key("/"));
    assert!(index.dirs.contains_key("/guide/"));
}

#[test]
fn test_scan_directory_entries() {
    let dir = create_test_dir();
    let index = scan_directory(dir.path()).unwrap();

    let root_entries = index.dirs.get("/").unwrap();
    let names: HashSet<_> = root_entries.iter().map(|e| e.name.as_str()).collect();

    assert!(names.contains("guide"));
    assert!(names.contains("README"));
    assert!(names.contains("logo.png"));
}

#[test]
fn test_resolve_root_readme() {
    let dir = create_test_dir();
    let index = scan_directory(dir.path()).unwrap();

    match resolve(&index, "/") {
        ResolveResult::Markdown(path) => {
            assert!(path.file_name() == Some(std::ffi::OsStr::new("README.md")));
        }
        other => panic!("Expected Markdown, got {:?}", other),
    }
}

#[test]
fn test_resolve_markdown_file() {
    let dir = create_test_dir();
    let index = scan_directory(dir.path()).unwrap();

    match resolve(&index, "/guide/setup") {
        ResolveResult::Markdown(path) => {
            assert!(path.file_name() == Some(std::ffi::OsStr::new("setup.md")));
        }
        other => panic!("Expected Markdown, got {:?}", other),
    }
}

#[test]
fn test_resolve_static_file() {
    let dir = create_test_dir();
    let index = scan_directory(dir.path()).unwrap();

    match resolve(&index, "/logo.png") {
        ResolveResult::Static(path) => {
            assert!(path.file_name() == Some(std::ffi::OsStr::new("logo.png")));
        }
        other => panic!("Expected Static, got {:?}", other),
    }
}

#[test]
fn test_resolve_md_redirect() {
    let dir = create_test_dir();
    let index = scan_directory(dir.path()).unwrap();

    match resolve(&index, "/guide/setup.md") {
        ResolveResult::Redirect(to) => {
            assert_eq!(to, "/guide/setup");
        }
        other => panic!("Expected Redirect, got {:?}", other),
    }
}

#[test]
fn test_resolve_directory_listing() {
    let dir = create_test_dir();
    let index = scan_directory(dir.path()).unwrap();

    match resolve(&index, "/guide/") {
        ResolveResult::Directory(path) => {
            assert_eq!(path, "/guide/");
        }
        other => panic!("Expected Directory, got {:?}", other),
    }
}

#[test]
fn test_resolve_not_found() {
    let dir = create_test_dir();
    let index = scan_directory(dir.path()).unwrap();

    match resolve(&index, "/does-not-exist") {
        ResolveResult::NotFound => {}
        other => panic!("Expected NotFound, got {:?}", other),
    }
}

#[test]
fn test_render_markdown_produces_html() {
    let input = "# Hello\n\nThis is **bold**.\n";
    let rendered = render_markdown(input);

    assert!(rendered.html.contains("<h1>"));
    assert!(rendered.html.contains("Hello"));
    assert!(rendered.html.contains("<strong>"));
    assert_eq!(rendered.title, Some("Hello".to_string()));
    assert!(!rendered.toc.is_empty());
}

#[test]
fn test_render_markdown_toc_levels() {
    let input = "# A\n## B\n### C\n";
    let rendered = render_markdown(input);

    assert_eq!(rendered.toc.len(), 3);
    assert_eq!(rendered.toc[0].level, 1);
    assert_eq!(rendered.toc[1].level, 2);
    assert_eq!(rendered.toc[2].level, 3);
}

#[test]
fn test_render_markdown_tasklists() {
    let input = "- [x] done\n- [ ] not done\n";
    let rendered = render_markdown(input);

    assert!(rendered.html.contains("checkbox"));
}
