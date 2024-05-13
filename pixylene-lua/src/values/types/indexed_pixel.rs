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


/// Lua interface to libpixylene's [`IndexedPixel`](types::IndexedPixel) type
#[derive(Copy, Clone)]
pub struct IndexedPixel(pub types::IndexedPixel);

impl<'lua> FromLua<'lua> for IndexedPixel {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> Result<Self> {
        match value.as_userdata() {
            Some(ud) => Ok(*ud.borrow::<Self>()?),
            None => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "IndexedPixel",
                message: None,
            }),
        }
    }
}

impl TealData for IndexedPixel {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        use mlua::Error::{ ExternalError };

        methods.document_type("An indexed color type");

        //Lua interface to construct a new IndexedPixel with a 8-bit index
        {
            mlua_create_named_parameters!(
                IndexedPixelArgs with
                    index: u8,
            );
            methods.document("Create & return a new IndexedPixel with a specified index");
            methods.add_meta_method(MetaMethod::Call, |_, _, args: IndexedPixelArgs| {
                Ok(types::IndexedPixel(args.index))
            });
        }

        methods.generate_help();
    }

    /*
    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {

        //todo: add constants RED, GREEN, BLUE, etc.

        fields.document("the red level of the IndexedPixel");
        fields.add_field_method_get("red", |_, this| Ok(this.0.r));
        fields.add_field_method_set("red", |_, this, value| {
            this.0.r = value;
            Ok(())
        });

        fields.document("the green level of the IndexedPixel");
        fields.add_field_method_get("green", |_, this| Ok(this.0.g));
        fields.add_field_method_set("green", |_, this, value| {
            this.0.g = value;
            Ok(())
        });

        fields.document("the blue level of the IndexedPixel");
        fields.add_field_method_get("blue", |_, this| Ok(this.0.b));
        fields.add_field_method_set("blue", |_, this, value| {
            this.0.b = value;
            Ok(())
        });

        fields.document("the alpha level of the IndexedPixel");
        fields.add_field_method_get("alpha", |_, this| Ok(this.0.a));
        fields.add_field_method_set("alpha", |_, this, value| {
            this.0.a = value;
            Ok(())
        });
    }
    */
}

impl ToTypename for IndexedPixel {
    fn to_typename() -> tealr::Type {
        tealr::Type::new_single("IndexedPixel", tealr::KindOfType::External)
    }
}

impl UserData for IndexedPixel {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        let mut wrapper = UserDataWrapper::from_user_data_methods(methods);
        <Self as TealData>::add_methods(&mut wrapper)
    }
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        let mut wrapper = UserDataWrapper::from_user_data_fields(fields);
        <Self as TealData>::add_fields(&mut wrapper)
    }
}

impl TypeBody for IndexedPixel {
    fn get_type_body() -> tealr::TypeGenerator {
        let mut gen = tealr::RecordGenerator::new::<Self>(false);
        gen.is_user_data = true;
        <Self as TealData>::add_fields(&mut gen);
        <Self as TealData>::add_methods(&mut gen);
        gen.into()
    }
}
