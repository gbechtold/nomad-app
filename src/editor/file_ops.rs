use super::core::Editor;
use std::fs;
use std::io;

pub trait FileOps {

    fn save_file(&mut self) -> io::Result<()>;
    fn set_filename(&mut self, filename: String);
}

impl FileOps for Editor {

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