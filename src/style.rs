use std::path::PathBuf;
use std::fs;

use super::resource::Resource;

pub fn get_default_css(path_to_save: Option<PathBuf>) -> String {
//    let file = Resource::get("default_style.css").unwrap();
//    let content = String::from_utf8(file.as_ref().to_vec())

    let content = Resource::from_file("default_style.css")
        .expect("Cannot read default style");


    if let Some(path) = path_to_save {
        let _ = fs::write(path, &content);
    }

    content
}

pub fn get_css() -> String {
    let mut content = String::from("");
    if let Some(mut style_path) = dirs::config_dir() {
        style_path.push("waffy");
        style_path.push("style.css");

        if style_path.exists() {
            content = fs::read_to_string(style_path).expect("Could not read config");
        } else {
            content = get_default_css(Some(style_path))
        }
    }

    if content == "" {
        content = get_default_css(None);
    }

    // TODO: Check config option
    if let Some(mut pywal_path) = dirs::cache_dir() {
        pywal_path.push("wal");
        pywal_path.push("colors-waybar.css");

        if pywal_path.exists() {
            let mut pywal_content = fs::read_to_string(pywal_path).expect("Could not read config");
            pywal_content.push_str(content.as_str());
            content = pywal_content;
        }
    }

    content
}

