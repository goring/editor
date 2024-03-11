use crate::{
    editor_config::EditorConfig,
    screen::Screen,
    types::{Cursor, CursorStyle, EditorCommand, EditorEvent, KeyCode, Mode},
};
use log::debug;
use std::time::Duration;

pub struct Editor {
    screen: Screen,
    mode: Mode,
    cursor: Cursor,
    doc: Vec<String>,
}

impl Editor {
    pub fn new() -> anyhow::Result<Editor> {
        Ok(Editor {
            screen: Screen::build()?,
            mode: Mode::Insert,
            cursor: Cursor { col: 0, row: 0 },
            doc: vec![String::new()],
        })
    }

    fn change_mode(&mut self, mode: Mode) -> anyhow::Result<()> {
        match mode {
            Mode::Insert => self.screen.set_cursor_style(CursorStyle::SteadyBar)?,
            Mode::Normal => self.screen.set_cursor_style(CursorStyle::SteadyBlock)?,
            Mode::Visual => self
                .screen
                .set_cursor_style(CursorStyle::SteadyUnderScore)?,
        };
        self.mode = mode;
        Ok(())
    }

    fn remove_char(&mut self, cursor: Option<Cursor>) {
        let cursor = cursor.unwrap_or(self.cursor.clone());
        if cursor.col > 0 {
            self.doc[cursor.row as usize].remove(cursor.col as usize - 1);
            self.cursor.col -= 1;
        } else if cursor.row > 0 {
            let row = self.doc.remove(cursor.row as usize);
            let col = self.doc[cursor.row as usize - 1].len();
            self.doc[cursor.row as usize - 1].push_str(&row);
            self.cursor.row -= 1;
            self.cursor.col = col as u16;
        }
    }

    fn insert_char(&mut self, c: char, cursor: Option<Cursor>) {
        let cursor = cursor.unwrap_or(self.cursor.clone());
        self.doc[cursor.row as usize].insert(cursor.col as usize, c);
        self.cursor.col += 1;
    }

    fn insert_line(&mut self, cursor: Option<Cursor>) {
        let cursor = cursor.unwrap_or(self.cursor.clone());
        let rest = self.doc[cursor.row as usize][cursor.col as usize..].to_string();
        self.doc[cursor.row as usize].truncate(cursor.col as usize);
        self.doc.insert(cursor.row as usize + 1, rest);
        self.cursor.row += 1;
        self.cursor.col = 0;
    }

    fn execute_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Quit => {
                self.screen.teardown().unwrap();
                std::process::exit(0);
            }
            EditorCommand::InsertChar(c) => {
                self.insert_char(c, None);
            }
            EditorCommand::RemoveChar => {
                self.remove_char(None);
            }
            EditorCommand::InsertLine => {
                self.insert_line(None);
            }
            EditorCommand::MoveCursorUp => {
                if self.cursor.row > 0 {
                    self.cursor.row = self.cursor.row.saturating_sub(1);
                    // clamp the column to the length of the row
                    self.cursor.col = self
                        .cursor
                        .col
                        .min(self.doc[self.cursor.row as usize].len() as u16);
                }
            }
            EditorCommand::MoveCursorDown => {
                if self.cursor.row < self.doc.len() as u16 - 1 {
                    self.cursor.row = self.cursor.row.saturating_add(1);
                    self.cursor.col = self
                        .cursor
                        .col
                        .min(self.doc[self.cursor.row as usize].len() as u16);
                }
            }
            EditorCommand::MoveCursorLeft => {
                if self.cursor.col > 0 {
                    self.cursor.col = self.cursor.col.saturating_sub(1);
                }
            }
            EditorCommand::MoveCursorRight => {
                if self.cursor.col < self.doc[self.cursor.row as usize].len() as u16 {
                    self.cursor.col = self.cursor.col.saturating_add(1);
                }
            }
            _ => {
                todo!("Implement {command:?}")
            }
        }
    }

    pub fn run(&mut self, config: EditorConfig) -> anyhow::Result<()> {
        loop {
            self.change_mode(Mode::Insert)?;
            self.screen.clear()?;
            self.screen.draw_rows(&self.doc)?;
            self.screen.move_cursor(self.cursor)?;
            self.screen.flush()?;

            if let Some(event) = self.screen.poll(Duration::from_millis(300))? {
                match event {
                    EditorEvent::Key(event) => {
                        debug!("Comparing {event:?} to {config:?}");
                        if let Some(keymap) = config.keymaps.iter().find(|keymap| {
                            keymap.key == event.key && keymap.modifiers == event.modifiers
                        }) {
                            self.execute_command(keymap.command);
                        } else {
                            match event.key {
                                KeyCode::Char(ch) => {
                                    self.execute_command(EditorCommand::InsertChar(ch));
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        self.screen.teardown().unwrap();
    }
}
