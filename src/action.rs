use crate::elements::common::{ Coord, Pixel };
use crate::elements::stroke::StrokeState;
use crate::project::Project;

pub trait Action {
    //perform action and transform to reverted action (for when undo/redo called)
    fn perform_action(&mut self, project: &mut Project) -> Result<(), String>;
}

pub struct ResizeCamera {
    pub new_dim: Coord
}
impl Action for ResizeCamera {
    fn perform_action(&mut self, project: &mut Project) -> Result<(), String> {
        let old_dim = project.camera.dim;
        project.camera.set_dim(self.new_dim)?;
        self.new_dim = old_dim;
        Ok(())
    }
}

pub struct MoveCamera {
    pub new_focus: Coord
}
impl Action for MoveCamera {
    fn perform_action(&mut self, project: &mut Project) -> Result<(), String> {
        let old_focus = project.camera.focus;
        project.camera.set_focus(&project.layers[project.selected_layer].scene, self.new_focus)?;
        self.new_focus = old_focus;
        Ok(())
    }
}

pub struct ZoomCamera {
    pub new_mult: isize
}
impl Action for ZoomCamera {
    fn perform_action(&mut self, project: &mut Project) -> Result<(), String> {
        let old_mult = project.camera.mult;
        project.camera.set_mult(self.new_mult)?;
        self.new_mult = old_mult;
        Ok(())
    }
}

pub struct ChangeCameraRepeat {
    pub new_repeat: Coord
}
impl Action for ChangeCameraRepeat {
    fn perform_action(&mut self, project: &mut Project) -> Result<(), String> {
        let old_repeat = project.camera.repeat;
        project.camera.set_repeat(self.new_repeat)?;
        self.new_repeat = old_repeat;
        Ok(())
    }
}

pub struct DrawOnce {
    pub palette_index: usize,
    pub new_pixel: Option<Option<Pixel>>
}
impl Action for DrawOnce {
    fn perform_action(&mut self, project: &mut Project) -> Result<(), String> {
        if let Some(( stroke, stroke_state )) = project.strokes.get_mut(&project.selected_stroke) {
            let old_pixel = project.layers[project.selected_layer].scene.get_pixel(
                project.camera.focus
            )?;
            stroke.perform_stroke(
                stroke_state.clicks_done,
                &mut project.layers[project.selected_layer].scene,
                project.camera.focus,
                match self.new_pixel {
                    Some(pixel_maybe) => pixel_maybe,
                    None => project.palette.get_color(self.palette_index)?
                }
            );
            let StrokeState {
                clicks_done: clicks_done,
                clicks_required: clicks_required
            } = stroke_state;
            stroke_state.clicks_done = if *clicks_done == *clicks_required - 1u8 {
                0
            } else {
                *clicks_done + 1
            };
            self.new_pixel = Some(old_pixel);
            Ok(())
        } else {
            Err(format!("stroke '{}' was not found in the project", project.selected_stroke))
        }
    }
}
