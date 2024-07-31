use libpixylene::types;
use tealr::{
    mlu::{
        mlua::{
            self, prelude::LuaValue, FromLua, Lua, MetaMethod, Result, UserData, UserDataFields,
            UserDataMethods,
        },
        TealData, TealDataMethods, UserDataWrapper,
    },
    mlua_create_named_parameters, ToTypename, TypeBody,
};

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
        methods.document_type("An indexed color type");

        //Lua interface to construct a new IndexedPixel with a 8-bit index
        {
            mlua_create_named_parameters!(
                IndexedPixelArgs with
                    index: u8,
            );
            methods.document("Create & return a new IndexedPixel with a specified index");
            methods.add_meta_method(MetaMethod::Call, |_, _, args: IndexedPixelArgs| {
                Ok(IndexedPixel(types::IndexedPixel(args.index)))
            });
        }

        methods.generate_help();
    }

    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {
        //Lua interface to IndexedPixel.0
        fields.document("the index of the IndexedPixel");
        fields.add_field_method_get("index", |_, this| Ok(this.0 .0));
        fields.add_field_method_set("index", |_, this, value| {
            this.0 .0 = value;
            Ok(())
        });
    }
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
