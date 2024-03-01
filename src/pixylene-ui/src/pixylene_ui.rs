use libpixylene::Pixylene;
use pixylene_actions::{ action_manager::ActionManager };


struct PixyleneWindow {
    pixylene: Pixylene,
    action_manager: ActionManager,
}

struct PixyleneUI {
    //windows: Vec<PixyleneWindow>, //we're not ready for this yet
    window: PixyleneWindow,
    console: Console,
    status_line: StatusLine,
    action_manager: ActionManager,
    last_action_name: Option<String>,
    project_file_path: Option<String>,
    discard_key: event::KeyEvent,
}

impl PixyleneUI {
    fn start() {
    }
}
