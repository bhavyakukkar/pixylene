use libpixylene::{project, types, Pixylene};
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use tealr::mlu::mlua::{Table, Value};

pub mod utils;
pub mod values;

#[derive(Clone)]
pub enum Context<T, U> {
    Solo(T),
    Linked(Rc<RefCell<Pixylene>>, U),
}

impl<T, U> Context<T, U> {
    //Mutably do & return something with this context by passing what to do in both cases
    fn do_mut<'a, FS, FL, S: 'a>(&'a mut self, f_solo: FS) -> Box<dyn FnOnce(FL) -> S + 'a>
    where
        FS: FnOnce(&'a mut T) -> S + 'a,
        FL: FnOnce(RefMut<'a, Pixylene>, &'a U) -> S + 'a,
    {
        match self {
            Context::Solo(t) => {
                let s = f_solo(t);
                Box::new(move |_| s)
            }
            Context::Linked(pixylene, data) => {
                Box::new(move |f_linked: FL| f_linked(pixylene.borrow_mut(), data))
            }
        }
    }

    //Immutably do & return something with this context by passing what to do in both cases
    fn do_imt<'a, FS, FL, S: 'a>(&'a self, f_solo: FS) -> Box<dyn FnOnce(FL) -> S + 'a>
    where
        FS: FnOnce(&'a T) -> S + 'a,
        FL: FnOnce(Ref<'a, Pixylene>, &'a U) -> S + 'a,
    {
        match self {
            Context::Solo(t) => {
                let s = f_solo(t);
                Box::new(move |_| s)
            }
            Context::Linked(pixylene, data) => {
                Box::new(move |f_linked: FL| f_linked(pixylene.borrow(), data))
            }
        }
    }
}

pub struct LuaActionManager(mlua::Lua);

impl LuaActionManager {
    pub fn new() -> Result<Self, mlua::Error> {
        let mut lua_ctx = mlua::Lua::new();

        Self::add_actions_table(&mut lua_ctx)?;
        Self::add_types(&mut lua_ctx)?;
        Ok(Self(lua_ctx))
    }

    //Load script containing 0 or more actions
    pub fn load(&mut self, user_lua: &str) -> Result<(), mlua::Error> {
        self.0.load(user_lua).set_name("actions.lua").exec()?;
        Ok(())
    }

    pub fn invoke_action(
        &mut self,
        action_name: &str,
        pixylene: Rc<RefCell<libpixylene::Pixylene>>,
        console: Rc<dyn pixylene_actions::Console>,
    ) -> Result<(), mlua::Error> {
        use crate::values::{project::Project, Console};

        let project_lua = Project(pixylene);
        self.0.globals().set("Project", project_lua).unwrap();
        self.0.globals().set("Console", Console(console)).unwrap();
        self.0
            .load(format!(
                "actions.{0}.perform(actions.{0}, Project, Console)",
                action_name
            ))
            .set_name("action invocation")
            .exec()?;
        self.0.globals().set("Project", Value::Nil)?;
        self.0.globals().set("Console", Value::Nil)?;

        Ok(())
    }

    pub fn invoke(
        &mut self,
        statement: &str,
        pixylene: Rc<RefCell<libpixylene::Pixylene>>,
        console: Rc<dyn pixylene_actions::Console>,
    ) -> Result<(), mlua::Error> {
        use crate::values::{project::Project, Console};

        let project_lua = Project(pixylene);
        self.0.globals().set("Project", project_lua).unwrap();
        self.0.globals().set("Console", Console(console)).unwrap();
        self.0.load(statement).set_name("console-input").exec()?;
        self.0.globals().set("Project", Value::Nil)?;
        self.0.globals().set("Console", Value::Nil)?;

        Ok(())
    }

    pub fn list_actions(&self) -> Vec<String> {
        self.0
            .globals()
            .get::<_, Table>("actions")
            .unwrap()
            .pairs::<String, Table>()
            .map(|pair| pair.unwrap().0)
            .collect::<Vec<String>>()
    }

    fn add_actions_table(lua_ctx: &mut mlua::Lua) -> Result<(), mlua::Error> {
        lua_ctx.globals().set("actions", lua_ctx.create_table()?)?;
        Ok(())
    }

    fn add_types(lua_ctx: &mut mlua::Lua) -> Result<(), mlua::Error> {
        let coord;
        let ucoord;
        let pcoord;
        let true_pixel;
        let indexed_pixel;
        let blend_mode;
        let true_scene;
        let indexed_scene;
        let true_layer;
        let indexed_layer;
        let palette;
        let true_canvas;
        let indexed_canvas;
        let log_type;

        //Construct Initial Pixylene Types
        {
            use pixylene_actions::LogType;
            use project::*;
            use types::*;

            coord = Coord::zero();
            ucoord = UCoord::zero();
            pcoord = PCoord::new(1, 1).unwrap();
            true_pixel = TruePixel::empty();
            indexed_pixel = IndexedPixel::empty();
            blend_mode = BlendMode::Normal;
            true_scene = Scene::<TruePixel>::new(pcoord, vec![None]).unwrap();
            indexed_scene = Scene::<IndexedPixel>::new(pcoord, vec![None]).unwrap();
            true_layer = Layer::<TruePixel>::new_with_solid_color(pcoord, None);
            indexed_layer = Layer::<IndexedPixel>::new_with_solid_color(pcoord, None);
            palette = Palette::new();
            true_canvas = Canvas {
                layers: LayersType::True(Layers::<TruePixel>::new(pcoord)),
                palette: palette.clone(),
            };
            indexed_canvas = Canvas {
                layers: LayersType::Indexed(Layers::<IndexedPixel>::new(pcoord)),

                palette: palette.clone(),
            };
            log_type = LogType::Info;
        }

        //Add Pixylene Types to Lua Global State
        {
            use crate::values::{project::*, types::*, LogType};

            lua_ctx.globals().set("C", Coord(coord))?;
            lua_ctx.globals().set("UC", UCoord(ucoord))?;
            lua_ctx.globals().set("PC", PCoord(pcoord))?;
            lua_ctx.globals().set("TP", TruePixel(true_pixel))?;
            lua_ctx.globals().set("IP", IndexedPixel(indexed_pixel))?;
            lua_ctx.globals().set("BlendMode", BlendMode(blend_mode))?;
            lua_ctx
                .globals()
                .set("TScene", TrueScene(Context::Solo(true_scene)))?;
            lua_ctx
                .globals()
                .set("IScene", IndexedScene(Context::Solo(indexed_scene)))?;
            lua_ctx
                .globals()
                .set("SLayer", TrueLayer(Context::Solo(true_layer)))?;
            lua_ctx
                .globals()
                .set("ILayer", IndexedLayer(Context::Solo(indexed_layer)))?;
            lua_ctx
                .globals()
                .set("Palette", Palette(Context::Solo(palette)))?;
            lua_ctx
                .globals()
                .set("TCanvas", Canvas(Context::Solo(true_canvas)))?;
            lua_ctx
                .globals()
                .set("ICanvas", Canvas(Context::Solo(indexed_canvas)))?;
            lua_ctx.globals().set("LogType", LogType(log_type))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn main() -> Result<(), tealr::mlu::mlua::Error> {
        use crate::LuaActionManager;
        use libpixylene::{
            project::Palette,
            types::{Coord, PCoord},
            PixyleneDefaults,
        };
        use pixylene_actions::{Console, LogType};
        use std::cell::RefCell;
        use std::rc::Rc;

        let pixylene = Rc::new(RefCell::new(libpixylene::Pixylene::new(
            /*defaults*/
            &PixyleneDefaults {
                dim: PCoord::new(10, 10).unwrap(),
                palette: Palette::new(),
                repeat: PCoord::new(1, 1).unwrap(),
            },
            /*indexed*/ false,
        )));

        struct ExampleConsole(pub Rc<RefCell<String>>);
        impl Console for ExampleConsole {
            fn cmdin(&self, _message: &str) -> Option<String> {
                Some(String::from("hi"))
            }
            fn cmdout(&self, message: &str, _log_type: &LogType) {
                *self.0.borrow_mut() = message.to_owned();
            }
        }
        let console = Rc::new(ExampleConsole(Rc::new(RefCell::new(String::new()))));

        let mut lam = LuaActionManager::new()?;
        lam.load(
            &r#"
            actions['test'] = {
                count = 1300135,
                perform = function(self, project, console)
                    project.focus = { ['coord'] = C(-69,420), ['layer'] = 999 }
                    console:cmdout("haii")
                end
            }
            "#
            .to_owned(),
        )?;
        lam.invoke_action("test", pixylene.clone(), console.clone())?;
        assert_eq!(
            pixylene.borrow().project.focus,
            (Coord { x: -69, y: 420 }, 999),
        );
        assert_eq!(console.0.borrow().clone(), String::from("haii"));

        lam.invoke(
            &"Console:cmdout('| ' .. actions['test'].count .. ' |')".to_owned(),
            pixylene.clone(),
            console.clone(),
        )?;
        assert_eq!(console.0.borrow().clone(), String::from("| 1300135 |"));
        Ok(())
    }
}
