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

    pub fn insert_char(&mut self, c: char) {
        if self.cursor_y == self.content.len() {
            self.content.push(String::new());
        }
        self.content[self.cursor_y].insert(self.cursor_x, c);
        self.move_cursor_right();
    }

    pub fn insert_newline(&mut self) {
        if self.cursor_x == self.content[self.cursor_y].len() {
            self.content.insert(self.cursor_y + 1, String::new());
        } else {
            let new_line = self.content[self.cursor_y].split_off(self.cursor_x);
            self.content.insert(self.cursor_y + 1, new_line);
        }
        self.cursor_y += 1;
        self.cursor_x = 0;
    }

    pub fn delete_char(&mut self) {
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

    pub fn move_cursor_left(&mut self) {
        if self.cursor_x > 0 {
            self.cursor_x -= 1;
        } else if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.cursor_x = self.content[self.cursor_y].len();
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_y < self.content.len() && self.cursor_x < self.content[self.cursor_y].len() {
            self.cursor_x += 1;
        } else if self.cursor_y < self.content.len() - 1 {
            self.cursor_y += 1;
            self.cursor_x = 0;
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
            self.cursor_x = self.cursor_x.min(self.content[self.cursor_y].len());
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor_y < self.content.len() - 1 {
            self.cursor_y += 1;
            self.cursor_x = self.cursor_x.min(self.content[self.cursor_y].len());
        }
    }

    pub fn get_content(&self) -> String {
        self.content.join("\n")
    }
}