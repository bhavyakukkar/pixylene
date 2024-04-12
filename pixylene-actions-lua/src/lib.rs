use tealr::mlu::mlua::{ Lua, Table, Result };
//use std::io::Read;
use libpixylene::{ project, types };
use std::rc::Rc;
use std::cell::RefCell;

pub mod values;

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
    
        //left here when need to generate teal docs
        /*
        let file_contents = TypeWalker::new()
            //tells it that you want to include the Example type
            //chain extra calls to include more types
            .process_type::<Coord>()
            //generate the file
            .to_json()
            .expect("serde_json failed to serialize our data");
        println!("{}\n ", file_contents);
        */
    
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
            lua_ctx.globals().set("Scene", Scene(scene))?;
            lua_ctx.globals().set("Layer", Layer(layer))?;
            lua_ctx.globals().set("Palette", Palette(palette))?;
            lua_ctx.globals().set("Canvas", Canvas(canvas))?;
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
            dim: PCoord::new(10, 10).unwrap(),
            palette: Palette::new(),
        })));

        struct ExampleConsole;
        impl Console for ExampleConsole {
            fn cmdin(&self, _message: &str) -> Option<String> { Some(String::from("hi")) }
            fn cmdout(&self, message: &str, _log_type: &LogType) { println!("{}", message); }
        }

        let mut lam = LuaActionManager::setup(Path::new("/home/bhavya/.config/pixylene.lua"))?;
        lam.invoke("example", pixylene.clone(), Rc::new(ExampleConsole))?;

        println!("{}", pixylene.borrow().project.canvas.dim());
        Ok(())
    }
}
