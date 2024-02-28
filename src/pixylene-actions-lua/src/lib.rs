mod values;
mod utils;
use crate::values::{ types::*, project::* };
use utils::messages;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}



//



#[cfg(test)]
mod tests {
    use super::*;
    use mlua::{ self, Lua, Table, prelude::{ LuaError }, Result };
    use std::io::Read;
    use std::sync::{ Arc, Mutex };

    use tealr::{ TypeWalker };
    use libpixylene::{ types, project };
    
    const LOCKERR: &str = "Concurrency of actions has been compromised";
    
    #[derive(Debug)]
    enum ProjectError {
        UnrecognizedFunction,
        MissingFunction,
        MissingArguments,
        InvalidArguments(LuaError),
    }
    impl From<LuaError> for ProjectError {
        fn from(item: LuaError) -> ProjectError {
            ProjectError::InvalidArguments(item)
        }
    }
    
    struct Project {
        layers: u8,
        lock: bool,
        pub focus: types::Coord,
        pub cursor: types::UCoord,
    }
    impl Project {
        pub fn new(layers: u8) -> Self {
            Self {
                layers,
                lock: false,
                focus: types::Coord::zero(),
                cursor: types::UCoord::zero(),
            }
        }
        pub fn add_layer(&mut self, by: u8) {
            self.layers += by;
            println!("layers now {}", self.layers);
        }
        pub fn lock(&mut self) -> Option<()> {
            if self.lock {
                println!("project has already been locked");
                None
            }
            else {
                self.lock = true;
                println!("project has been locked");
                Some(())
            }
        }
        pub fn focus_at(&mut self, new_focus: types::Coord) {
            self.focus = new_focus;
            println!("focus now {}", self.focus);
        }
    }

    #[test]
    fn main() -> Result<()> {
        let project = Arc::new(Mutex::new(Project::new(0)));

        let lua_ctx = Lua::new();

        let mut user_lua = String::new();
        let mut user_lua_file = std::fs::File::open("./src/example.lua").unwrap();
        user_lua_file.read_to_string(&mut user_lua).unwrap();

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

        //User Actions Context
        {
            //Set Actions table
            let actions_tbl = lua_ctx.create_table()?;

            for _ in 0..1 {
                //Load All User Action scripts
                {
                    //Dummy action table that will be moved and nil'ed after action
                    //logic is moved out of this context into main context
                    lua_ctx.globals().set("action", lua_ctx.create_table()?)?;
                    lua_ctx.load(&user_lua).exec()?;
                    actions_tbl.set(
                        lua_ctx.globals().get::<_, Table>("action")?.get::<_, String>("name")?,
                        lua_ctx.globals().get::<_, Table>("action")?
                    )?;
                    //lua_user_actions.push(lua_ctx.globals().get::<&str, Table>("action")?);
                    lua_ctx.globals().set("action", lua_ctx.create_table()?)?;
                }
            }

            lua_ctx.globals().set("actions", actions_tbl)?;
        }
        
        //Add Pixylene Types
        {
            lua_ctx.globals().set("Coord", Coord(types::Coord::zero()))?;
            lua_ctx.globals().set("UCoord", UCoord(types::UCoord::zero()))?;
            lua_ctx.globals().set("PCoord", PCoord(types::PCoord::new(1,1)
                                                   .expect(messages::PCOORD_NOTFAIL)))?;
        }

        //Project Context
        {
            //Set Project namespace
            let project_tbl = lua_ctx.create_table()?;

            {
                //Create Lua fns for Project fns
                let project_clone = Arc::clone(&project);
                let focus_at = lua_ctx.create_function(move |_, args: Coord| {
                    Ok(project_clone.lock().expect(LOCKERR).focus_at(args.0))
                })?;


                //Set Lua fns to Project namespace
                project_tbl.set("focus_at", focus_at)?;
            }

            //Add Project to globals
            lua_ctx.globals().set("project", project_tbl)?;
        }


        //Invoke User's Action script
        lua_ctx.load(r#"actions.echo.perform(actions.echo)"#).exec().unwrap();

        Ok(())
    }
}
