use crate::values::{ types::{ Pixel }, project::{ Scene } };

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
use libpixylene::{ project };


/// Lua interface to libpixylene's [`Palette`][project::Palette] type
#[derive(Clone)]
pub struct Palette(pub project::Palette);

impl<'lua> FromLua<'lua> for Palette {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> Result<Palette> {
        match value.as_userdata() {
            Some(ud) => Ok((*ud.borrow::<Palette>()?).clone()),
            None => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Palette",
                message: None,
            }),
        }
    }
}

impl TealData for Palette {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("A color palette of indexed colors where the significant color at \
                              any time can be chosen and equipped.");

        //Lua interface to Palette::from()
        {
            methods.document("Creates & returns a new Palette from a table of pairs of color \
                             indexes and hex-strings, \
                             e.g.: Palette{{1, \"#ffffff\"}, {2, \"#000000\"}}");
            methods.add_meta_method(MetaMethod::Call, |_, _, a: mlua::Table| {
                use mlua::Error::{ ExternalError };
                let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

                let mut palette = project::Palette::new();

                for color_pair in a.pairs::<u8, String>() {
                    let (index, color) = color_pair?;
                    match palette.set_color(index, &color) {
                        Ok(()) => (),
                        Err(err) => {
                            return Err(ExternalError(Arc::from(boxed_error(&err.to_string()))));
                        },
                    }
                }

                Ok(Palette(palette))
            });
        }

        //Lua interface to Palette::get_color()
        {
            mlua_create_named_parameters!(
                PaletteGetColorArgs with
                    index: u8,
            );
            methods.document("Gets the color at an index of the Palette");
            methods.add_method("get_color", |_, this, a: PaletteGetColorArgs| {
                use mlua::Error::{ ExternalError };
                let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

                match this.0.get_color(a.index) {
                    Ok(pixel) => Ok(Pixel(pixel.clone())),
                    Err(err) => Err(ExternalError(Arc::from(
                        boxed_error(&err.to_string())
                    ))),
                }
            });
        }

        //Lua interface to Palette::equipped()
        {
        }

        // todo: + metamethod alias for merge that checks for consistent sizes and uses top layer's
        //       blend_mode

        methods.generate_help();
    }

    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {
        //over here
    }
}

impl ToTypename for Palette {
    fn to_typename() -> tealr::Type {
        tealr::Type::new_single("Palette", tealr::KindOfType::External)
    }
}

impl UserData for Palette {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        let mut wrapper = UserDataWrapper::from_user_data_methods(methods);
        <Self as TealData>::add_methods(&mut wrapper)
    }
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        let mut wrapper = UserDataWrapper::from_user_data_fields(fields);
        <Self as TealData>::add_fields(&mut wrapper)
    }
}

impl TypeBody for Palette {
    fn get_type_body() -> tealr::TypeGenerator {
        let mut gen = tealr::RecordGenerator::new::<Self>(false);
        gen.is_user_data = true;
        <Self as TealData>::add_fields(&mut gen);
        <Self as TealData>::add_methods(&mut gen);
        gen.into()
    }
}
