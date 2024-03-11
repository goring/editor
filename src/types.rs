use bitflags::bitflags;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Cursor {
    pub col: u16,
    pub row: u16,
}

// Taken from crossterm cursor
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub enum CursorStyle {
    /// Default cursor shape configured by the user.
    DefaultUserShape,
    /// A blinking block cursor shape (■).
    BlinkingBlock,
    /// A non blinking block cursor shape (inverse of `BlinkingBlock`).
    SteadyBlock,
    /// A blinking underscore cursor shape(_).
    BlinkingUnderScore,
    /// A non blinking underscore cursor shape (inverse of `BlinkingUnderScore`).
    SteadyUnderScore,
    /// A blinking cursor bar shape (|)
    BlinkingBar,
    /// A steady cursor bar shape (inverse of `BlinkingBar`).
    SteadyBar,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub enum EditorCommand {
    InsertChar(char),
    InsertLine,
    HandleKey(KeyEvent),
    RemoveChar,
    MoveCursorLeft,
    MoveCursorRight,
    MoveCursorUp,
    MoveCursorDown,
    MoveCursorStart,
    MoveCursorEnd,
    MoveCursorPageUp,
    MoveCursorPageDown,
    Quit,
    Save,
    Open,
    Find,
    FindNext,
    FindPrev,
    Undo,
    Redo,
    Mode(Mode),
}

// bitflags! {
//     /// Represents key modifiers (shift, control, alt, etc.).
//     ///
//     /// **Note:** `SUPER`, `HYPER`, and `META` can only be read if
//     /// [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
//     /// [`PushKeyboardEnhancementFlags`].
//     // #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(transparent))]
//     #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize, JsonSchema)]
//     pub struct KeyModifiers: u8 {
//         const SHIFT = 0b0000_0001;
//         const CONTROL = 0b0000_0010;
//         const ALT = 0b0000_0100;
//         const SUPER = 0b0000_1000;
//         const HYPER = 0b0001_0000;
//         const META = 0b0010_0000;
//         const NONE = 0b0000_0000;
//     }
// }
#[derive(
    Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize, JsonSchema,
)]
pub enum KeyModifiers {
    SHIFT = 0b0000_0001,
    CONTROL = 0b0000_0010,
    ALT = 0b0000_0100,
    SUPER = 0b0000_1000,
    HYPER = 0b0001_0000,
    META = 0b0010_0000,
    NONE = 0b0000_0000,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
pub enum KeyCode {
    Char(char),
    F(u8),
    Enter,
    Backspace,
    Delete,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    CapsLock,
    ScrollLock,
    NumLock,
    PageUp,
    PageDown,
    PrintScreen,
    Pause,
    Menu,
    Home,
    End,
    Tab,
    Escape,
    Insert,
    Null,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub struct KeyEvent {
    pub key: KeyCode,
    pub modifiers: KeyModifiers,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub enum EditorEvent {
    Key(KeyEvent),
    // Command(EditorCommand),
}
