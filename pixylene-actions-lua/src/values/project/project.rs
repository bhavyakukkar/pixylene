use crate::{
    Context,
    values::{ types::{ Coord, UCoord, PCoord }, project::{ Canvas } }
};

use tealr::{
    mlu::{
        mlua::{
            self,
            UserData, UserDataFields, UserDataMethods,
        },
        TealData, TealDataMethods, UserDataWrapper,
    },
    ToTypename, TypeBody, mlua_create_named_parameters,
};
use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;
use libpixylene::{ Pixylene };


/// Lua interface to libpixylene's [`Project`][project::Project] type
//#[derive(Clone)]
pub struct Project(pub Rc<RefCell<Pixylene>>);

// No FromLua impl because Project never needs to be constructed from lua
/*
impl<'lua> FromLua<'lua> for Project {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> Result<Project> {
        match value.as_userdata() {
            Some(ud) => Ok((*ud.borrow::<Project>()?).clone()),
            None => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Project",
                message: None,
            }),
        }
    }
}
*/

impl TealData for Project {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("The absolute state of a Pixel Art project at any given instance.");

        //Lua interface to new()
        /*
        {
            mlua_create_named_parameters!(
                ProjectArgs with
                    canvas: Canvas,
            );
            methods.document("Creates a new empty Project containing the given Canvas");
            methods.add_meta_method(MetaMethod::Call, |_, _, a: ProjectArgs| {
                Ok(Project(project::Project::new(a.canvas.0)))
            });
        }
        */

        //Lua interface to render_layer()
        //this isn't usual for an action to use
        /*
        {
            methods.document("todo");
            methods.add_method("render_layer", |_, this, _| {
                use mlua::Error::{ ExternalError };
                let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

                match this.0.render_layer() {
                    Ok(buffer) => Ok(
                }
                Ok(Scene(this.0.merged_scene(match a.background {
                    Some(color) => Some(color.0),
                    None => None
                })))
            });
        }
        */

        //Lua inteface to set_out_mul()
        {
            mlua_create_named_parameters!(
                ProjectSetMulArgs with
                    new_mul: u8,
            );
            methods.document("Sets the output multiplier for this given Project, failing if 0 is \
                             passed");
            methods.add_method_mut("set_mul", |_, this, a: ProjectSetMulArgs| {
                use mlua::Error::{ ExternalError };
                let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

                match this.0.borrow_mut().project.set_out_mul(a.new_mul) {
                    Ok(()) => Ok(()),
                    Err(err) => Err(ExternalError(Arc::from(
                        boxed_error(&err.to_string())
                    ))),
                }
            });
        }


        //Flexible Lua interface to get or set Project's focus field
        {
            mlua_create_named_parameters!(
                ProjectFocusArgs with
                    coord: Option<Coord>,
                    layer: Option<u16>,
            );
            methods.document("Gets the output focus as a table of fields 'coord' and 'layer' for \
                             this given Project if nothing is passed, sets it if the coordinate \
                             and layer are passed");
            methods.add_method_mut("focus", |lua, this, a: ProjectFocusArgs| {
                if a.coord.is_some() && a.layer.is_some() {
                    this.0.borrow_mut().project.focus = (a.coord.unwrap().0, a.layer.unwrap());
                    Ok(None)
                } else {
                    let focus = lua.create_table()?;
                    focus.set("coord", Coord(this.0.borrow().project.focus.0))?;
                    focus.set("layer", this.0.borrow().project.focus.1)?;
                    Ok(Some(focus))
                }
            });
        }

        //todo: add more methods

        methods.generate_help();
    }

    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {
        fields.document("the Output-Multiplier of the Project");
        fields.add_field_method_get("mul", |_, this| Ok(this.0.borrow().project.get_out_mul()));

        fields.document("the Output-Dimensions of the Project (not to be confused with the \
                        Canvas's dimensions)");
        fields.add_field_method_get("dim", |_, this| Ok(PCoord(this.0.borrow().project.out_dim)));
        fields.add_field_method_set("dim", |_, this, value: PCoord| {
            this.0.borrow_mut().project.out_dim = value.0;
            Ok(())
        });

        fields.document("the Output-Repeat of the Project");
        fields.add_field_method_get("repeat", |_, this| Ok(PCoord(this.0.borrow().project
                                                                  .out_repeat)));
        fields.add_field_method_set("repeat", |_, this, value: PCoord| {
            this.0.borrow_mut().project.out_repeat = value.0;
            Ok(())
        });

        fields.document("the Canvas contained by the Project");
        fields.add_field_method_get("canvas", |_, this|
            Ok(Canvas(Context::Linked(this.0.clone(), ()))));

        fields.add_field_method_set("canvas", |_, this, canvas: Canvas| {
            this.0.borrow_mut().project.canvas = canvas.0.do_imt(
                |canvas| canvas.clone(),
                |pixylene, _| pixylene.project.canvas.clone()
            );
            Ok(())
        });

        fields.document("the number of cursors in the Project");
        fields.add_field_method_get("num_cursors", |_, this|
            Ok(this.0.borrow().project.num_cursors()));

        fields.document("the cursors in the Project");
        fields.add_field_method_get("cursors", |lua_ctx, this| {
            let element = lua_ctx.create_table()?;
            let mut cursors = Vec::new();
            for (coord, layer) in this.0.borrow().project.cursors() {
                element.set("coord", UCoord(coord.clone()))?;
                element.set("layer", *layer)?;
                cursors.push(element.clone());
            }
            Ok(cursors)
        });
            //Ok(this.0.borrow().project.cursors()
            //   .map(|(coord, layer)| lua_ctx.create_table()
            //           .map(|table| => {
            //               table.set("coord", UCoord(coord.clone()));
            //               table.set("layer", *layer);
            //           })
            //   })
            //   .collect::<Vec<(UCoord, u16)>>()));
    }
}

impl ToTypename for Project {
    fn to_typename() -> tealr::Type {
        tealr::Type::new_single("Project", tealr::KindOfType::External)
    }
}

impl UserData for Project {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        let mut wrapper = UserDataWrapper::from_user_data_methods(methods);
        <Self as TealData>::add_methods(&mut wrapper)
    }
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        let mut wrapper = UserDataWrapper::from_user_data_fields(fields);
        <Self as TealData>::add_fields(&mut wrapper)
    }
}

impl TypeBody for Project {
    fn get_type_body() -> tealr::TypeGenerator {
        let mut gen = tealr::RecordGenerator::new::<Self>(false);
        gen.is_user_data = true;
        <Self as TealData>::add_fields(&mut gen);
        <Self as TealData>::add_methods(&mut gen);
        gen.into()
    }
}
