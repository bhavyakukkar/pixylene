use libpixylene::{ self, Pixylene, common::Coord, action::{ self, actions::* }};


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

    pixylene.add_action("copy_paste_all_cursors", Box::new(copy_paste_all_cursors::CopyPasteAllCursors{selected_pixels: Vec::new()}));
    pixylene.add_action("toggle_cursor", Box::new(toggle_cursor_at_focus::ToggleCursorAtFocus));

    pixylene.add_action("move_one_layer_down", Box::new(SensiblyMoveToLayer{to: None, by: 1}));
    pixylene.add_action("move_one_layer_up", Box::new(SensiblyMoveToLayer{to: None, by: -1}));
    pixylene.add_action("move_to_first_layer", Box::new(SensiblyMoveToLayer{to: Some(1), by: 0}));
    pixylene.add_action("move_to_last_layer", Box::new(SensiblyMoveToLayer{to: Some(std::usize::MAX), by: 0}));
}


struct SensiblyMoveToLayer {
    to: Option<usize>,
    by: isize, //not checked if 'to' is defined
}
impl action::Action for SensiblyMoveToLayer {
    fn perform_action(&mut self, project: &mut libpixylene::project::Project) -> Result<Vec<action::Change>, action::ActionError> {
        let mut changes: Vec<action::Change> = vec![action::Change::Start];
        let actual_to: isize = if let Some(to) = self.to { to as isize - 1 } else { project.focus.layer as isize + self.by };

        //move focus's layer
        /*
        if let Ok(set_focus) = (set_focus::SetFocus {
            coord: None,
            layer: Some(if actual_to >= 0 {
                if actual_to as usize <= project.layers.len() { actual_to.try_into().unwrap() }
                /* use when strict */
                //else { return Err(ActionError::InputsError(format!("trying to move to layer {} when only {} layers present", actual_to + 1, project.layers.len()))); }
                else { project.layers.len() - 1 }
            } else {
                return Err(action::ActionError::InputsError(format!("layers start from 1")));
            }),
        }).perform_action(project) {
            for change in set_focus {
                changes.push(change.as_untracked()?);
            }
        }
        */
        action::include(Box::new(set_focus::SetFocus {
            coord: None,
            layer: Some(if actual_to >= 0 {
                if actual_to as usize <= project.layers.len() { actual_to.try_into().unwrap() }
                /* use when strict */
                //else { return Err(ActionError::InputsError(format!("trying to move to layer {} when only {} layers present", actual_to + 1, project.layers.len()))); }
                else { project.layers.len() - 1 }
            } else {
                return Err(action::ActionError::InputsError(format!("layers start from 1")));
            }),
        }), project, &mut changes);

        //move every cursor to focus's layer
        /*
        for index in 0..project.cursors.len() {
            if let Ok(set_one_cursor) = (set_one_cursor::SetOneCursor {
                index: index,
                coord: None,
                layer: Some(project.focus.layer),
            }).perform_action(project) {
                for change in set_one_cursor {
                    changes.push(change.as_untracked()?);
                }
            }
        }
        */
        for index in 0..project.cursors.len() {
            action::include(Box::new(set_one_cursor::SetOneCursor {
                index: index,
                coord: None,
                layer: Some(project.focus.layer),
            }), project, &mut changes);
        }

        changes.push(action::Change::End);
        Ok(changes)
    }
}
