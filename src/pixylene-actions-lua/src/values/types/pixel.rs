use tealr::{
    mlu::{
        mlua::{
            self,
            prelude::{ LuaValue, LuaUserData },
            FromLua, Value, Lua, Result, UserData, UserDataFields, UserDataMethods, MetaMethod,
        },
        self, TealData, TealDataMethods, UserDataWrapper,
    },
    ToTypename, TypeBody, TypeWalker, mlua_create_named_parameters,
};

use std::sync::Arc;
use libpixylene::types;


/// Lua interface to libpixylene's [`Pixel`][types::Pixel]
#[derive(Copy, Clone)]
pub struct Pixel(pub types::Pixel);

impl<'lua> FromLua<'lua> for Pixel {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> Result<Self> {
        match value.as_userdata() {
            Some(ud) => Ok(*ud.borrow::<Self>()?),
            None => Err(mlu::mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Pixel",
                message: None,
            }),
        }
    }
}

impl TealData for Pixel {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        use mlua::Error::{ ExternalError };

        methods.document_type("An RGBA type to represent a color, composed of 8-bit red, green, \
                              blue & alpha values.");

        //Flexible Lua call metamethod to construct a new RGBA Pixel
        {
            mlua_create_named_parameters!(
                PixelArgs with
                    r : Option<u8>,
                    g : Option<u8>,
                    b : Option<u8>,
                    a : Option<u8>,
            );
            methods.document("Create & return a new Pixel with optional red, green, blue & alpha \
                             levels, each between 0-255, defaulting to 0");
            methods.add_meta_method(MetaMethod::Call, |_, _, args: PixelArgs| {
                Ok(Pixel(types::Pixel{
                    r: args.r.unwrap_or(0),
                    g: args.g.unwrap_or(0),
                    b: args.b.unwrap_or(0),
                    a: args.a.unwrap_or(0),
                }))
            });
        }

        //Lua interface to construct a new Pixel with r, g, b, a
        {
            mlua_create_named_parameters!(
                PixelRgbaArgs with
                    r : u8,
                    g : u8,
                    b : u8,
                    a : u8,
            );
            methods.document("Create & return a new Pixel with specified red, green, blue & alpha \
                             levels, each between 0-255");
            methods.add_function("rgba", |_, args: PixelRgbaArgs| {
                Ok(Pixel(types::Pixel{r: args.r, g: args.g, b: args.b, a: args.a}))
            });
        }

        //Lua interface to construct a new Pixel with r, g, b (a defaults to 255)
        {
            mlua_create_named_parameters!(
                PixelRgbArgs with
                    r : u8,
                    g : u8,
                    b : u8,
            );
            methods.document("Create & return a new Pixel with specified red, green & blue \
                             levels, each between 0-255 (alpha defaults to 0)");
            methods.add_function("rgb", |_, args: PixelRgbArgs| {
                Ok(Pixel(types::Pixel{r: args.r, g: args.g, b: args.b, a: 255}))
            });
        }

        //todo: cmyk & hsl


        //Lua interface to construct a new Pixel with a hex-triplet (6 or 8 digits)
        {
            mlua_create_named_parameters!(
                PixelHexArgs with
                    s: String,
            );
            methods.document("Create & return a new Pixel with a specified CSS-like hex string, \
                             e.g. `#694269` or `#69426942`");
            methods.add_function("hex", |_, a: PixelHexArgs| {
                let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

                match types::Pixel::from_hex(a.s) {
                    Ok(p) => Ok(Pixel(p)),
                    Err(desc) => Err(ExternalError(Arc::from(
                        boxed_error(&desc.to_string())
                    ))),
                }
            });
        }

        methods.generate_help();
    }

    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {

        //todo: add constants RED, GREEN, BLUE, etc.

        fields.document("the red level of the Pixel");
        fields.add_field_method_get("r", |_, this| Ok(this.0.r));
        fields.add_field_method_set("r", |_, this, value| {
            this.0.r = value;
            Ok(())
        });

        fields.document("the green level of the Pixel");
        fields.add_field_method_get("g", |_, this| Ok(this.0.g));
        fields.add_field_method_set("g", |_, this, value| {
            this.0.g = value;
            Ok(())
        });

        fields.document("the blue level of the Pixel");
        fields.add_field_method_get("b", |_, this| Ok(this.0.b));
        fields.add_field_method_set("b", |_, this, value| {
            this.0.b = value;
            Ok(())
        });

        fields.document("the alpha level of the Pixel");
        fields.add_field_method_get("a", |_, this| Ok(this.0.a));
        fields.add_field_method_set("a", |_, this, value| {
            this.0.a = value;
            Ok(())
        });
    }
}

impl ToTypename for Pixel {
    fn to_typename() -> tealr::Type {
        tealr::Type::new_single("Pixel", tealr::KindOfType::External)
    }
}

impl UserData for Pixel {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        let mut wrapper = UserDataWrapper::from_user_data_methods(methods);
        <Self as TealData>::add_methods(&mut wrapper)
    }
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        let mut wrapper = UserDataWrapper::from_user_data_fields(fields);
        <Self as TealData>::add_fields(&mut wrapper)
    }
}

impl TypeBody for Pixel {
    fn get_type_body() -> tealr::TypeGenerator {
        let mut gen = tealr::RecordGenerator::new::<Self>(false);
        gen.is_user_data = true;
        <Self as TealData>::add_fields(&mut gen);
        <Self as TealData>::add_methods(&mut gen);
        gen.into()
    }
}
