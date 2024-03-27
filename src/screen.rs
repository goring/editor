use crossterm::{
    cursor, event::poll, execute, style::Print, terminal, ExecutableCommand, QueueableCommand,
};

use std::{
    io::{Stdout, Write},
    time::Duration,
};

use crate::types::{Cursor, CursorStyle, EditorEvent, KeyCode, KeyEvent, KeyModifiers};

pub struct Screen {
    stdout: Stdout,
}

// Figure out how to make a generic Screen trait that can be implemented for different platforms
#[allow(dead_code)]
impl Screen {
    pub fn new() -> Screen {
        Screen {
            stdout: std::io::stdout(),
        }
    }

    pub fn build() -> anyhow::Result<Screen> {
        terminal::enable_raw_mode()?;
        let screen = Screen::new();
        #[cfg(feature = "use_alternate_screen")]
        {
            execute!(&screen.stdout, terminal::EnterAlternateScreen)?;
        };
        Ok(screen)
    }

    pub fn clear(&mut self) -> anyhow::Result<()> {
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))?;
        Ok(())
    }

    fn transform_key_event(
        &mut self,
        event: crossterm::event::KeyEvent,
    ) -> anyhow::Result<EditorEvent> {
        let key = match event.code {
            crossterm::event::KeyCode::Char(c) => KeyCode::Char(c),
            crossterm::event::KeyCode::Enter => KeyCode::Enter,
            crossterm::event::KeyCode::Backspace => KeyCode::Backspace,
            crossterm::event::KeyCode::Delete => KeyCode::Delete,
            crossterm::event::KeyCode::Up => KeyCode::ArrowUp,
            crossterm::event::KeyCode::Down => KeyCode::ArrowDown,
            crossterm::event::KeyCode::Left => KeyCode::ArrowLeft,
            crossterm::event::KeyCode::Right => KeyCode::ArrowRight,
            crossterm::event::KeyCode::PageUp => KeyCode::PageUp,
            crossterm::event::KeyCode::PageDown => KeyCode::PageDown,
            crossterm::event::KeyCode::Home => KeyCode::Home,
            crossterm::event::KeyCode::End => KeyCode::End,
            crossterm::event::KeyCode::Tab => KeyCode::Tab,
            crossterm::event::KeyCode::Esc => KeyCode::Escape,
            crossterm::event::KeyCode::F(n) => KeyCode::F(n),
            crossterm::event::KeyCode::BackTab => KeyCode::Tab,
            crossterm::event::KeyCode::Insert => KeyCode::Insert,
            crossterm::event::KeyCode::CapsLock => KeyCode::CapsLock,
            crossterm::event::KeyCode::ScrollLock => KeyCode::ScrollLock,
            crossterm::event::KeyCode::NumLock => KeyCode::NumLock,
            crossterm::event::KeyCode::PrintScreen => KeyCode::PrintScreen,
            crossterm::event::KeyCode::Pause => KeyCode::Pause,
            crossterm::event::KeyCode::Menu => KeyCode::Menu,
            crossterm::event::KeyCode::KeypadBegin => todo!("Crossterm KeypadBegin"),
            crossterm::event::KeyCode::Media(_) => todo!("Crossterm Media"),
            crossterm::event::KeyCode::Modifier(_) => todo!("Crossterm Modifier"),
            crossterm::event::KeyCode::Null => KeyCode::Null,
        };

        let modifiers = match event.modifiers {
            crossterm::event::KeyModifiers::SHIFT => KeyModifiers::SHIFT,
            crossterm::event::KeyModifiers::CONTROL => KeyModifiers::CONTROL,
            crossterm::event::KeyModifiers::ALT => KeyModifiers::ALT,
            crossterm::event::KeyModifiers::SUPER => KeyModifiers::SUPER,
            crossterm::event::KeyModifiers::NONE | _ => KeyModifiers::NONE,
        };

        Ok(EditorEvent::Key(KeyEvent { key, modifiers }))
    }

    pub fn poll(&mut self, duration: Duration) -> anyhow::Result<Option<EditorEvent>> {
        let result = if poll(duration)? {
            let event = crossterm::event::read()?;
            match event {
                crossterm::event::Event::Key(event) => Some(self.transform_key_event(event)?),
                _ => None,
            }
        } else {
            None
        };
        Ok(result)
    }

    pub fn flush(&mut self) -> anyhow::Result<()> {
        self.stdout.flush()?;
        Ok(())
    }

    pub fn move_cursor(&mut self, cursor: Cursor) -> anyhow::Result<()> {
        self.stdout.queue(cursor::MoveTo(cursor.col, cursor.row))?;
        Ok(())
    }

    pub fn set_cursor_style(&mut self, style: CursorStyle) -> anyhow::Result<()> {
        self.stdout.queue(match style {
            CursorStyle::DefaultUserShape => crossterm::cursor::SetCursorStyle::DefaultUserShape,
            CursorStyle::BlinkingBar => crossterm::cursor::SetCursorStyle::BlinkingBar,
            CursorStyle::BlinkingBlock => crossterm::cursor::SetCursorStyle::BlinkingBlock,
            CursorStyle::SteadyBar => crossterm::cursor::SetCursorStyle::SteadyBar,
            CursorStyle::SteadyBlock => crossterm::cursor::SetCursorStyle::SteadyBlock,
            CursorStyle::BlinkingUnderScore => {
                crossterm::cursor::SetCursorStyle::BlinkingUnderScore
            }
            CursorStyle::SteadyUnderScore => crossterm::cursor::SetCursorStyle::SteadyUnderScore,
        })?;
        Ok(())
    }

    pub fn draw_rows(&mut self, rows: &Vec<String>) -> anyhow::Result<()> {
        self.stdout.queue(cursor::MoveTo(0, 0))?;
        for (row_index, row) in rows.iter().enumerate() {
            for (col_index, col) in row.chars().enumerate() {
                self.stdout
                    .queue(cursor::MoveTo(col_index as u16, row_index as u16))?
                    .queue(Print(col))?;
            }
        }
        self.flush()?;
        Ok(())
    }

    pub fn teardown(&mut self) -> anyhow::Result<()> {
        terminal::disable_raw_mode()?;
        self.set_cursor_style(CursorStyle::DefaultUserShape)?;
        // #[cfg(feature = "use_alternate_screen")]
        // {
        //     execute!(&self.stdout, terminal::LeaveAlternateScreen)?;
        // };
        Ok(())
    }
}
