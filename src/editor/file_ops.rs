use super::core::Editor;
use std::fs;
use std::io;
use std::path::Path;

pub trait FileOps {
    fn load_file(&mut self, filename: &str) -> io::Result<()>;
    fn save_file(&mut self) -> io::Result<()>;
    fn prompt_filename(&mut self) -> io::Result<()>;
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

    fn prompt_filename(&mut self) -> io::Result<()> {
        let prompt = "Enter filename to save/open: ";
        let input = super::ui::UI::prompt(self, prompt)?;
        if !input.is_empty() {
            if Path::new(&input).exists() {
                self.load_file(&input)?;
            } else {
                self.filename = Some(input);
                self.status_message = "New file. Use Ctrl-S to save.".to_string();
            }
        }
        Ok(())
    }
}