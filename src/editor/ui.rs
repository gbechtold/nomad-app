use super::core::Editor;
use crossterm::{
    cursor,
    execute,
    terminal::{self, ClearType},
};
use std::io::{self, stdout, Write};

pub struct UI;

impl UI {
    pub fn init() -> io::Result<()> {
        terminal::enable_raw_mode()
    }

    pub fn cleanup() -> io::Result<()> {
        terminal::disable_raw_mode()
    }

    pub fn refresh_screen(editor: &Editor) -> io::Result<()> {
        execute!(
            stdout(),
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        for line in &editor.content {
            println!("{}\r", line);
        }

        let status = format!(
            "{} | Ctrl-Q: Quit, Ctrl-S: Save, Ctrl-O: Open",
            editor.status_message
        );
        execute!(
            stdout(),
            cursor::MoveTo(0, (editor.content.len() + 1) as u16),
            terminal::Clear(ClearType::CurrentLine)
        )?;
        print!("{}\r", status);

        execute!(
            stdout(),
            cursor::MoveTo(editor.cursor_x as u16, editor.cursor_y as u16)
        )?;
        
        io::stdout().flush()
    }

    pub fn prompt(editor: &Editor, prompt: &str) -> io::Result<String> {
        execute!(stdout(), cursor::MoveTo(0, editor.content.len() as u16))?;
        print!("{}", prompt);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }
}