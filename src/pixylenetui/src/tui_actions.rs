use libpixylene::{
    self,
    Pixylene,
    types::{ Coord, BlendMode },
    project::{ Cursor },
    action::{ self, actions::* }
};
use crate::{ utils::LogType, pixylene_tui::Console };

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


struct CircularOutline {
    palette_index: usize,
    console: Rc<Console>,
}
impl action::Action for CircularOutline {
    fn perform_action(&mut self, project: &mut libpixylene::project::Project) -> Result<Vec<action::Change>, action::ActionError> {
        use action::actions::draw_at_one_cursor::DrawAtOneCursor;

        if project.cursors.len() != 1 {
            return Err(action::ActionError::OnlyNCursorsSupported(String::from("1"), project.cursors.len()));
        }
        project.palette.get_color((&self).palette_index)?;

        let mut changes: Vec<action::Change> = vec![action::Change::Start];
        let radius: u16 = match self.console.cmdin("Radius: ") {
            Some(input) => {
                match str::parse::<u16>(&input) {
                    Ok(radius) => radius,
                    Err(_) => {
                        self.console.cmdout("invalid radius", LogType::Error);
                        //return action::ActionError::ActionCancelled;
                        return Ok(Vec::new());
                    }
                }
            },
            //None => { return action::ActionError::ActionCancelled; },
            None => { return Ok(Vec::new()); },
        };
        if radius == 0 {
            self.console.cmdout("invalid radius: 0", LogType::Error);
            return Ok(Vec::new());
        }

        let Cursor{ coord: center, .. } = project.get_cursor(0)?;
        let x0 = center.x;
        let y0 = center.y;
        let mut plot = | x: isize, y: isize | {
            action::include(Box::new(DrawAtOneCursor{
                cursor: Cursor { coord: Coord{ x, y }, layer: project.cursors[0].layer },
                color: project.palette.get_color((&self).palette_index).unwrap(),
                blend_mode: BlendMode::Normal,
            }), project, &mut changes);
        };

        /* 
         * The following algorithm was referred to from:
         * https://rosettacode.org/wiki/Bitmap/Midpoint_circle_algorithm?oldid=358330
         */
         let mut f = 1 - radius as isize;
         let mut ddf_x = 1;
         let mut ddf_y = -2 * radius as isize;
         let mut x = 0 as isize;
         let mut y = radius as isize;
         plot(x0, y0 + radius as isize);
         plot(x0, y0 - radius as isize);
         plot(x0 + radius as isize, y0);
         plot(x0 - radius as isize, y0);
         while x < y {
             if f >= 0 { 
                 y -= 1;
                 ddf_y += 2;
                 f += ddf_y;
             }
             x += 1;
             ddf_x += 2;
             f += ddf_x;   
             plot(x0 + x, y0 + y);
             plot(x0 - x, y0 + y);
             plot(x0 + x, y0 - y);
             plot(x0 - x, y0 - y);
             plot(x0 + y, y0 + x);
             plot(x0 - y, y0 + x);
             plot(x0 + y, y0 - x);
             plot(x0 - y, y0 - x);
         }
        /*
         * End
         */

        changes.push(action::Change::End);
        Ok(changes)
    }
}

struct Echo {
    console: Rc<Console>,
}
impl action::Action for Echo {
    fn perform_action(&mut self, project: &mut libpixylene::project::Project) -> Result<Vec<action::Change>, action::ActionError> {
        if let Some(string) = self.console.cmdin(":echo ") {
            self.console.cmdout(&string, LogType::Info);
        }
        Ok(Vec::new())
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

    actions.insert(String::from("circular_outline"), Box::new(CircularOutline{
        palette_index: 4, console: Rc::clone(console),
    }));
    actions.insert(String::from("echo"), Box::new(Echo{
        console: Rc::clone(console),
    }));
}
