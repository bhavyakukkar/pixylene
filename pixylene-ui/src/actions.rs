use libpixylene::{ types::{ Coord } };
use pixylene_actions::{
    memento::Action, utils::Direction,
    std_actions::{ scene, cursors, layer }
};
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;


pub enum ActionLocation {
    Native(Rc<RefCell<dyn Action>>),
    Lua,
}

fn insert<T: Action + 'static>(action_map: &mut HashMap<String, ActionLocation>, action_name: &str,
                               action: T) {
    action_map.insert(action_name.to_string(),
                          ActionLocation::Native(Rc::new(RefCell::new(action))));
}

pub fn add_my_native_actions(am: &mut HashMap<String, ActionLocation>) {
    // Insert Your Actions Here
    insert(am, "pencil", scene::Pencil::new(None));
    for i in 1..9 {
        insert(am, &format!("pencil{}", i), scene::Pencil::new(Some(i)));
    }

    insert(am, "cursors_up", cursors::MoveAllCursors::new(Coord{ x: -1, y: 0 }));
    insert(am, "cursors_left", cursors::MoveAllCursors::new(Coord{ x: 0, y: -1 }));
    insert(am, "cursors_down", cursors::MoveAllCursors::new(Coord{ x: 1, y: 0 }));
    insert(am, "cursors_right", cursors::MoveAllCursors::new(Coord{ x: 0, y: 1 }));

    insert(am, "cursors_dup_up", cursors::DuplicateCursors::new(Direction::Up, 1));
    insert(am, "cursors_dup_left", cursors::DuplicateCursors::new(Direction::Left, 1));
    insert(am, "cursors_dup_down", cursors::DuplicateCursors::new(Direction::Down, 1));
    insert(am, "cursors_dup_right", cursors::DuplicateCursors::new(Direction::Right, 1));

    insert(am, "cursors_reset", cursors::ResetCursors);

    insert(am, "layer_new", layer::New);
    insert(am, "layer_opacity", layer::ChangeOpacity);
    insert(am, "layer_mute", layer::Mute);
}
