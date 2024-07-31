use super::TruePixel;

use libpixylene::types;
use std::sync::Arc;
use tealr::{
    mlu::{
        mlua::{
            self, prelude::LuaValue, FromLua, Lua, Result, UserData, UserDataFields,
            UserDataMethods,
        },
        TealData, TealDataMethods, UserDataWrapper,
    },
    mlua_create_named_parameters, ToTypename, TypeBody,
};

/// Lua interface to libpixylene's [`BlendMode`](types::BlendMode) type
#[derive(Copy, Clone)]
pub struct BlendMode(pub types::BlendMode);

impl<'lua> FromLua<'lua> for BlendMode {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> Result<Self> {
        match value.as_userdata() {
            Some(ud) => Ok(*ud.borrow::<Self>()?),
            None => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "BlendMode",
                message: None,
            }),
        }
    }
}

impl TealData for BlendMode {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("An enumeration of the different types of blend-modes.");

        //Lua interface to construct BlendMode::Composite
        {
            mlua_create_named_parameters!(
                BlendModeCompositeArgs with
                    fraction_a : u8,
                    fraction_b : u8,
            );
            methods.document("Construct Blend-mode that composts pixels with given fractions");
            methods.add_function("COMPOSITE", |_, a: BlendModeCompositeArgs| {
                Ok(BlendMode(types::BlendMode::Composite(
                    a.fraction_a,
                    a.fraction_b,
                )))
            });
        }

        //Lua interface to BlendMode::blend()
        {
            mlua_create_named_parameters!(
                BlendModeBlendArgs with
                    top: TruePixel,
                    bottom: TruePixel,
            );
            methods.document("Blend two pixels and return the resultant pixel");
            methods.add_method("blend", |_, this, a: BlendModeBlendArgs| {
                use mlua::Error::ExternalError;
                let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

                match this.0.blend(a.top.0, a.bottom.0) {
                    Ok(p) => Ok(TruePixel(p)),
                    Err(err) => Err(ExternalError(Arc::from(boxed_error(&err.to_string())))),
                }
            });
        }

        //todo: Eq metamethod so blendmodes can be compared

        methods.generate_help();
    }
    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {
        fields.document("Blend-mode that uses the standard interpolation approach");
        fields.add_field_method_get("NORMAL", |_, _| Ok(BlendMode(types::BlendMode::Normal)));

        fields.document("Blend-mode that overwrites the top pixel onto the bottom pixel");
        fields.add_field_method_get("OVERWRITE", |_, _| {
            Ok(BlendMode(types::BlendMode::Overwrite))
        });
    }
}

impl ToTypename for BlendMode {
    fn to_typename() -> tealr::Type {
        tealr::Type::new_single("BlendMode", tealr::KindOfType::External)
    }
}

impl UserData for BlendMode {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        let mut wrapper = UserDataWrapper::from_user_data_methods(methods);
        <Self as TealData>::add_methods(&mut wrapper)
    }
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        let mut wrapper = UserDataWrapper::from_user_data_fields(fields);
        <Self as TealData>::add_fields(&mut wrapper)
    }
}

impl TypeBody for BlendMode {
    fn get_type_body() -> tealr::TypeGenerator {
        let mut gen = tealr::RecordGenerator::new::<Self>(false);
        gen.is_user_data = true;
        <Self as TealData>::add_fields(&mut gen);
        <Self as TealData>::add_methods(&mut gen);
        gen.into()
    }
}
