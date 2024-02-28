use tealr::{
    mlu::{
        self,
        mlua::{
            prelude::{ LuaValue, LuaUserData },
            FromLua, Value, Lua, Result, UserData, UserDataMethods, MetaMethod,
        },
        TealData, TealDataMethods, UserDataWrapper,
    },
    ToTypename, TypeBody, TypeWalker, mlua_create_named_parameters,
};

use libpixylene::types;


/// Lua interface to [`types::Coord`]
#[derive(Copy, Clone)]
pub struct Coord(pub types::Coord);

impl<'lua> FromLua<'lua> for Coord {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> Result<Self> {
        match value.as_userdata() {
            Some(ud) => Ok(*ud.borrow::<Self>()?),
            None => Err(mlu::mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Coord",
                message: None,
            }),
        }
    }
}

impl TealData for Coord {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("An integer coordinate type composed of two 32-bit integers.");

        //Lua interface to [`types::Coord::new`]
        {
            mlua_create_named_parameters!(
                CoordNewArgs with
                    x : i32,
                    y : i32,
            );
            methods.document("Create & return a new Coord with 'x' and 'y' coordinates");
            methods.add_function("new", |_, a: CoordNewArgs| {
                Ok(Coord(types::Coord{x: a.x, y: a.y}))
            });
        }

        //Lua interface to [`types::Coord::zero`]
        {
            methods.document("Create & return a new (0,0) Coord");
            methods.add_function("zero", |_, _: ()| {
                Ok(Coord(types::Coord::zero()))
            });
        }

        //Lua interface to [`types::Coord::area`]
        {
            methods.document("Return the 'area' of a Coord, i.e., product of x and y");
            methods.add_method("area", |_, this, _: ()| -> Result<i64> {
                Ok(this.0.area())
            });
        }

        //Lua metamethod '+' interface to [`types::Coord::add`]
        {
            mlua_create_named_parameters!(
                CoordAddArgs with
                    first : Coord,
                    second : Coord,
            );
            methods.document("Return a Coord composed of the overflowing sums of two Coord's \
                             coordinates");
            methods.add_meta_function(MetaMethod::Add, |_, a: CoordAddArgs| {
                Ok(Coord(types::Coord{
                    x: a.first.0.x.overflowing_add(a.second.0.x).0,
                    y: a.first.0.y.overflowing_add(a.second.0.y).0,
                }))
            });
        }

        methods.generate_help();
    }
    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_, this| Ok(this.0.x));
        fields.add_field_method_set("x", |_, this, value| {
            this.0.x = value;
            Ok(())
        });
        fields.add_field_method_get("y", |_, this| Ok(this.0.y));
        fields.add_field_method_set("y", |_, this, value| {
            this.0.y = value;
            Ok(())
        });
    }
}

impl ToTypename for Coord {
    //how the type should be called in lua.
    fn to_typename() -> tealr::Type {
        tealr::Type::new_single("Coord", tealr::KindOfType::External)
    }
}

impl UserData for Coord {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        let mut wrapper = UserDataWrapper::from_user_data_methods(methods);
        <Self as TealData>::add_methods(&mut wrapper)
    }
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        let mut wrapper = UserDataWrapper::from_user_data_fields(fields);
        <Self as TealData>::add_fields(&mut wrapper)
    }
}

impl TypeBody for Coord {
    fn get_type_body() -> tealr::TypeGenerator {
        let mut gen = tealr::RecordGenerator::new::<Self>(false);
        gen.is_user_data = true;
        <Self as TealData>::add_fields(&mut gen);
        <Self as TealData>::add_methods(&mut gen);
        gen.into()
    }
}
