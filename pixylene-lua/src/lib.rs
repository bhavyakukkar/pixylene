use tealr::mlu::mlua::{ Lua, Table, Result };
//use std::io::Read;
use libpixylene::{ Pixylene, project, types };
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

pub mod values;
pub mod utils;


#[derive(Clone)]
pub enum Context<T, U> {
    Solo(T),
    Linked(Rc<RefCell<Pixylene>>, U),
}

impl<T, U> Context<T, U> {

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

pub enum ErrorType {
    ConfigError(mlua::Error),
    LuaError(mlua::Error),
}

pub struct LuaActionManager{
    ctx: Lua,
    pub error: Option<ErrorType>,
}

impl LuaActionManager {
    pub fn invoke(&mut self, action_name: &str, pixylene: Rc<RefCell<libpixylene::Pixylene>>,
                  console: Rc<dyn pixylene_actions::Console>)
        -> Result<()>
    {
        use crate::values::{ Console, project::Project };

        let project_lua = Project(pixylene);
        self.ctx.globals().set("Project", project_lua).unwrap();
        self.ctx.globals().set("Console", Console(console)).unwrap();
        self.ctx.load(format!("actions.{0}.perform(actions.{0}, Project, Console)", action_name))
            .exec()?;
        //self.ctx.globals().set("Project", Value::Nil)?;

        Ok(())
    }

    pub fn list_actions(&self) -> Vec<String> {
        self.ctx.globals().get::<_, Table>("actions").unwrap().pairs::<String, Table>()
            .map(|pair| pair.unwrap().0).collect::<Vec<String>>()
    }

    pub fn setup(user_lua: &String) -> LuaActionManager {
        use ErrorType::*;
        let mut lua_ctx = Lua::new();
        let mut error: Option<mlua::Error>;
    
        error = Self::add_actions_table(&mut lua_ctx).err();
        if let Some(err) = error {
            return LuaActionManager{ ctx: lua_ctx, error: Some(LuaError(err)) };
        }

        error = Self::add_types(&mut lua_ctx).err();
        if let Some(err) = error {
            return LuaActionManager{ ctx: lua_ctx, error: Some(LuaError(err)) };
        }

        //Load script containing 1 or more actions
        error = lua_ctx.load(user_lua).exec().err();
        if let Some(err) = error {
            return LuaActionManager{ ctx: lua_ctx, error: Some(ConfigError(err)) };
        }

        LuaActionManager{ ctx: lua_ctx, error: None }
    }

    fn add_actions_table(lua_ctx: &mut Lua) -> Result<()> {
        lua_ctx.globals().set("actions", lua_ctx.create_table()?)?;
        Ok(())
    }

    fn add_types(lua_ctx: &mut Lua) -> Result<()> {
        let coord;
        let ucoord;
        let pcoord;
        let pixel;
        let blend_mode;
        let scene;
        let layer;
        let palette;
        let canvas;
        let log_type;

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
            lua_ctx.globals().set("LogType", LogType(log_type))?;
        }
        Ok(())
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
