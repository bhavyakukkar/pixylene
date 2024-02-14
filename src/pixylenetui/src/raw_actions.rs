use libpixylene::{ self, Pixylene, common::{ Coord, BlendMode }, action::{ self, actions::* }};
use std::collections::HashMap;

pub fn add_raw_actions(actions: &mut HashMap<String, Box<dyn action::Action>>) {
    for i in 0..8 {
        actions.insert(format!("pencil{}", i+1), Box::new(pencil::Pencil{palette_index: i+1}));
    }
    actions.insert(String::from("eraser"), Box::new(draw_at_all_cursors::DrawAtAllCursors{color: None, blend_mode: BlendMode::Overwrite}));

    actions.insert(String::from("rectangular_fill"), Box::new(rectangular_fill::RectangularFill{palette_index: 6, start_corner: None}));
    actions.insert(String::from("cursor_up"), Box::new(move_all_cursors::MoveAllCursors{displacement: Coord{ x: -1, y: 0 }}));
    actions.insert(String::from("cursor_down"), Box::new(move_all_cursors::MoveAllCursors{displacement: Coord{ x: 1, y: 0 }}));
    actions.insert(String::from("cursor_left"), Box::new(move_all_cursors::MoveAllCursors{displacement: Coord{ x: 0, y: -1 }}));
    actions.insert(String::from("cursor_right"), Box::new(move_all_cursors::MoveAllCursors{displacement: Coord{ x: 0, y: 1 }}));

    actions.insert(String::from("focus_up"), Box::new(move_focus::MoveFocus{displacement:Coord{x:-1,y:0}}));
    actions.insert(String::from("focus_down"), Box::new(move_focus::MoveFocus{displacement:Coord{x:1,y:0}}));
    actions.insert(String::from("focus_left"), Box::new(move_focus::MoveFocus{displacement:Coord{x:0,y:-1}}));
    actions.insert(String::from("focus_right"), Box::new(move_focus::MoveFocus{displacement:Coord{x:0,y:1}}));

    actions.insert(String::from("zoom_in"), Box::new(zoom_camera::ZoomCamera{mult_incr: 1}));
    actions.insert(String::from("zoom_out"), Box::new(zoom_camera::ZoomCamera{mult_incr: -1}));
    actions.insert(String::from("hor+"), Box::new(change_camera_repeat::ChangeCameraRepeat{repeat_diff: Coord{ x: 0, y: 1 }}));
    actions.insert(String::from("hor-"), Box::new(change_camera_repeat::ChangeCameraRepeat{repeat_diff: Coord{ x: 0, y: -1 }}));
    actions.insert(String::from("ver+"), Box::new(change_camera_repeat::ChangeCameraRepeat{repeat_diff: Coord{ x: 1, y: 0 }}));
    actions.insert(String::from("ver-"), Box::new(change_camera_repeat::ChangeCameraRepeat{repeat_diff: Coord{ x: -1, y: 0 }}));

    actions.insert(String::from("copy_paste_all_cursors"), Box::new(copy_paste_all_cursors::CopyPasteAllCursors{selected_pixels: Vec::new()}));
    actions.insert(String::from("toggle_cursor"), Box::new(toggle_cursor_at_focus::ToggleCursorAtFocus));
}
