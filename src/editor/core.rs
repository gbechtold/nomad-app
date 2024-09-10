use super::file_ops::FileOps;
use super::ui::UI;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::io;

pub struct Editor {
    pub content: Vec<String>,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub filename: Option<String>,
    pub status_message: String,
}

impl Editor {
    pub fn new() -> io::Result<Self> {
        Ok(Editor {
            content: vec![String::new()],
            cursor_x: 0,
            cursor_y: 0,
            filename: None,
            status_message: String::new(),
        })
    }

    pub fn run(&mut self) -> io::Result<()> {
        UI::init()?;
        loop {
            UI::refresh_screen(self)?;
            if self.process_keypress()? {
                break;
            }
        }
        UI::cleanup()?;
        Ok(())
    }

    fn process_keypress(&mut self) -> io::Result<bool> {
        match event::read()? {
            Event::Key(KeyEvent { code, modifiers, .. }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => return Ok(true),
                (KeyCode::Char('s'), KeyModifiers::CONTROL) => self.save_file()?,
                (KeyCode::Char('o'), KeyModifiers::CONTROL) => self.prompt_filename()?,
                (KeyCode::Char('g'), KeyModifiers::CONTROL) => self.move_cursor_to_end(),
                (KeyCode::Char('a'), KeyModifiers::CONTROL) => self.move_cursor_to_start(),
                (KeyCode::Char('k'), KeyModifiers::CONTROL) => self.cut_line(),
                (KeyCode::Char('u'), KeyModifiers::CONTROL) => self.paste_line(),
                (KeyCode::Char(c), _) => self.insert_char(c),
                (KeyCode::Enter, _) => self.insert_newline(),
                (KeyCode::Left, _) => self.move_cursor_left(),
                (KeyCode::Right, _) => self.move_cursor_right(),
                (KeyCode::Up, _) => self.move_cursor_up(),
                (KeyCode::Down, _) => self.move_cursor_down(),
                (KeyCode::Backspace, _) => self.delete_char(),
                _ => {}
            },
            _ => {}
        }
        Ok(false)
    }

    fn insert_char(&mut self, c: char) {
        if self.cursor_y == self.content.len() {
            self.content.push(String::new());
        }
        self.content[self.cursor_y].insert(self.cursor_x, c);
        self.move_cursor_right();
    }

    fn insert_newline(&mut self) {
        if self.cursor_x == self.content[self.cursor_y].len() {
            self.content.insert(self.cursor_y + 1, String::new());
        } else {
            let new_line = self.content[self.cursor_y].split_off(self.cursor_x);
            self.content.insert(self.cursor_y + 1, new_line);
        }
        self.cursor_y += 1;
        self.cursor_x = 0;
    }

    fn delete_char(&mut self) {
        if self.cursor_x > 0 {
            self.content[self.cursor_y].remove(self.cursor_x - 1);
            self.cursor_x -= 1;
        } else if self.cursor_y > 0 {
            let line = self.content.remove(self.cursor_y);
            self.cursor_y -= 1;
            self.cursor_x = self.content[self.cursor_y].len();
            self.content[self.cursor_y].push_str(&line);
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        } else if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.cursor_x = self.content[self.cursor_y].len();
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_y < self.content.len() && self.cursor_x < self.content[self.cursor_y].len() {
            self.cursor_x += 1;
        } else if self.cursor_y < self.content.len() - 1 {
            self.cursor_y += 1;
            self.cursor_x = 0;
        }
    }

    fn move_cursor_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.cursor_x = self.cursor_x.min(self.content[self.cursor_y].len());
        }
    }

    fn move_cursor_down(&mut self) {
        if self.cursor_y < self.content.len() - 1 {
            self.cursor_y += 1;
            self.cursor_x = self.cursor_x.min(self.content[self.cursor_y].len());
        }
    }

    fn move_cursor_to_end(&mut self) {
        self.cursor_y = self.content.len() - 1;
        self.cursor_x = self.content[self.cursor_y].len();
    }

    fn move_cursor_to_start(&mut self) {
        self.cursor_y = 0;
        self.cursor_x = 0;
    }

    fn cut_line(&mut self) {
        if self.cursor_y < self.content.len() {
            self.content.remove(self.cursor_y);
            if self.content.is_empty() {
                self.content.push(String::new());
            }
            if self.cursor_y == self.content.len() {
                self.cursor_y -= 1;
            }
            self.cursor_x = 0;
        }
    }

    fn paste_line(&mut self) {
        // This is a simplified paste. In a real implementation, you'd store the cut line.
        self.content.insert(self.cursor_y, String::new());
        self.cursor_y += 1;
    }

    pub fn get_content(&self) -> String {
        self.content.join("\n")
    }
}