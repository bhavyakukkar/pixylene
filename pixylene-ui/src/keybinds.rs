use crate::ui::Key;

use serde::{ Deserialize };
use std::collections::HashMap;


/// The map of [`Key`](super::target::Key) to the ordered [`UiFns`](UiFn) it executes when pressed
pub type KeyMap = HashMap<Key, Vec<UiFn>>;

#[derive(Debug, Deserialize, Clone, Eq, Hash, PartialEq)]
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

    RunKey(Key),

    RunCommand(String),
    RunCommandSpecify,

    RunAction(String),
    RunActionSpecify,
    RunLastAction,

    PreviewFocusLayer,
    PreviewProject,

    UpdateStatusline,
}

/// The mapping of [`Keys`](Key) to functions required by the app. 
#[derive(Debug, Deserialize)]
pub struct ReqUiFnMap {
    pub start_command: Key,
    pub discard_command: Key,
    pub force_quit: Key,
}

/// A [`UiFn`](UiFn) that is mandatorily needed to be mapped to a key for the functioning of the
/// app.
//#[derive(Clone, Eq, Hash, PartialEq, Debug)]


pub fn get_keybinds() -> KeyMap {
    use crossterm::event::{ KeyCode, KeyModifiers };

    //Vim Like
    HashMap::from([
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

        ( Key::new(KeyCode::Char('h'), KeyModifiers::CONTROL),
            vec![UiFn::RunAction(String::from("dup_cursors_left"))] ),
        ( Key::new(KeyCode::Char('j'), KeyModifiers::CONTROL),
            vec![UiFn::RunAction(String::from("dup_cursors_down"))] ),
        ( Key::new(KeyCode::Char('k'), KeyModifiers::CONTROL),
            vec![UiFn::RunAction(String::from("dup_cursors_up"))] ),
        ( Key::new(KeyCode::Char('l'), KeyModifiers::CONTROL),
            vec![UiFn::RunAction(String::from("dup_cursors_right"))] ),

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

        ( Key::new(KeyCode::Char('u'), KeyModifiers::empty()),
            vec![UiFn::Undo] ),
        ( Key::new(KeyCode::Char('r'), KeyModifiers::CONTROL),
            vec![UiFn::Redo] ),

        ( Key::new(KeyCode::Char('i'), KeyModifiers::empty()),
            vec![UiFn::RunAction(String::from("zoomin"))] ),
        ( Key::new(KeyCode::Char('o'), KeyModifiers::empty()),
            vec![UiFn::RunAction(String::from("zoomout"))] ),
    ])
}
