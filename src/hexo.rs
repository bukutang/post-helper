use std::fs;
use std::process::Command;

/// New a post.
///
/// # Arguments
///
/// * `title` - The title of the post.
/// * `scaffold` - The category of the post. It should match the filename in scaffolds folder. Leave it empty string to use default post scaffold.
///
/// # Example
///
/// ```
/// let output = hexo::new("hello", "post");
/// ```
///
/// ```
/// let output = hexo::new("hello", "");
/// ```
///
pub fn new(title: &str, scaffold: &str) -> std::process::Output {
    Command::new("C:/Program Files/nodejs/hexo.cmd")
        .args(vec!["new", scaffold, title])
        .output()
        .expect("Failed to execute hexo new.")
}

/// Get all scaffolds in scaffolds folder.
pub fn get_scaffolds() -> Vec<String> {
    let mut scaffolds = Vec::new();
    for file in fs::read_dir("./scaffolds").unwrap() {
        let path = file.unwrap().path();
        let filename = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(".md", "");
        scaffolds.push(filename.to_string());
    }
    scaffolds
}
