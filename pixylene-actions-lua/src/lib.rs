pub mod values;
pub mod utils;

use crate::values::{ types::*, project::* };
use utils::messages;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}



//



#[cfg(test)]
mod tests {
    use super::*;
    use tealr::mlu::mlua::{ Lua, Table, Result };
    use std::io::Read;
    //use std::sync::{ Arc, Mutex };

    //use tealr::{ TypeWalker };
    use libpixylene::{ types, project };
    
    #[test]
    fn main() -> Result<()> {
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

        //Add User Actions
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
            lua_ctx.globals().set("Pixel", Pixel(types::Pixel::empty()))?;
            lua_ctx.globals().set("BlendMode", BlendMode(types::BlendMode::Normal))?;
            lua_ctx.globals().set("Scene", Scene(project::Scene::new(
                        types::PCoord::new(1,1).expect(messages::PCOORD_NOTFAIL),
                        vec![None]
            ).expect(messages::SCENE_NOTFAIL)))?;
            lua_ctx.globals().set("Layer", Layer(project::Layer::new_with_solid_color(
                        types::PCoord::new(1,1).expect(messages::PCOORD_NOTFAIL), None)))?;
            lua_ctx.globals().set("Palette", Palette(project::Palette::new()));
        }


        //Invoke User's Action script
        lua_ctx.load(r#"actions.example.perform(actions.echo)"#).exec().unwrap();

        Ok(())
    }
}
