use crate::elements::common::{ Coord, Pixel, BlendMode };
use crate::project::Project;

use std::cmp::{ min, max };

pub trait Action {
    //perform action and transform to reverted action (for when undo/redo called)
    fn perform_action(&mut self, project: &mut Project) -> Result<(), String>;
    //whether action has been completely executed
    fn end_action(&self) -> bool;
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
    fn end_action(&self) -> bool {
        true
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
    fn end_action(&self) -> bool {
        true
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
    fn end_action(&self) -> bool {
        true
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
    fn end_action(&self) -> bool {
        true
    }
}

pub struct Pencil {
    pub palette_index: usize,
    pub new_pixel: Option<Option<Pixel>>,
}
impl Action for Pencil {
    fn perform_action(&mut self, project: &mut Project) -> Result<(), String> {
        let old_pixel = project.layers[project.selected_layer].scene.get_pixel(
            project.camera.focus
        )?;
        project.draw_pixel(
            Some(project.selected_layer),
            project.camera.focus,
            match self.new_pixel {
                Some(pixel_maybe) => pixel_maybe,
                None => project.palette.get_color(self.palette_index)?
            },
            BlendMode::Normal
        )?;
        self.new_pixel = Some(old_pixel);
        Ok(())
    }
    fn end_action(&self) -> bool {
        true
    }
}

pub struct RectangleFill {
    pub palette_index: usize,
    pub start_corner: Option<Coord>,
}
impl Action for RectangleFill {
    fn perform_action(&mut self, project: &mut Project) -> Result<(), String> {
        if let Some(start_corner) = self.start_corner {
            for i in min(start_corner.x, project.camera.focus.x)..(max(
                start_corner.x,
                project.camera.focus.x
            ) + 1) {
                for j in min(start_corner.y, project.camera.focus.y)..(max(
                    start_corner.y,
                    project.camera.focus.y
                ) + 1) {
                    project.draw_pixel(
                        Some(project.selected_layer),
                        Coord{ x: i, y: j },
                        project.palette.get_color(self.palette_index)?,
                        BlendMode::Normal
                    )?;
                }
            }
            self.start_corner = None;
        } else {
            self.start_corner = Some(project.camera.focus);
        }
        Ok(())
    }
    fn end_action(&self) -> bool {
        match self.start_corner {
            Some(_) => false,
            None => true,
        }
    }
}

pub struct RectangleOutline {
    pub palette_index: usize,
    pub start_corner: Option<Coord>,
}
impl Action for RectangleOutline {
    fn perform_action(&mut self, project: &mut Project) -> Result<(), String> {
        if let Some(start_corner) = self.start_corner {
            for i in min(start_corner.x, project.camera.focus.x)..(max(
                start_corner.x,
                project.camera.focus.x
            ) + 1) {
                project.draw_pixel(
                    Some(project.selected_layer),
                    Coord{ x: i, y: start_corner.y },
                    project.palette.get_color(self.palette_index)?,
                    BlendMode::Normal
                )?;
                project.draw_pixel(
                    Some(project.selected_layer),
                    Coord{ x: i, y: project.camera.focus.y },
                    project.palette.get_color(self.palette_index)?,
                    BlendMode::Normal
                )?;
            }
            for j in (min(start_corner.y, project.camera.focus.y) + 1)..max(
                start_corner.y,
                project.camera.focus.y
            ) {
                project.draw_pixel(
                    Some(project.selected_layer),
                    Coord{ x: start_corner.x, y: j },
                    project.palette.get_color(self.palette_index)?,
                    BlendMode::Normal
                )?;
                project.draw_pixel(
                    Some(project.selected_layer),
                    Coord{ x: project.camera.focus.x, y: j },
                    project.palette.get_color(self.palette_index)?,
                    BlendMode::Normal
                )?;
            }
            self.start_corner = None;
        } else {
            self.start_corner = Some(project.camera.focus);
        }
        Ok(())
    }
    fn end_action(&self) -> bool {
        match self.start_corner {
            Some(_) => false,
            None => true,
        }
    }
}
