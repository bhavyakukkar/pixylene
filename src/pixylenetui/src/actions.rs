use libpixylene::{ Pixylene, common::Coord, action::actions::* };

pub fn add_my_actions(pixylene: &mut Pixylene) {
    for i in 0..8 {
        pixylene.add_action(&format!("pencil{}", i+1), Box::new(pencil::Pencil{palette_index: i+1}));
    }
    pixylene.add_action("eraser", Box::new(draw_at_all_cursors::DrawAtAllCursors{color: None}));

    pixylene.add_action("rectangular_fill", Box::new(rectangular_fill::RectangularFill{palette_index: 1, start_corner: None}));
    pixylene.add_action("cursor_up", Box::new(move_all_cursors::MoveAllCursors{displacement: Coord{ x: -1, y: 0 }}));
    pixylene.add_action("cursor_down", Box::new(move_all_cursors::MoveAllCursors{displacement: Coord{ x: 1, y: 0 }}));
    pixylene.add_action("cursor_left", Box::new(move_all_cursors::MoveAllCursors{displacement: Coord{ x: 0, y: -1 }}));
    pixylene.add_action("cursor_right", Box::new(move_all_cursors::MoveAllCursors{displacement: Coord{ x: 0, y: 1 }}));

    pixylene.add_action("focus_up", Box::new(move_focus::MoveFocus{displacement:Coord{x:-1,y:0}}));
    pixylene.add_action("focus_down", Box::new(move_focus::MoveFocus{displacement:Coord{x:1,y:0}}));
    pixylene.add_action("focus_left", Box::new(move_focus::MoveFocus{displacement:Coord{x:0,y:-1}}));
    pixylene.add_action("focus_right", Box::new(move_focus::MoveFocus{displacement:Coord{x:0,y:1}}));

    pixylene.add_action("zoom_in", Box::new(zoom_camera::ZoomCamera{mult_incr: 1}));
    pixylene.add_action("zoom_out", Box::new(zoom_camera::ZoomCamera{mult_incr: -1}));
    pixylene.add_action("hor+", Box::new(change_camera_repeat::ChangeCameraRepeat{repeat_diff: Coord{ x: 0, y: 1 }}));
    pixylene.add_action("hor-", Box::new(change_camera_repeat::ChangeCameraRepeat{repeat_diff: Coord{ x: 0, y: -1 }}));
    pixylene.add_action("ver+", Box::new(change_camera_repeat::ChangeCameraRepeat{repeat_diff: Coord{ x: 1, y: 0 }}));
    pixylene.add_action("ver-", Box::new(change_camera_repeat::ChangeCameraRepeat{repeat_diff: Coord{ x: -1, y: 0 }}));

    pixylene.add_action("copy_paste_all_cursors", Box::new(copy_paste_all_cursors::CopyPasteAllCursors{selected_pixels:Vec::new()}));
    pixylene.add_action("toggle_cursor", Box::new(toggle_cursor_at_focus::ToggleCursorAtFocus));
}
