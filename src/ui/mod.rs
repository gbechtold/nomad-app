use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal,
    execute,
};
use std::{error::Error, io};
use tui::widgets::ListState;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

use crate::editor::{Editor, FileOps};
use crate::llm::LlamaModel;

pub struct UI {
    editor: Editor,
    llm: LlamaModel,
}

impl UI {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(UI {
            editor: Editor::new()?,
            llm: LlamaModel::new(),
        })
    }

    fn run_app<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            terminal.draw(|f| self.ui(f))?;
    
            if let Event::Key(key) = event::read()? {
                match (key.code, key.modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => return Ok(()),
                    (KeyCode::Char('s'), KeyModifiers::CONTROL) => self.editor.save_file()?,
                    (KeyCode::Char('o'), KeyModifiers::CONTROL) => self.prompt_filename(terminal)?,
                    (KeyCode::Char('l'), KeyModifiers::CONTROL) => self.process_llm(terminal)?,
                    (KeyCode::Left, _) => self.editor.move_cursor_left(),
                    (KeyCode::Right, _) => self.editor.move_cursor_right(),
                    (KeyCode::Up, _) => self.editor.move_cursor_up(),
                    (KeyCode::Down, _) => self.editor.move_cursor_down(),
                    (KeyCode::Char(c), _) => self.editor.insert_char(c),
                    (KeyCode::Backspace, _) => self.editor.delete_char(),
                    (KeyCode::Enter, _) => self.editor.insert_newline(),
                    _ => {}
                }
            }
        }
    }

    fn ui<B: Backend>(&self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(f.size());
    
        // Title bar
        let title = Paragraph::new("Nomad Editor")
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);
    
        // Content area
        let content: Vec<ListItem> = self
            .editor
            .content
            .iter()
            .enumerate()
            .map(|(i, line)| {
                let content = if i == self.editor.cursor_y {
                    let (before, after) = line.split_at(self.editor.cursor_x.min(line.len()));
                    Spans::from(vec![
                        Span::raw(before),
                        Span::styled(
                            if after.is_empty() { " " } else { &after[..1] },
                            Style::default().add_modifier(Modifier::REVERSED),
                        ),
                        Span::raw(if after.len() > 1 { &after[1..] } else { "" }),
                    ])
                } else {
                    Spans::from(Span::raw(line))
                };
                
                if i == self.editor.cursor_y {
                    ListItem::new(content).style(Style::default().bg(Color::DarkGray))
                } else {
                    ListItem::new(content)
                }
            })
            .collect();
    
        let content = List::new(content)
            .block(Block::default().borders(Borders::ALL).title("Content"))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED));
    
        f.render_stateful_widget(content, chunks[1], &mut ListState::default());
    
        // Status bar
        let filename = self.editor.filename.as_deref().unwrap_or("Untitled");
        let status = format!(
            "Cursor: ({}, {}) | {} | Ctrl-Q: Quit, Ctrl-S: Save, Ctrl-O: Open, Ctrl-L: LLM",
            self.editor.cursor_x, self.editor.cursor_y, filename
        );
        let status_bar = Paragraph::new(status)
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(status_bar, chunks[2]);
    }

    fn prompt_filename<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        let mut filename = String::new();
        loop {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
                    .split(f.size());

                let prompt = Paragraph::new(format!("Enter filename: {}", filename))
                    .style(Style::default().fg(Color::Yellow))
                    .block(Block::default().borders(Borders::ALL));
                f.render_widget(prompt, chunks[1]);
            })?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter => {
                        self.editor.set_filename(filename);
                        break;
                    }
                    KeyCode::Char(c) => filename.push(c),
                    KeyCode::Backspace => {
                        filename.pop();
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn process_llm<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        let mut instruction = String::new();
        loop {
            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(2)
                    .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
                    .split(f.size());
    
                let prompt = Paragraph::new(format!("Enter LLM instruction: {}", instruction))
                    .style(Style::default().fg(Color::Yellow))
                    .block(Block::default().borders(Borders::ALL));
                f.render_widget(prompt, chunks[1]);
            })?;
    
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter => {
                        let content = self.editor.get_content();
                        let response = self.llm.process(&instruction, content);
                        self.editor.status_message = response;
                        break;
                    }
                    KeyCode::Char(c) => instruction.push(c),
                    KeyCode::Backspace => {
                        instruction.pop();
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, terminal::EnterAlternateScreen, event::EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let res = self.run_app(&mut terminal);

        // restore terminal
        terminal::disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            terminal::LeaveAlternateScreen,
            event::DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            println!("{:?}", err)
        }

        Ok(())
    }
}
