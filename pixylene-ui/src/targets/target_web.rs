use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use libpixylene::{
    project::{OPixel, Palette},
    types::PCoord,
    PixyleneDefaults,
};
use pixylene_actions::LogType;
use pixylene_ui::{
    config::Config,
    controller::{Controller, StartType},
    ui::{Key, KeyInfo, KeyMap, Rectangle, ReqUiFnMap, Statusline, UiFn, UserInterface},
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

macro_rules! keybind {
    ($key:expr, $($ui_fn:expr),+) => {
        (KeyEvent::new($key.0, $key.1).into(), vec![$($ui_fn),+])
    }
}

//impl KeyCode {
//    pub fn from_js(in: &str) -> Self {
//        //over here
//        todo!()
//    }
//}

struct Wrap<T>(pub T);

impl TryFrom<JsValue> for Wrap<Option<KeyEvent>> {
    type Error = u8;
    fn try_from(item: JsValue) -> Result<Wrap<Option<KeyEvent>>, u8> {
        item.is_object();
        todo!()
    }
}

impl Into<JsValue> for Wrap<Key> {
    fn into(self) -> JsValue {
        todo!()
    }
}

#[allow(dead_code)]
#[wasm_bindgen]
struct OPixelJS {
    pub r#type: u8,
    pub scene_coord_x: u16,
    pub scene_coord_y: u16,
    pub color_r: u8,
    pub color_g: u8,
    pub color_b: u8,
    pub color_a: u8,
    pub is_focus: bool,
    pub has_cursor: bool,
}

//impl JsObject for OPixelJS {
//}

impl From<OPixel> for OPixelJS {
    fn from(item: OPixel) -> OPixelJS {
        match item {
            OPixel::Filled {
                scene_coord,
                color,
                is_focus,
                has_cursor,
            } => OPixelJS {
                r#type: 0,
                scene_coord_x: scene_coord.x,
                scene_coord_y: scene_coord.y,
                color_r: color.r,
                color_g: color.g,
                color_b: color.b,
                color_a: color.a,
                is_focus,
                has_cursor,
            },
            OPixel::Empty {
                scene_coord,
                has_cursor,
            } => OPixelJS {
                r#type: 1,
                scene_coord_x: scene_coord.x,
                scene_coord_y: scene_coord.y,
                color_r: 0,
                color_g: 0,
                color_b: 0,
                color_a: 0,
                is_focus: false,
                has_cursor,
            },
            OPixel::OutOfScene => OPixelJS {
                r#type: 2,
                scene_coord_x: 0,
                scene_coord_y: 0,
                color_r: 0,
                color_g: 0,
                color_b: 0,
                color_a: 0,
                is_focus: false,
                has_cursor: false,
            },
        }
    }
}

impl Into<OPixel> for OPixelJS {
    fn into(self) -> OPixel {
        todo!()
    }
}

#[allow(dead_code)]
#[wasm_bindgen]
struct RectangleC {
    boundary_start_x: u16,
    boundary_start_y: u16,
    boundary_size_x: u16,
    boundary_size_y: u16,
}

impl From<&Rectangle> for RectangleC {
    fn from(item: &Rectangle) -> RectangleC {
        RectangleC {
            boundary_start_x: item.start.x,
            boundary_start_y: item.start.y,
            boundary_size_x: item.size.x(),
            boundary_size_y: item.size.y(),
        }
    }
}

//type JsKey = JsValue;
type JsKey = String;

#[wasm_bindgen(module = "/imports.js")]
extern "C" {
    type PixyleneWebJS;

    #[wasm_bindgen(constructor)]
    fn new() -> PixyleneWebJS;

    #[wasm_bindgen(method)]
    fn initialize(this: &PixyleneWebJS);

    #[wasm_bindgen(method)]
    fn finalize(this: &PixyleneWebJS);

    #[wasm_bindgen(method)]
    fn refresh(this: &PixyleneWebJS) -> bool;
    //todo

    #[wasm_bindgen(method)]
    fn get_key(this: &PixyleneWebJS) -> Option<JsKey>;

    #[wasm_bindgen(method)]
    fn get_size(this: &PixyleneWebJS) -> Box<[u16]>;
    //fn size(x: *mut u16, y: *mut u16);

    #[wasm_bindgen(method)]
    fn draw_camera(
        this: &PixyleneWebJS,
        dim_x: u16,
        dim_y: u16,
        buffer: Box<[OPixelJS]>,
        show_cursors: bool,
        boundary: *mut RectangleC,
    );

    #[wasm_bindgen(method)]
    fn draw_paragraph(this: &PixyleneWebJS, paragraph: String, boundary: *mut RectangleC);

    #[wasm_bindgen(method)]
    fn console_in(
        this: &PixyleneWebJS,
        message: String,
        discard_key: JsKey,
        boundary: *mut RectangleC,
    ) -> Option<String>;

    #[wasm_bindgen(method)]
    fn clear(this: &PixyleneWebJS, boundary: *mut RectangleC);

    #[wasm_bindgen(method)]
    fn clear_all(this: &PixyleneWebJS);
}

struct TargetWeb(PixyleneWebJS);

impl TargetWeb {
    pub const START_COMMAND: KeyEvent = KeyEvent::new(KeyCode::Char(':'), KeyModifiers::empty());
    pub const DISCARD_COMMAND: KeyEvent = KeyEvent::new(KeyCode::Char('`'), KeyModifiers::empty());
    pub const SUBMIT_COMMAND: KeyEvent = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
    pub const FORCE_QUIT: KeyEvent = KeyEvent::new(KeyCode::Char('w'), KeyModifiers::CONTROL);
}

impl UserInterface for TargetWeb {
    fn initialize(&mut self) {
        self.0.initialize();
    }

    fn finalize(&mut self) {
        self.0.finalize();
    }

    fn refresh(&mut self) -> bool {
        self.0.refresh()
    }

    fn get_key(&self) -> Option<KeyInfo> {
        //if let Ok(key) = Wrap::try_from(self.0.get_key()) {
        //    key.0.map(|key| KeyInfo::Key(key))
        //} else {
        //    None
        //}
        let key = self.0.get_key()?.chars().collect::<Vec<char>>();
        if key.len() != 1 {
            return None;
        }
        Some(KeyInfo::Key(KeyEvent::new(
            KeyCode::Char(key[0]),
            KeyModifiers::empty(),
        )))
    }

    fn get_size(&self) -> PCoord {
        let values = self.0.get_size();
        let x = *values.get(0).unwrap_or(&30);
        let y = *values.get(1).unwrap_or(&30);
        PCoord::new(x, y).unwrap_or(PCoord::new(30u16, 30u16).unwrap())
    }

    fn draw_camera(
        &mut self,
        dim: PCoord,
        buffer: Vec<OPixel>,
        show_cursors: bool,
        boundary: &Rectangle,
    ) {
        let buffer = buffer
            .into_iter()
            .map(|opixel| OPixelJS::from(opixel))
            .collect::<Vec<OPixelJS>>();
        self.0.draw_camera(
            dim.x(),
            dim.y(),
            buffer.into_boxed_slice(),
            show_cursors,
            &mut boundary.into(),
        );
    }

    fn draw_statusline(&mut self, statusline: &Statusline, boundary: &Rectangle) {
        let statusline = statusline
            .iter()
            .fold("".to_owned(), |a, b| a + &b.to_string());
        self.0.draw_paragraph(statusline, &mut boundary.into());
    }

    fn draw_paragraph(&mut self, paragraph: Vec<colored::ColoredString>, boundary: &Rectangle) {
        let paragraph = paragraph
            .iter()
            .fold("".to_owned(), |a, b| a + &b.to_string());
        self.0.draw_paragraph(paragraph, &mut boundary.into());
    }

    fn clear(&mut self, boundary: &Rectangle) {
        self.0.clear(&mut boundary.into());
    }

    fn console_in(
        &mut self,
        message: &str,
        _discard_key: &Key,
        boundary: &Rectangle,
    ) -> Option<String> {
        self.0
            .console_in(message.to_owned(), "`".to_owned(), &mut boundary.into())
        //let mut input = String::new();
        //loop {
        //    let k_i_m = self.get_key();
        //    if let Some(k_i) = k_i_m {
        //        if let KeyInfo::Key(k) = k_i {
        //            if k == Self::SUBMIT_COMMAND {
        //                break;
        //            } else if k == Self::DISCARD_COMMAND {
        //                return None;
        //            } else if let KeyCode::Char(c) = k.code {
        //                input += c;
        //            }
        //        }
        //    }
        //}
        //Some(input)
    }

    fn console_out(&mut self, message: &str, _log_type: &LogType, boundary: &Rectangle) {
        self.0
            .draw_paragraph(message.to_owned(), &mut boundary.into());
    }

    fn clear_all(&mut self) {
        self.0.clear_all();
    }
}

thread_local! {
    pub static APP: RefCell<Option<Controller>> = RefCell::new(None);
}

#[wasm_bindgen(start)]
pub fn start() {
    use KeyCode::*;
    use UiFn::*;

    APP.with_borrow_mut(|controller_maybe| {
        *controller_maybe = Some(Controller::new(
            Rc::new(RefCell::new(TargetWeb(PixyleneWebJS::new()))),
            Config {
                defaults: PixyleneDefaults {
                    dim: PCoord::new(21, 21).unwrap(),
                    palette: Palette::gruvbox(),
                    repeat: PCoord::new(1, 1).unwrap(),
                },
                default_namespace: "Main".to_owned(),
                keymap_show_command_names: true,
                possible_namespaces: HashMap::from([("Main".to_owned(), ())]),
                keymap: KeyMap::from([(
                    Some("Main".to_owned()),
                    HashMap::from([
                        keybind!(
                            (Char('h'), KeyModifiers::empty()),
                            RunAction {
                                name: "cursors_left".to_owned()
                            }
                        ),
                        keybind!(
                            (Char('h'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("cursors_left")
                            }
                        ),
                        keybind!(
                            (Char('j'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("cursors_down")
                            }
                        ),
                        keybind!(
                            (Char('k'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("cursors_up")
                            }
                        ),
                        keybind!(
                            (Char('l'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("cursors_right")
                            }
                        ),
                        keybind!(
                            (Left, KeyModifiers::empty()),
                            RunAction {
                                name: String::from("cursors_left")
                            }
                        ),
                        keybind!(
                            (Down, KeyModifiers::empty()),
                            RunAction {
                                name: String::from("cursors_down")
                            }
                        ),
                        keybind!(
                            (Up, KeyModifiers::empty()),
                            RunAction {
                                name: String::from("cursors_up")
                            }
                        ),
                        keybind!(
                            (Right, KeyModifiers::empty()),
                            RunAction {
                                name: String::from("cursors_right")
                            }
                        ),
                        keybind!(
                            (Char('H'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("cursors_dup_left")
                            }
                        ),
                        keybind!(
                            (Char('J'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("cursors_dup_down")
                            }
                        ),
                        keybind!(
                            (Char('K'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("cursors_dup_up")
                            }
                        ),
                        keybind!(
                            (Char('L'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("cursors_dup_right")
                            }
                        ),
                        keybind!(
                            (Char('R'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("cursors_reset")
                            }
                        ),
                        keybind!(
                            (Char('i'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("zoomin")
                            }
                        ),
                        keybind!(
                            (Char('o'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("zoomout")
                            }
                        ),
                        keybind!((Char('u'), KeyModifiers::empty()), Undo),
                        keybind!((Char('r'), KeyModifiers::empty()), Redo),
                        keybind!(
                            (Enter, KeyModifiers::empty()),
                            RunAction {
                                name: String::from("pencil")
                            }
                        ),
                        keybind!(
                            (Char('1'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("pencil1")
                            }
                        ),
                        keybind!(
                            (Char('2'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("pencil2")
                            }
                        ),
                        keybind!(
                            (Char('3'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("pencil3")
                            }
                        ),
                        keybind!(
                            (Char('4'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("pencil4")
                            }
                        ),
                        keybind!(
                            (Char('5'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("pencil5")
                            }
                        ),
                        keybind!(
                            (Char('6'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("pencil6")
                            }
                        ),
                        keybind!(
                            (Char('7'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("pencil7")
                            }
                        ),
                        keybind!(
                            (Char('8'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("pencil8")
                            }
                        ),
                        keybind!(
                            (Char('!'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("equip1")
                            }
                        ),
                        keybind!(
                            (Char('@'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("equip2")
                            }
                        ),
                        keybind!(
                            (Char('#'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("equip3")
                            }
                        ),
                        keybind!(
                            (Char('$'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("equip4")
                            }
                        ),
                        keybind!(
                            (Char('%'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("equip5")
                            }
                        ),
                        keybind!(
                            (Char('^'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("equip6")
                            }
                        ),
                        keybind!(
                            (Char('&'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("equip7")
                            }
                        ),
                        keybind!(
                            (Char('*'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("equip8")
                            }
                        ),
                        keybind!(
                            (Char('c'), KeyModifiers::empty()),
                            RunAction {
                                name: String::from("circularoutline")
                            }
                        ),
                    ]),
                )]),
                required_keys: ReqUiFnMap {
                    force_quit: Key::from(TargetWeb::FORCE_QUIT),
                    start_command: Key::from(TargetWeb::START_COMMAND),
                    discard_command: Key::from(TargetWeb::DISCARD_COMMAND),
                },
                every_frame: vec![UiFn::PreviewFocusLayer, UiFn::DrawStatusline],
                padding: 0,
            },
        ));
    });

    APP.with_borrow_mut(|controller_maybe| {
        if let Some(controller) = controller_maybe {
            controller.new_session(
                &StartType::New {
                    width: None,
                    height: None,
                    indexed: false,
                },
                true,
            );
        }
    });
}

#[wasm_bindgen]
pub fn tick() -> bool {
    APP.with_borrow_mut(|controller_maybe| {
        if let Some(controller) = controller_maybe {
            controller.once()
        } else {
            false
        }
    })
}

fn main() {}
