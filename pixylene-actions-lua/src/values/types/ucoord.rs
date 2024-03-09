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

use libpixylene::types;


/// Lua interface to libpixylene's [`UCoord`](types::UCoord) type
#[derive(Copy, Clone)]
pub struct UCoord(pub types::UCoord);

impl<'lua> FromLua<'lua> for UCoord {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> Result<Self> {
        match value.as_userdata() {
            Some(ud) => Ok(*ud.borrow::<Self>()?),
            None => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "UCoord",
                message: None,
            }),
        }
    }
}

impl TealData for UCoord {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("An unsigned integer coordinate type composed of two 16-bit unsigned \
                             integers.");

        //Flexible Lua metamethod Call interface to construct a new UCoord
        //
        // u = UCoord(1, 2)   -- ->(1,2). or,
        // u = UCoord(1)      -- ->(1,0). or,
        // u = UCoord(nil, 1) -- ->(0,1). or,
        // u = UCoord()       -- ->(0,0)
        {
            mlua_create_named_parameters!(
                UCoordArgs with
                    x : Option<u16>,
                    y : Option<u16>,
            );
            methods.document("Create & return a new UCoord with optional 'x' and 'y' coordinates \
                             that default to 0");
            methods.add_meta_method(MetaMethod::Call, |_, _, a: UCoordArgs| {
                Ok(UCoord(types::UCoord{
                    x: a.x.unwrap_or(0),
                    y: a.y.unwrap_or(0),
                }))
            });
        }

        //Lua interface to construct a new UCoord
        {
            mlua_create_named_parameters!(
                UCoordNewArgs with
                    x : u16,
                    y : u16,
            );
            methods.document("Create & return a new UCoord with 'x' and 'y' coordinates");
            methods.add_function("new", |_, a: UCoordNewArgs| {
                Ok(UCoord(types::UCoord{x: a.x, y: a.y}))
            });
        }

        //Lua interface to UCoord::zero
        {
            methods.document("Create & return a new (0,0) UCoord");
            methods.add_function("zero", |_, _: ()| {
                Ok(UCoord(types::UCoord::zero()))
            });
        }

        //Lua interface to UCoord::area
        {
            methods.document("Return the 'area' of a UCoord, i.e., product of x and y");
            methods.add_method("area", |_, this, _: ()| -> Result<u32> {
                Ok(this.0.area())
            });
        }

        //Lua metamethod '+' interface to UCoord::add
        {
            mlua_create_named_parameters!(
                UCoordAddArgs with
                    first : UCoord,
                    second : UCoord,
            );
            methods.document("Return a UCoord composed of the overflowing sums of two UCoord's \
                             coordinates");
            methods.add_meta_function(MetaMethod::Add, |_, a: UCoordAddArgs| {
                Ok(UCoord(types::UCoord{
                    x: a.first.0.x.overflowing_add(a.second.0.x).0,
                    y: a.first.0.y.overflowing_add(a.second.0.y).0,
                }))
            });
        }

        methods.generate_help();
    }
    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {

        fields.document("the 'x' coordinate of the UCoord");
        fields.add_field_method_get("x", |_, this| Ok(this.0.x));
        fields.add_field_method_set("x", |_, this, value| {
            this.0.x = value;
            Ok(())
        });

        fields.document("the 'y' coordinate of the UCoord");
        fields.add_field_method_get("y", |_, this| Ok(this.0.y));
        fields.add_field_method_set("y", |_, this, value| {
            this.0.y = value;
            Ok(())
        });
    }
}

impl ToTypename for UCoord {
    fn to_typename() -> tealr::Type {
        tealr::Type::new_single("UCoord", tealr::KindOfType::External)
    }
}

impl UserData for UCoord {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        let mut wrapper = UserDataWrapper::from_user_data_methods(methods);
        <Self as TealData>::add_methods(&mut wrapper)
    }
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        let mut wrapper = UserDataWrapper::from_user_data_fields(fields);
        <Self as TealData>::add_fields(&mut wrapper)
    }
}

impl TypeBody for UCoord {
    fn get_type_body() -> tealr::TypeGenerator {
        let mut gen = tealr::RecordGenerator::new::<Self>(false);
        gen.is_user_data = true;
        <Self as TealData>::add_fields(&mut gen);
        <Self as TealData>::add_methods(&mut gen);
        gen.into()
    }
}
