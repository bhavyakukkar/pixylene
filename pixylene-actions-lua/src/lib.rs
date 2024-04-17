use tealr::mlu::mlua::{ Lua, Table, Result };
//use std::io::Read;
use libpixylene::{ Pixylene, project, types };
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

pub mod values;
pub mod utils;


#[derive(Clone)]
enum Context<T: Default, U> {
    Solo(T),
    Linked(Rc<RefCell<Pixylene>>, U),
}

impl<T: Default, U> Context<T, U> {

    //Mutably do & return something with this context by passing what to do in both cases
    fn do_mut<FS, FL, S>(&mut self, f_solo: FS, f_linked: FL) -> S
    where
        FS: Fn(&mut T) -> S,
        FL: Fn(RefMut<'_, Pixylene>, &U) -> S
    {
        match self {
            Context::Solo(t) => f_solo(t),
            Context::Linked(p, data) => f_linked(p.borrow_mut(), data),
        }
    }

    //Immutably do & return something with this context by passing what to do in both cases
    fn do_imt<FS, FL, S>(&self, f_solo: FS, f_linked: FL) -> S
    where
        FS: Fn(&T) -> S,
        FL: Fn(Ref<'_, Pixylene>, &U) -> S
    {
        match self {
            Context::Solo(t) => f_solo(t),
            Context::Linked(p, data) => f_linked(p.borrow(), data),
        }
    }
}

impl<T: Default, U> Default for Context<T, U> {
    fn default() -> Context<T, U> {
        Context::Solo(Default::default())
    }
}


pub struct LuaActionManager(Lua);

impl LuaActionManager {
    pub fn invoke(&mut self, action_name: &str, pixylene: Rc<RefCell<libpixylene::Pixylene>>,
                  console: Rc<dyn pixylene_actions::Console>)
        -> Result<()>
    {
        use crate::values::{ Console, project::Project };

        let project_lua = Project(pixylene);
        self.0.globals().set("Project", project_lua).unwrap();
        self.0.globals().set("Console", Console(console)).unwrap();
        self.0.load(format!("actions.{0}.perform(actions.{0}, Project, Console)", action_name))
            .exec()?;
        //self.0.globals().set("Project", Value::Nil)?;

        Ok(())
    }

    pub fn list_actions(&self) -> Vec<String> {
        self.0.globals().get::<_, Table>("actions").unwrap().pairs::<String, Table>()
            .map(|pair| pair.unwrap().0).collect::<Vec<String>>()
    }

    pub fn setup(user_lua: &String) -> Result<LuaActionManager> {
        let lua_ctx = Lua::new();
    
        let coord;
        let ucoord;
        let pcoord;
        let pixel;
        let blend_mode;
        let scene;
        let layer;
        let palette;
        let canvas;
        //let project;

        //let console;
        let log_type;
    
        //Add User Actions
        {
            //Set Actions table
            lua_ctx.globals().set("actions", lua_ctx.create_table()?)?;

            //Load script containing 1 or more actions
            lua_ctx.load(user_lua).exec()?;

            /*
            actions_tbl.set(
                lua_ctx.globals().get::<_, Table>("action")?.get::<_, String>("name")?,
                lua_ctx.globals().get::<_, Table>("action")?
            )?;
            */
            //lua_user_actions.push(lua_ctx.globals().get::<&str, Table>("action")?);
            //lua_ctx.globals().set("action", lua_ctx.create_table()?)?;
    
            //lua_ctx.globals().set("actions", actions_tbl)?;
        }
        
        //Construct Initial Pixylene Types
        {
            use types::*;
            use project::*;
            use pixylene_actions::LogType;
    
            coord = Coord::zero();
            ucoord = UCoord::zero();
            pcoord = PCoord::new(1,1).unwrap();
            pixel = Pixel::empty();
            blend_mode = BlendMode::Normal;
            scene = Scene::new(pcoord, vec![None]).unwrap();
            layer = Layer::new_with_solid_color(pcoord, None);
            palette = Palette::new();
            canvas = Canvas::new(pcoord, palette.clone());
            //project = Project::new(canvas.clone());
    
            //struct ExampleConsole;
            //impl Console for ExampleConsole {
            //    fn cmdin(&self, _message: &str) -> Option<String> { None }
            //    fn cmdout(&self, _message: &str, _log_type: &LogType) { () }
            //}
            //console = ExampleConsole;
            log_type = LogType::Info;
        }
    
        //Add Pixylene Types to Lua Global State
        {
            use crate::values::{ LogType, types::*, project::* };
    
            lua_ctx.globals().set("C", Coord(coord))?;
            lua_ctx.globals().set("UC", UCoord(ucoord))?;
            lua_ctx.globals().set("PC", PCoord(pcoord))?;
            lua_ctx.globals().set("P", Pixel(pixel))?;
            lua_ctx.globals().set("BlendMode", BlendMode(blend_mode))?;
            lua_ctx.globals().set("Scene", Scene(Context::Solo(scene)))?;
            lua_ctx.globals().set("Layer", Layer(Context::Solo(layer)))?;
            lua_ctx.globals().set("Palette", Palette(Context::Solo(palette)))?;
            lua_ctx.globals().set("Canvas", Canvas(Context::Solo(canvas)))?;
            //lua_ctx.globals().set("Project", Project(project))?;
    
            //lua_ctx.globals().set("Console", Console(Box::new(console)))?;
            lua_ctx.globals().set("LogType", LogType(log_type))?;
        }
    
        //Invoke User's Action script
        //lua_ctx.load(r#"actions.example.perform(actions.example)"#).exec().unwrap();
    
        Ok(LuaActionManager(lua_ctx))
    }
}


#[cfg(test)]
mod tests {
    use tealr::mlu::mlua::{ Result };

    #[test]
    fn main() -> Result<()> {
        use libpixylene::{ PixyleneDefaults, types::PCoord, project::Palette };
        use pixylene_actions::{ Console, LogType };
        use crate::LuaActionManager;
        use std::path::Path;
        use std::rc::Rc;
        use std::cell::RefCell;

        let pixylene = Rc::new(RefCell::new(libpixylene::Pixylene::new(&PixyleneDefaults {
            dim: PCoord::new(2, 2).unwrap(),
            palette: Palette::new(),
            repeat: PCoord::new(1, 1).unwrap(),
        })));

        struct ExampleConsole;
        impl Console for ExampleConsole {
            fn cmdin(&self, _message: &str) -> Option<String> { Some(String::from("hi")) }
            fn cmdout(&self, message: &str, _log_type: &LogType) { println!("{}", message); }
        }

        let path = Path::new("/home/bhavya/.config/pixylene/actions.lua");
        let mut lam = LuaActionManager::setup(&std::fs::read_to_string(path).unwrap())?;
        lam.invoke("test", pixylene.clone(), Rc::new(ExampleConsole))?;

        Ok(())
    }
}
