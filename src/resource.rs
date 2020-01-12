use rust_embed::RustEmbed;
use std::string::FromUtf8Error;

#[derive(RustEmbed)]
#[folder = "res/"]
pub struct Resource;

impl Resource {
    pub fn from_file (filename: &str) -> Result<String, FromUtf8Error> {
        let file = Self::get(filename).unwrap();
        String::from_utf8(file.as_ref().to_vec())
    }
}
