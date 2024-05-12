use crate::{ Console, memento, ActionError, utils::OptionalTrueOrIndexed };

use libpixylene::{
    types::{ TruePixel, IndexedPixel },
    project::{ CanvasType, Canvas, Project },
};


#[derive(Debug)]
pub struct New;

impl memento::Action for New {
    fn perform(&mut self, project: &mut Project, console: &dyn Console) -> memento::ActionResult {
        use ActionError::{InputError, Discarded, InvalidCanvasType};
        use OptionalTrueOrIndexed::*;

        let color: OptionalTrueOrIndexed = match project.canvas_mut() {
            CanvasType::True(ref mut canvas) => {
                let input = console.cmdin("color (#hex or palette) (default: empty): ")
                    .ok_or(Discarded)?;
                True(match input.len() {
                    0 => None,
                    _ => match input.as_bytes()[0] {
                        b'#' => Some(TruePixel::from_hex(&input)?),
                        b'0'..=b'9' => match str::parse::<u8>(&input) {
                            Ok(index) => Some(canvas.palette().get_color(index)?.clone()),
                            Err(err) => {
                                return Err(InputError(err.to_string()));
                            }
                        },
                        _ => {
                            return Err(InputError(
                                format!("don't know how to parse '{}'", input)
                            ));
                        },
                    },
                })
            },
            CanvasType::Indexed(_) => {
                let input = console.cmdin("color index (default: empty): ")
                    .ok_or(Discarded)?;
                Indexed(match input.len() {
                    0 => None,
                    _ => match str::parse::<u8>(&input) {
                         Ok(index) => Some(IndexedPixel(index)),
                         Err(err) => {
                             return Err(InputError(err.to_string()));
                         }
                    },
                })
            },
        };

        match (project.canvas_mut(), color.clone()) {
            (CanvasType::True(ref mut canvas), True(color)) => {
                project.focus.1 = canvas.layers_mut().new_layer(color)?;
            },
            (CanvasType::Indexed(ref mut canvas), Indexed(color)) => {
                project.focus.1 = canvas.layers_mut().new_layer(color)?;
            },
            _ => { return Err(InvalidCanvasType{
                expecting_indexed: if let True(_) = color { true } else { false },
            }); }
        };
        Ok(())
    }
}
