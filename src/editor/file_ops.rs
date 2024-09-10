use super::core::Editor;
use std::fs;
use std::io;

pub trait FileOps {
    fn load_file(&mut self, filename: &str) -> io::Result<()>;
    fn save_file(&mut self) -> io::Result<()>;
    fn set_filename(&mut self, filename: String);
}

impl FileOps for Editor {
    fn load_file(&mut self, filename: &str) -> io::Result<()> {
        let content = fs::read_to_string(filename)?;
        self.content = content.lines().map(String::from).collect();
        if self.content.is_empty() {
            self.content.push(String::new());
        }
        self.filename = Some(filename.to_string());
        self.status_message = format!("Loaded file: {}", filename);
        Ok(())
    }

    fn save_file(&mut self) -> io::Result<()> {
        if let Some(filename) = &self.filename {
            let content = self.content.join("\n");
            fs::write(filename, content)?;
            self.status_message = format!("Saved file: {}", filename);
        } else {
            self.status_message = "No filename set. Use Ctrl-O to set filename.".to_string();
        }
        Ok(())
    }

    fn set_filename(&mut self, filename: String) {
        self.filename = Some(filename);
        self.status_message = format!("File name set to: {}", self.filename.as_ref().unwrap());
    }
}