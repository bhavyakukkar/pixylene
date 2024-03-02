pub trait PixyleneUITarget {
    type Key;

    fn draw_camera(&self, dim: PCoord, buffer: Vec<CameraPixel>);
    fn getkey(&self) -> Key;
    fn draw_statusline(&self, statusline: Statusline);
    fn command_in(&self) -> Option<String>;
    fn command_out(&self, message: String, log_type: LogType);
    fn draw_paragraph(&self, paragraph: Vec<String>);
}

//leaving this here
//Cursors similar to helix where clone_cursor_left clones
//the leftmost cursor/s, and vice versa wrt up, down & right

enum Key {
    Save,
    Export,
    Quit,

    Undo,
    Redo,

    StartCommand,
    DiscardCommand,
    FinishCommand,

    StartPreviewEntireProject,
    StopPreviewEntireProject,

    MoveCursorUp,
    MoveCursorLeft,
    MoveCursorDown,
    MoveCursorRight,

    MoveFocusUp,
    MoveFocusLeft,
    MoveFocusDown,
    MoveFocusRight,
    ZoomIn,
    ZoomOut,

    CircularOutline,
    CircularFill,

    StartRectangularFill,
    EndRectangularFill,

    StartRectangularOutline,
    EndRectangularOutline,

    Erase,

    CopySelection,
    PaseSelection,

    Pencil(u8),

    MoveOneLayerUp,
    MoveOneLayerDown,
}
