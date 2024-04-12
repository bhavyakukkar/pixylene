use crate::{ Console, memento, ActionError };

use libpixylene::{
    types::{ Pixel },
    project::{ Project },
};


#[derive(Debug)]
pub struct New;

impl memento::Action for New {
    fn perform(&mut self, project: &mut Project, console: &dyn Console) -> memento::ActionResult {
        use ActionError::{InputError, Discarded};

        let input = console.cmdin("color (#hex or palette) (default: empty): ").ok_or(Discarded)?;
        let color: Option<Pixel> = match input.len() {
            0 => None,
            _ => match input.as_bytes()[0] {
                b'#' => Some(Pixel::from_hex(&input)?),
                b'0'..=b'9' => match str::parse::<u8>(&input) {
                    Ok(index) => Some(project.canvas.palette.get_color(index)?.clone()),
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
        };

        project.focus.1 = project.canvas.new_layer(color)?;
        Ok(())
    }
}
