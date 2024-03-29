use libpixylene::{ types::{ Coord, UCoord, Pixel, BlendMode } };
use pixylene_actions::{ memento::Action, utils::Direction, std_actions::{ scene::*, cursors::* } };
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
    insert(am, "draw", Draw::new((UCoord{ x: 0, y: 0 }, 0), Some(Pixel::BLACK), BlendMode::Overwrite));

    for i in 1..17 {
        insert(am, &format!("pencil{}", i), Pencil::new(Some(i)));
    }

    insert(am, "cursors_up", MoveAllCursors::new(Coord{ x: -1, y: 0 }));
    insert(am, "cursors_left", MoveAllCursors::new(Coord{ x: 0, y: -1 }));
    insert(am, "cursors_down", MoveAllCursors::new(Coord{ x: 1, y: 0 }));
    insert(am, "cursors_right", MoveAllCursors::new(Coord{ x: 0, y: 1 }));

    insert(am, "dup_cursors_up", DuplicateCursors::new(Direction::Up, 1));
    insert(am, "dup_cursors_left", DuplicateCursors::new(Direction::Left, 1));
    insert(am, "dup_cursors_down", DuplicateCursors::new(Direction::Down, 1));
    insert(am, "dup_cursors_right", DuplicateCursors::new(Direction::Right, 1));
}
