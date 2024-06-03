use libpixylene::types::Coord;
use pixylene_actions::{
    memento::Action,
    std_actions::{cursors, layer, scene},
    utils::Direction,
};
use pixylene_lua::LuaActionManager;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub enum ActionLocation {
    Native(Rc<RefCell<dyn Action>>),
    Lua,
}

fn insert_native<T: Action + 'static>(
    action_map: &mut HashMap<String, ActionLocation>,
    action_name: &str,
    action: T,
) {
    action_map.insert(
        action_name.to_string(),
        ActionLocation::Native(Rc::new(RefCell::new(action))),
    );
}

pub fn add_my_native_actions(amp: &mut HashMap<String, ActionLocation>) {
    // Insert Native Action Instances Here
    insert_native(amp, "pencil", scene::Pencil::new(None));
    for i in 1..9 {
        insert_native(amp, &format!("pencil{}", i), scene::Pencil::new(Some(i)));
    }

    insert_native(amp, "cursors_up", cursors::MoveAllCursors::new(Coord{ x: -1, y: 0 }));
    insert_native(amp, "cursors_left", cursors::MoveAllCursors::new(Coord{ x: 0, y: -1 }));
    insert_native(amp, "cursors_down", cursors::MoveAllCursors::new(Coord{ x: 1, y: 0 }));
    insert_native(amp, "cursors_right", cursors::MoveAllCursors::new(Coord{ x: 0, y: 1 }));

    insert_native(amp, "cursors_dup_up", cursors::DuplicateCursors::new(Direction::Up, 1));
    insert_native(amp, "cursors_dup_left", cursors::DuplicateCursors::new(Direction::Left, 1));
    insert_native(amp, "cursors_dup_down", cursors::DuplicateCursors::new(Direction::Down, 1));
    insert_native(amp, "cursors_dup_right", cursors::DuplicateCursors::new(Direction::Right, 1));

    insert_native(amp, "cursors_reset", cursors::ResetCursors);

    insert_native(amp, "layer_new", layer::New);
    insert_native(amp, "layer_opacity", layer::ChangeOpacity);
    insert_native(amp, "layer_mute", layer::Mute);
}

pub fn add_my_lua_actions(am: &mut LuaActionManager) {
    let std_actions = std::include_str!("std-actions.lua");
    let _ = am.load(std_actions);
}
