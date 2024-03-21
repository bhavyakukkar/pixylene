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

    RunCommand(String),
    RunCommandSpecify,

    RunAction(String),
    RunActionSpecify,
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
        ( Key::new(KeyCode::Char(':'), KeyModifiers::empty()), vec![UiFn::RunCommandSpecify] ),
        ( Key::new(KeyCode::Esc, KeyModifiers::empty()), vec![UiFn::Quit] ),

        ( Key::new(KeyCode::Char('h'), KeyModifiers::empty()),
            vec![UiFn::RunAction(String::from("cursors_left"))] ),
        ( Key::new(KeyCode::Char('j'), KeyModifiers::empty()),
            vec![UiFn::RunAction(String::from("cursors_down"))] ),
        ( Key::new(KeyCode::Char('k'), KeyModifiers::empty()),
            vec![UiFn::RunAction(String::from("cursors_up"))] ),
        ( Key::new(KeyCode::Char('l'), KeyModifiers::empty()),
            vec![UiFn::RunAction(String::from("cursors_right"))] ),

        ( Key::new(KeyCode::Char('1'), KeyModifiers::empty()),
            vec![UiFn::RunAction(String::from("pencil1"))] ),
        ( Key::new(KeyCode::Char('2'), KeyModifiers::empty()),
            vec![UiFn::RunAction(String::from("pencil2"))] ),
        ( Key::new(KeyCode::Char('3'), KeyModifiers::empty()),
            vec![UiFn::RunAction(String::from("pencil3"))] ),
        ( Key::new(KeyCode::Char('4'), KeyModifiers::empty()),
            vec![UiFn::RunAction(String::from("pencil4"))] ),
        ( Key::new(KeyCode::Char('5'), KeyModifiers::empty()),
            vec![UiFn::RunAction(String::from("pencil5"))] ),
        ( Key::new(KeyCode::Char('6'), KeyModifiers::empty()),
            vec![UiFn::RunAction(String::from("pencil6"))] ),
        ( Key::new(KeyCode::Char('7'), KeyModifiers::empty()),
            vec![UiFn::RunAction(String::from("pencil7"))] ),
        ( Key::new(KeyCode::Char('8'), KeyModifiers::empty()),
            vec![UiFn::RunAction(String::from("pencil8"))] ),
        ( Key::new(KeyCode::Char('9'), KeyModifiers::empty()),
            vec![UiFn::RunAction(String::from("pencil9"))] ),
        ( Key::new(KeyCode::Char('0'), KeyModifiers::empty()),
            vec![UiFn::RunAction(String::from("pencil10"))] ),

        ( Key::new(KeyCode::Char('i'), KeyModifiers::empty()),
            vec![UiFn::RunAction(String::from("zoomin"))] ),
        ( Key::new(KeyCode::Char('o'), KeyModifiers::empty()),
            vec![UiFn::RunAction(String::from("zoomout"))] ),
    ]);

    let rev_keymap: ReverseKeyMap = HashMap::from([
        ( KeyFn::DiscardCommand, Key::new(KeyCode::Esc, KeyModifiers::empty()) ),
    ]);

    (keymap, rev_keymap)
}
