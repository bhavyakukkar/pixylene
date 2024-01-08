use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::{ min, max };

use crate::elements::common::{ Coord, Pixel, BlendMode };
use crate::project::Project;
use crate::action::{ Action, ActionError, Change, actions::draw_once::DrawOnce };

pub struct CopyAndPaste {
    pub start_corner: Option<Coord>,
    pub selected_pixels: Vec<Coord>,
}
