use std::path::PathBuf;
use std::{fs, io};
use gtk::{Label, Image, IconTheme, ImageExt, IconThemeExt};

use super::grid::GridButton;

pub struct DesktopEntry {
    pub name: String,
    pub display_name: String,
    pub icon_path: Option<String>,
}

impl DesktopEntry {
    pub fn empty () -> Self {
        Self {
            name: String::from(""),
            display_name: String::from(""),
            icon_path: None,
        }
    }

    pub fn from_path (path: PathBuf) -> io::Result<Option<Self>> {
        let content = fs::read_to_string(path)?;

        let mut entry = Self::empty();
        let mut entry_section = false;

        for line in content.lines() {
            let mut chars = line.chars();

            if let Some(first_char) = chars.nth(0) {
                if first_char == '\0' || first_char == '#' {
                    continue;
                }
            }

            if entry_section {
                if let Some(first_char) = chars.nth(0) {
                    if first_char == '[' {
                        break;
                    }
                }

                let split = line.splitn(2, "=").map(String::from).collect::<Vec<_>>();

                let key = match split.get(0) {
                    Some(key) => key,
                    None => continue,
                };

                let value = match split.get(1) {
                    Some(value) => value,
                    None => continue,
                };

                if (key == "NoDisplay" || key == "Hidden") && value == "true" {
                    return Ok(None);
                }

                if key == "Name" {
                    entry.set_name(value);
                    continue;
                }

                if key == "Icon" {
                    entry.set_icon(value);
                    continue;
                }
            } else if line == "[Desktop Entry]" {
                entry_section = true;
            }
        }

        Ok(Some(entry))
    }

    pub fn get_dirs () -> Vec<PathBuf> {
        let mut dirs: Vec<PathBuf> = vec!["/usr/share/applications".into(), "/usr/local/share/applications".into()];

        if let Some(user_dir) = dirs::home_dir() {
            dirs.push(user_dir.join(".local/share/applications/"));
        }

        dirs
    }

    pub fn get_all () -> Vec<DesktopEntry> {
        let mut desktop_entries: Vec<DesktopEntry> = Vec::with_capacity(10);

        for dir in Self::get_dirs() {
            if let Ok(read_dir) = fs::read_dir(dir) {
                for entry in read_dir {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        let extension = path.extension();

                        match extension {
                            Some(extension) => {
                                if extension != "desktop" {
                                    continue;
                                }
                            }
                            None => continue,
                        }

                        if let Ok(desktop_entry) = Self::from_path(path) {
                            if let Some(desktop_entry) = desktop_entry {
                                desktop_entries.push(desktop_entry);
                            }
                        }
                    }
                }
            }
        }

        desktop_entries
    }

    pub fn set_name<S: Into<String>> (&mut self, name: S) {
        self.name = name.into();
        self.display_name = self.name.clone();
    }

    pub fn set_icon<S: Into<String>> (&mut self, icon: S) {
        self.icon_path = Some(icon.into());
    }
}

impl GridButton for DesktopEntry {
    fn label(&self) -> &String {
        &self.name
    }

    fn display_label(&self) -> Label {
        Label::new(Some(&self.display_name))
    }

    fn icon(&self) -> Image {
        if self.icon_path == None {
            // gtk::IconSize::Dialog is 48x48
            return Image::new_from_icon_name(Some("application-x-executable"), gtk::IconSize::Dialog);
        }

        let icon = self.icon_path.as_ref().unwrap();
        if PathBuf::from(&icon).exists() {
            let image = Image::new_from_file(&icon);

            // Scale down to 48x48
            if let Some(pixbuf) = image.get_pixbuf() {
                if let Some(pixbuf) = pixbuf.scale_simple(48, 48, gdk_pixbuf::InterpType::Tiles) {
                    return Image::new_from_pixbuf(Some(&pixbuf));
                }
            }
        }

        if let Some(theme) = IconTheme::get_default() {
            let info = theme.load_icon(&icon, 48, gtk::IconLookupFlags::FORCE_SIZE);

            if let Ok(info) = info {
                if  let Some(pixbuf) = info {
                    return Image::new_from_pixbuf(Some(&pixbuf));
                }
            }
        }

        // gtk::IconSize::Dialog is 48x48
        Image::new_from_icon_name(Some("application-x-executable"), gtk::IconSize::Dialog)
    }

    fn set_display_label(&mut self, label: String) {
        self.display_name = label;
    }
}

