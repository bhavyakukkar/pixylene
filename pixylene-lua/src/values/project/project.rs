use crate::{
    utils::BOXED_ERROR,
    values::{ types::{ Coord, UCoord, PCoord }, project::{ Canvas } },
    Context,
};

use tealr::{
    mlu::{
        mlua::{
            self, UserData, UserDataFields, UserDataMethods, Error::ExternalError,
        },
        TealData, TealDataMethods, UserDataWrapper,
    },
    ToTypename, TypeBody, mlua_create_named_parameters,
};
use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;
use libpixylene::Pixylene;


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

        //Project never needs to be constructed from lua

        //Lua inteface to is_cursor_at()
        {
            mlua_create_named_parameters!(
                ProjectIsCursorAtArgs with
                    coord: UCoord,
                    layer: u16,
            );
            methods.document("Returns whether there is a cursor at the provided coordinate on the \
                             layer at given layer index");
            methods.add_method_mut("is_cursor_at", |_, this, a: ProjectIsCursorAtArgs| {
                this.0.borrow().project.is_cursor_at(&(a.coord.0, a.layer))
                    .map_err(|err| ExternalError(Arc::from(BOXED_ERROR(&err.to_string()))))
            });
        }

        //todo: add interfaces to methods toggle_cursor_at & clear_cursors

        methods.generate_help();
    }

    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {
        fields.document("the Output-Multiplier of the Project");
        fields.add_field_method_get("mul", |_, this| Ok(this.0.borrow().project.get_out_mul()));
        fields.add_field_method_set("mul", |_, this, value: u8| 
            this.0.borrow_mut().project.set_out_mul(value)
                .map_err(|err| ExternalError(Arc::from(BOXED_ERROR(&err.to_string()))))
        );

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
            this.0.borrow_mut().project.canvas = canvas.0.do_imt
                (|canvas| canvas.clone())
                (|pixylene, _| pixylene.project.canvas.clone());
            Ok(())
        });

        fields.document("the number of cursors in the Project");
        fields.add_field_method_get("num_cursors", |_, this|
            Ok(this.0.borrow().project.num_cursors()));

        fields.document("the cursors in the Project");
        fields.add_field_method_get("cursors", |lua_ctx, this| {
            let mut cursors = Vec::new();
            for (coord, layer) in this.0.borrow().project.cursors() {
                let element = lua_ctx.create_table()?;
                element.set("coord", UCoord(coord.clone()))?;
                element.set("layer", *layer)?;
                cursors.push(element.clone());
            }
            Ok(cursors)
        });

        fields.document("table containing the focussed Layer ('layer') & focussed coordinate on \
                        the Layer ('coord') of the Project");
        fields.add_field_method_get("focus", |lua_ctx, this| {
            let focus = lua_ctx.create_table()?;
            focus.set("coord", Coord(this.0.borrow().project.focus.0))?;
            focus.set("layer", this.0.borrow().project.focus.1)?;
            Ok(focus)
        });
        fields.add_field_method_set("focus", |_, this, value: mlua::Table| {
            let coord: Coord = value.get("coord")?;
            this.0.borrow_mut().project.focus = (coord.0, value.get("layer")?);
            Ok(())
        });
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
