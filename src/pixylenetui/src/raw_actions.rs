use libpixylene::{ self, Pixylene, types::{ Coord, BlendMode }, action::{ self, actions::* }};
use std::collections::HashMap;


struct SensiblyMoveToLayer {
    to: Option<usize>,
    by: isize, //not checked if 'to' is defined
}

impl action::Action for SensiblyMoveToLayer {
    fn perform_action(
        &mut self,
        project: &mut libpixylene::project::Project
    ) -> Result<Vec<action::Change>, action::ActionError> {
        let mut changes: Vec<action::Change> = vec![action::Change::Start];
        let actual_to: isize = if let Some(to) = self.to {
            to as isize - 1
        } else {
            project.focus.layer as isize + self.by
        };

        action::include(Box::new(action::actions::set_focus::SetFocus {
            coord: None,
            layer: Some(if actual_to >= 0 {
                if (actual_to as usize) < project.layers.len() { actual_to.try_into().unwrap() }
                /* use when strict */
                //else { return Err(ActionError::InputsError(format!("trying to move to layer {} when only {} layers present", actual_to + 1, project.layers.len()))); }
                else { project.layers.len() - 1 }
            } else {
                return Err(action::ActionError::InputsError(format!("layers start from 1")));
            }),
        }), project, &mut changes);

        for index in 0..project.cursors.len() {
            action::include(Box::new(action::actions::set_one_cursor::SetOneCursor {
                index: index,
                coord: None,
                layer: Some(project.focus.layer),
            }), project, &mut changes);
        }

        changes.push(action::Change::End);
        Ok(changes)
    }
}


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

    actions.insert(String::from("move_one_layer_down"), Box::new(SensiblyMoveToLayer{to:None,by:1}));
    actions.insert(String::from("move_one_layer_up"), Box::new(SensiblyMoveToLayer{to:None,by:-1}));
    actions.insert(String::from("move_to_first_layer"), Box::new(SensiblyMoveToLayer{to:Some(1),by:0}));
    actions.insert(String::from("move_to_last_layer"), Box::new(SensiblyMoveToLayer{to:Some(std::usize::MAX),by:0}));
}
