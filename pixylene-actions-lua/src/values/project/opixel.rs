use crate::utils::messages;

use tealr::{
    mlu::{
        mlua::{
            self,
            prelude::{ LuaValue },
            FromLua, Lua, Result, UserData, UserDataFields, UserDataMethods, MetaMethod,
        },
        TealData, TealDataMethods, UserDataWrapper,
    },
    ToTypename, TypeBody, mlua_create_named_parameters,
};
use std::sync::Arc;
use libpixylene::types;


/// Lua interface to libpixylene's [`OPixel`](project::OPixel) type
#[derive(Clone)]
pub struct OPixel(pub types::OPixel);

impl<'lua> FromLua<'lua> for OPixel {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> Result<Self> {
        match value.as_userdata() {
            Some(ud) => Ok(*ud.borrow::<Self>()?),
            None => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "OPixel",
                message: None,
            }),
        }
    }
}

impl TealData for OPixel {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        use mlua::Error::{ ExternalError };

        methods.document_type("An Output Pixel as rendered by Scene's render method.");

        //Lua interface to construct OPixel::Filled
        {
            mlua_create_named_parameters!(
                OPixelFilledArgs with
                    scene_coord: UCoord,
                    color : Pixel,
                    is_focus: bool,
                    has_cursor: bool,
            );
            methods.document("Creates & returns a filled OPixel with the scene-coordinate being \
                             pointed to, the color, whether it points to the focus, and whether \
                             it has a cursor pointing to it.");
            methods.add_function("FILLED", |_, a: OPixelFilledArgs| {
                Ok(OPixel(project::OPixel::Filled {
                    scene_coord: a.scene_coord,
                    color: a.color,
                    is_focus: a.is_focus,
                    has_cursor: a.has_cursor,
                }))
            });
        }

        //Lua interface to construct OPixel::Empty
        {
            mlua_create_named_parameters!(
                OPixelEmptyArgs with
                    x : u16,
                    y : u16,
            );

            methods.document("Creates & returns an empty OPixel with the scene-coordinate being \
                             pointed to and whether it has a cursor pointing to it.");
            methods.add_function("EMPTY", |_, a: OPixelEmptyArgs| {
                Ok(OPixel(project::OPixel::Empty {
                    scene_coord: a.scene_coord,
                    has_cursor: a.has_cursor,
                }))
            });
        }

        //Lua interface to OPixel::area
        {
            methods.document("Return the 'area' of a OPixel, i.e., product of x and y");
            methods.add_method("area", |_, this, _: ()| -> Result<u32> {
                Ok(this.0.area())
            });
        }

        //Lua metamethod '+' interface to OPixel::add
        {
            mlua_create_named_parameters!(
                OPixelAddArgs with
                    first : OPixel,
                    second : OPixel,
            );

            methods.document("Return a OPixel composed of the sums of two OPixel's coordinates, \
                             failing if the addition would overflow");
            methods.add_meta_function(MetaMethod::Add, |_, a: OPixelAddArgs| {
                let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

                let x = a.first.0.x().checked_add(a.second.0.x())
                    .ok_or(ExternalError(Arc::from(
                        boxed_error("Addition of x coordinates of the two OPixel's has overflowed")
                    )))?;
                let y = a.first.0.y().checked_add(a.second.0.y())
                    .ok_or(ExternalError(Arc::from(
                        boxed_error("Addition of y coordinates of the two OPixel's has overflowed")
                    )))?;

                Ok(OPixel(types::OPixel::new(x,y).expect(messages::PCOORD_NOTFAIL)))
            });
        }

        methods.generate_help();
    }

    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {
        use mlua::Error::{ ExternalError };

        fields.document("the 'x' coordinate of the OPixel");
        fields.add_field_method_get("x", |_, this| Ok(this.0.x()));
        fields.add_field_method_set("x", |_, this, value| {
            let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

            this.0.set_x(value)
                .map_err(|_| ExternalError(Arc::from(
                    boxed_error("Trying to set x to 0 for OPixel")
                )))
        });

        fields.document("the 'y' coordinate of the OPixel");
        fields.add_field_method_get("y", |_, this| Ok(this.0.y()));
        fields.add_field_method_set("y", |_, this, value| {
            let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

            this.0.set_y(value)
                .map_err(|_| ExternalError(Arc::from(
                    boxed_error("Trying to set y to 0 for OPixel")
                )))
        });
    }
}

impl ToTypename for OPixel {
    fn to_typename() -> tealr::Type {
        tealr::Type::new_single("OPixel", tealr::KindOfType::External)
    }
}

impl UserData for OPixel {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        let mut wrapper = UserDataWrapper::from_user_data_methods(methods);
        <Self as TealData>::add_methods(&mut wrapper)
    }
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        let mut wrapper = UserDataWrapper::from_user_data_fields(fields);
        <Self as TealData>::add_fields(&mut wrapper)
    }
}

impl TypeBody for OPixel {
    fn get_type_body() -> tealr::TypeGenerator {
        let mut gen = tealr::RecordGenerator::new::<Self>(false);
        gen.is_user_data = true;
        <Self as TealData>::add_fields(&mut gen);
        <Self as TealData>::add_methods(&mut gen);
        gen.into()
    }
}
