use super::super::scene::Draw;
use crate::{memento, utils::OptionalTrueOrIndexed, ActionError, Console};

use libpixylene::{
    project::{LayersType, Project},
    types::{BlendMode, IndexedPixel, UCoord},
};

pub struct CircularOutline {
    palette_index: Option<u8>,
}

impl CircularOutline {
    pub fn new(palette_index: Option<u8>) -> Self {
        Self { palette_index }
    }
}

impl memento::Action for CircularOutline {
    fn perform(&mut self, project: &mut Project, console: &dyn Console) -> memento::ActionResult {
        use ActionError::{Discarded, InputError, OnlyNCursorsSupported, OperationError};
        use OptionalTrueOrIndexed::*;

        let num_cursors = project.num_cursors();
        if num_cursors != 1 {
            return Err(OnlyNCursorsSupported(
                String::from("1"),
                num_cursors as usize,
            ));
        }

        let radius_input = console.cmdin("Radius: ").ok_or(Discarded)?;
        let radius: u16 = str::parse::<u16>(&radius_input)
            .map_err(|_| InputError(format!("invalid radius: {}", radius_input)))?;
        if radius == 0 {
            return Err(InputError("radius cannot be 0".to_owned()));
        }

        let (center, layer) = project
            .cursors()
            .next()
            .expect("already asserted that number of cursors == 1")
            .clone();
        let color: OptionalTrueOrIndexed = match &project.canvas.layers {
            LayersType::True(_) => True(Some(match &self.palette_index {
                Some(index) => *project.canvas.palette.get_color(*index)?,
                None => *project.canvas.palette.get_equipped(),
            })),
            LayersType::Indexed(_) => Indexed(Some(IndexedPixel(
                self.palette_index
                    .unwrap_or(project.canvas.palette.equipped()),
            ))),
        };

        let x0 = center.x;
        let y0 = center.y;
        let mut plot = |x: u16, y: u16| -> memento::ActionResult {
            Draw::new((UCoord { x, y }, layer), color.clone(), BlendMode::Normal)
                .perform(project, console)
        };

        /*
         * The following algorithm was referred to from:
         * https://rosettacode.org/wiki/Bitmap/Midpoint_circle_algorithm?oldid=358330
         */
        let mut f = 1 - radius as isize;
        let mut ddf_x = 1;
        let mut ddf_y = -2 * radius as isize;
        let mut x = 0;
        let mut y = radius as isize;
        plot(
            x0,
            y0.checked_add(radius.try_into().or(Err(OperationError(None)))?)
                .ok_or(OperationError(None))?,
        )?;
        plot(
            x0,
            y0.checked_sub(radius.try_into().or(Err(OperationError(None)))?)
                .ok_or(OperationError(None))?,
        )?;
        plot(
            x0.checked_add(radius.try_into().or(Err(OperationError(None)))?)
                .ok_or(OperationError(None))?,
            y0,
        )?;
        plot(
            x0.checked_sub(radius.try_into().or(Err(OperationError(None)))?)
                .ok_or(OperationError(None))?,
            y0,
        )?;
        while x < y {
            if f >= 0 {
                y -= 1;
                ddf_y += 2;
                f += ddf_y;
            }
            x += 1;
            ddf_x += 2;
            f += ddf_x;
            plot(
                x0.checked_add(x.try_into().or(Err(OperationError(None)))?)
                    .ok_or(OperationError(None))?,
                y0.checked_add(y.try_into().or(Err(OperationError(None)))?)
                    .ok_or(OperationError(None))?,
            )?;
            plot(
                x0.checked_sub(x.try_into().or(Err(OperationError(None)))?)
                    .ok_or(OperationError(None))?,
                y0.checked_add(y.try_into().or(Err(OperationError(None)))?)
                    .ok_or(OperationError(None))?,
            )?;
            plot(
                x0.checked_add(x.try_into().or(Err(OperationError(None)))?)
                    .ok_or(OperationError(None))?,
                y0.checked_sub(y.try_into().or(Err(OperationError(None)))?)
                    .ok_or(OperationError(None))?,
            )?;
            plot(
                x0.checked_sub(x.try_into().or(Err(OperationError(None)))?)
                    .ok_or(OperationError(None))?,
                y0.checked_sub(y.try_into().or(Err(OperationError(None)))?)
                    .ok_or(OperationError(None))?,
            )?;
            plot(
                x0.checked_add(y.try_into().or(Err(OperationError(None)))?)
                    .ok_or(OperationError(None))?,
                y0.checked_add(x.try_into().or(Err(OperationError(None)))?)
                    .ok_or(OperationError(None))?,
            )?;
            plot(
                x0.checked_sub(y.try_into().or(Err(OperationError(None)))?)
                    .ok_or(OperationError(None))?,
                y0.checked_add(x.try_into().or(Err(OperationError(None)))?)
                    .ok_or(OperationError(None))?,
            )?;
            plot(
                x0.checked_add(y.try_into().or(Err(OperationError(None)))?)
                    .ok_or(OperationError(None))?,
                y0.checked_sub(x.try_into().or(Err(OperationError(None)))?)
                    .ok_or(OperationError(None))?,
            )?;
            plot(
                x0.checked_sub(y.try_into().or(Err(OperationError(None)))?)
                    .ok_or(OperationError(None))?,
                y0.checked_sub(x.try_into().or(Err(OperationError(None)))?)
                    .ok_or(OperationError(None))?,
            )?;
        }
        Ok(())
    }

    /*
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

        Ok(changes)
    }
    */
}
