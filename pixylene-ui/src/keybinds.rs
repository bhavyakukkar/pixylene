use crate::ui::Key;

use std::collections::HashMap;


/// The map of [`Key`](super::target::Key) to the ordered [`UiFns`](UiFn) it executes when pressed
pub type KeyMap = HashMap<Key, Vec<UiFn>>;

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum UiFn {
    New,
    Open,
    Import,
    Quit,
    ForceQuit,

    GoToNextSession,
    GoToPrevSession,

    Save,
    Export,

    Undo,
    Redo,

    RunCommand,
    RunAction,
    RunLastAction,

    PreviewFocusLayer,
    PreviewProject,

    UpdateStatusline,
}


/// The map of [`KeyFn`](KeyFn) to the [`Key`](Key) that it executes
///
/// This is needed for functions that require a reverse lookup, for example, the key that discards writing a command.
pub type ReverseKeyMap = HashMap<KeyFn, Key>;

/// A single key-unit that is mapped to some function executed by [`PixyleneUI`](super::PixyleneUI)
#[derive(Clone, Eq, Hash, PartialEq, Debug)]
pub enum KeyFn {
    //New,
    //Open,
    //Import,
    //Quit,
    ForceQuit,

    //GoToNextSession,
    //GoToPrevSession,

    //Save,
    //Export,

    //Undo,
    //Redo,

    //StartCommand,
    DiscardCommand,
    //FinishCommand,

    //StartPreviewEntireProject,
    //StopPreviewEntireProject,

    //DuplicateCursorsUp,
    //DuplicateCursorsLeft,
    //DuplicateCursorsDown,
    //DuplicateCursorsRight,

    //MoveCursorUp,
    //MoveCursorLeft,
    //MoveCursorDown,
    //MoveCursorRight,

    //MoveFocusUp,
    //MoveFocusLeft,
    //MoveFocusDown,
    //MoveFocusRight,
    //FocusNextLayer,
    //FocusPrevLayer,
    //ZoomIn,
    //ZoomOut,

    //CircularOutline,
    //CircularFill,

    //StartRectangularFill,
    //EndRectangularFill,

    //StartRectangularOutline,
    //EndRectangularOutline,

    //Erase,

    //CopySelection,
    //PasteSelection,

    ////Drawing Tools
    //Pencil,
    //ColorPencil,

    ////Palette
    //SetColor,
    //EquipColor,
    //UnsetColor,

    ////Layer
    //MoveOneLayerUp,
    //MoveOneLayerDown,
    //NewLayer,
    //DeleteLayer,
    //DuplicateLayer,

    //Nothing,
}


pub fn get_keybinds() -> (KeyMap, ReverseKeyMap) {
    use crossterm::event::{ KeyCode, KeyModifiers };

    //Vim Like
    let keymap: KeyMap = HashMap::from([
        ( Key::new(KeyCode::Char(':'), KeyModifiers::empty()), vec![UiFn::RunCommand] ),
    ]);

    let rev_keymap: ReverseKeyMap = HashMap::from([
        ( KeyFn::DiscardCommand, Key::new(KeyCode::Esc, KeyModifiers::empty()) ),
    ]);

    (keymap, rev_keymap)
}
