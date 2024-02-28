use crate::utils::messages;

use tealr::{
    mlu::{
        mlua::{
            prelude::{ LuaValue, LuaUserData },
            FromLua, Value, Lua, Result, UserData, UserDataMethods, MetaMethod,
        },
        self, TealData, TealDataMethods, UserDataWrapper,
    },
    ToTypename, TypeBody, TypeWalker, mlua_create_named_parameters,
};
use std::sync::Arc;
use libpixylene::types;


/// Lua interface to [`types::PCoord`]
#[derive(Copy, Clone)]
pub struct PCoord(pub types::PCoord);

impl<'lua> FromLua<'lua> for PCoord {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> Result<Self> {
        match value.as_userdata() {
            Some(ud) => Ok(*ud.borrow::<Self>()?),
            None => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "PCoord",
                message: None,
            }),
        }
    }
}

impl TealData for PCoord {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        use mlu::mlua::Error::{ ExternalError };

        methods.document_type("A positive coordinate type composed of two positive (1 or greater) \
                              16-bit unsigned integers.");
        //Lua interface to [`types::PCoord::new`]
        {
            mlua_create_named_parameters!(
                PCoordNewArgs with
                    x : u16,
                    y : u16,
            );

            methods.document("Try to create & return a new PCoord with 'x' and 'y' coordinates");
            methods.add_function("new", |_, a: PCoordNewArgs| {
                // thanks to https://github.com/Blightmud/Blightmud/blob/dev/src/lua/timer.rs for
                // this
                let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

                match types::PCoord::new(a.x, a.y) {
                    Ok(p) => Ok(PCoord(p)),
                    Err(()) => Err(ExternalError(Arc::from(boxed_error(
                        &format!(
                            "Parameters passed to PCoord.new were found not to be positive, \
                            found: ({}, {})",
                            a.x,
                            a.y,
                        )
                    )))),
                }
            });
        }

        //Lua interface to [`types::PCoord::area`]
        {
            methods.document("Return the 'area' of a PCoord, i.e., product of x and y");
            methods.add_method("area", |_, this, _: ()| -> Result<u32> {
                Ok(this.0.area())
            });
        }

        //Lua metamethod '+' interface to [`types::PCoord::add`]
        {
            mlua_create_named_parameters!(
                PCoordAddArgs with
                    first : PCoord,
                    second : PCoord,
            );

            methods.document("Return a PCoord composed of the sums of two PCoord's coordinates, \
                             failing if the addition would overflow");
            methods.add_meta_function(MetaMethod::Add, |_, a: PCoordAddArgs| {
                let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

                let x = a.first.0.x().checked_add(a.second.0.x())
                    .ok_or(ExternalError(Arc::from(
                        boxed_error("Addition of x coordinates of the two PCoord's has overflowed")
                    )))?;
                let y = a.first.0.y().checked_add(a.second.0.y())
                    .ok_or(ExternalError(Arc::from(
                        boxed_error("Addition of y coordinates of the two PCoord's has overflowed")
                    )))?;

                Ok(PCoord(types::PCoord::new(x,y).expect(messages::PCOORD_NOTFAIL)))
            });
        }

        methods.generate_help();
    }
    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {
        use mlu::mlua::Error::{ ExternalError };

        fields.add_field_method_get("x", |_, this| Ok(this.0.x()));
        fields.add_field_method_set("x", |_, this, value| {
            let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

            this.0.set_x(value)
                .map_err(|err| ExternalError(Arc::from(
                    boxed_error("Trying to set x to 0 for PCoord")
                )))
        });
        fields.add_field_method_get("y", |_, this| Ok(this.0.y()));
        fields.add_field_method_set("y", |_, this, value| {
            let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

            this.0.set_y(value)
                .map_err(|err| ExternalError(Arc::from(
                    boxed_error("Trying to set y to 0 for PCoord")
                )))
        });
    }
}

impl ToTypename for PCoord {
    //how the type should be called in lua.
    fn to_typename() -> tealr::Type {
        tealr::Type::new_single("PCoord", tealr::KindOfType::External)
    }
}

impl UserData for PCoord {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        let mut wrapper = UserDataWrapper::from_user_data_methods(methods);
        <Self as TealData>::add_methods(&mut wrapper)
    }
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        let mut wrapper = UserDataWrapper::from_user_data_fields(fields);
        <Self as TealData>::add_fields(&mut wrapper)
    }
}

impl TypeBody for PCoord {
    fn get_type_body() -> tealr::TypeGenerator {
        let mut gen = tealr::RecordGenerator::new::<Self>(false);
        gen.is_user_data = true;
        <Self as TealData>::add_fields(&mut gen);
        <Self as TealData>::add_methods(&mut gen);
        gen.into()
    }
}
