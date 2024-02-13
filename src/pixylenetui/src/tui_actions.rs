use libpixylene::{ self, Pixylene, common::Coord, action::{ self, actions::* }};
use crate::pixylene_tui::Console;

use std::rc::Rc;
use std::collections::HashMap;

struct SensiblyMoveToLayer {
    to: Option<usize>,
    by: isize, //not checked if 'to' is defined
    console: Rc<Console>,
}

impl action::Action for SensiblyMoveToLayer {
    fn perform_action(&mut self, project: &mut libpixylene::project::Project) -> Result<Vec<action::Change>, action::ActionError> {
        let mut changes: Vec<action::Change> = vec![action::Change::Start];
        let actual_to: isize = if let Some(to) = self.to { to as isize - 1 } else { project.focus.layer as isize + self.by };

        action::include(Box::new(action::actions::set_focus::SetFocus {
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


pub fn add_tui_actions(actions: &mut HashMap<String, Box<dyn action::Action>>, console: &Rc<Console>) {
    actions.insert(String::from("move_one_layer_down"), Box::new(SensiblyMoveToLayer{
        to: None, by: 1, console: Rc::clone(console),
    }));
    actions.insert(String::from("move_one_layer_up"), Box::new(SensiblyMoveToLayer{
        to: None, by: -1, console: Rc::clone(console),
    }));
    actions.insert(String::from("move_to_first_layer"), Box::new(SensiblyMoveToLayer{
        to: Some(1), by: 0, console: Rc::clone(console),
    }));
    actions.insert(String::from("move_to_last_layer"), Box::new(SensiblyMoveToLayer{
        to: Some(std::usize::MAX), by: 0, console: Rc::clone(console),
    }));
}
