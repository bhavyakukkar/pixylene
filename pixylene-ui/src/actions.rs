use libpixylene::types::Coord;
use pixylene_actions::{
    memento::Action,
    std_actions::{cursors, layer, palette, project, scene, shapes},
    utils::Direction,
};

#[cfg(feature = "lua")]
use pixylene_lua::LuaActionManager;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type ActionPtr = Rc<RefCell<dyn Action>>;

fn insert_native<T: Action + 'static>(
    action_map: &mut HashMap<String, Rc<RefCell<dyn Action>>>,
    action_name: &str,
    action: T,
) {
    action_map.insert(action_name.to_string(), Rc::new(RefCell::new(action)));
}

pub fn add_my_native_actions(amp: &mut HashMap<String, ActionPtr>) {
    /* Insert Native Action Instances Here */

    //Pencils
    insert_native(amp, "pencil", scene::Pencil::new(None));
    for i in 1..9 {
        insert_native(amp, &format!("pencil{}", i), scene::Pencil::new(Some(i)));
    }

    //Cursors
    insert_native(
        amp,
        "cursors_up",
        cursors::MoveAllCursors::new(Coord { x: -1, y: 0 }),
    );
    insert_native(
        amp,
        "cursors_left",
        cursors::MoveAllCursors::new(Coord { x: 0, y: -1 }),
    );
    insert_native(
        amp,
        "cursors_down",
        cursors::MoveAllCursors::new(Coord { x: 1, y: 0 }),
    );
    insert_native(
        amp,
        "cursors_right",
        cursors::MoveAllCursors::new(Coord { x: 0, y: 1 }),
    );
    insert_native(
        amp,
        "cursors_dup_up",
        cursors::DuplicateCursors::new(Direction::Up, 1),
    );
    insert_native(
        amp,
        "cursors_dup_left",
        cursors::DuplicateCursors::new(Direction::Left, 1),
    );
    insert_native(
        amp,
        "cursors_dup_down",
        cursors::DuplicateCursors::new(Direction::Down, 1),
    );
    insert_native(
        amp,
        "cursors_dup_right",
        cursors::DuplicateCursors::new(Direction::Right, 1),
    );
    insert_native(amp, "cursors_reset", cursors::ResetCursors);

    //Layer
    insert_native(amp, "layer_new", layer::New);
    insert_native(amp, "layer_opacity", layer::ChangeOpacity);
    insert_native(amp, "layer_mute", layer::Mute);

    //Goto
    insert_native(
        amp,
        "goto_row_start",
        cursors::GoToSingleCursor::new(None, Some(0)),
    );
    insert_native(
        amp,
        "goto_row_end",
        cursors::GoToSingleCursor::new(None, Some(u16::MAX)),
    );
    insert_native(
        amp,
        "goto_column_start",
        cursors::GoToSingleCursor::new(Some(0), None),
    );
    insert_native(
        amp,
        "goto_column_end",
        cursors::GoToSingleCursor::new(Some(u16::MAX), None),
    );

    //View
    insert_native(amp, "zoomin", project::Multiplier::new(1));
    insert_native(amp, "zoomout", project::Multiplier::new(-1));

    //Palette
    for i in 1..9 {
        insert_native(amp, &format!("equip{}", i), palette::Equip::new(i));
    }

    //Shapes
    insert_native(amp, "circularoutline", shapes::CircularOutline::new(None));
}

#[cfg(feature = "lua")]
pub fn add_my_lua_actions(am: &mut LuaActionManager) {
    let std_actions = std::include_str!("std-actions.lua");
    let _ = am.load(std_actions);
}
