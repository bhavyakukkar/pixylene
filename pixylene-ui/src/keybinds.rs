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

/// The mapping of [`Keys`](Key) to functions mandatorily required by the app. 
#[derive(Debug, Deserialize)]
pub struct ReqUiFnMap {
    pub start_command: Key,
    pub discard_command: Key,
    pub force_quit: Key,
}
