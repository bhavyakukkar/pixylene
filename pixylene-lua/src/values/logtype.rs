use tealr::{
    mlu::{
        mlua::{
            self, prelude::LuaValue, FromLua, Lua, Result, UserData, UserDataFields,
            UserDataMethods,
        },
        TealData, TealDataMethods, UserDataWrapper,
    },
    ToTypename, TypeBody,
};

/// Lua interface to pixylene-actions's [`LogType`](pixylene_actions::LogType) type
#[derive(Copy, Clone)]
pub struct LogType(pub pixylene_actions::LogType);

impl<'lua> FromLua<'lua> for LogType {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> Result<Self> {
        match value.as_userdata() {
            Some(ud) => Ok(*ud.borrow::<Self>()?),
            None => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "LogType",
                message: None,
            }),
        }
    }
}

impl TealData for LogType {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("Nature of the message outputted by an Action");
        methods.generate_help();
    }

    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {
        fields.document("Log of type 'Info'");
        fields.add_field_method_get("INFO", |_, _| Ok(LogType(pixylene_actions::LogType::Info)));

        fields.document("Log of type 'Error'");
        fields.add_field_method_get("ERROR", |_, _| {
            Ok(LogType(pixylene_actions::LogType::Error))
        });

        fields.document("Log of type 'Warning'");
        fields.add_field_method_get("WARNING", |_, _| {
            Ok(LogType(pixylene_actions::LogType::Warning))
        });

        fields.document("Log of type 'Success'");
        fields.add_field_method_get("SUCCESS", |_, _| {
            Ok(LogType(pixylene_actions::LogType::Success))
        });
    }
}

impl ToTypename for LogType {
    fn to_typename() -> tealr::Type {
        tealr::Type::new_single("LogType", tealr::KindOfType::External)
    }
}

impl UserData for LogType {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        let mut wrapper = UserDataWrapper::from_user_data_methods(methods);
        <Self as TealData>::add_methods(&mut wrapper)
    }
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        let mut wrapper = UserDataWrapper::from_user_data_fields(fields);
        <Self as TealData>::add_fields(&mut wrapper)
    }
}

impl TypeBody for LogType {
    fn get_type_body() -> tealr::TypeGenerator {
        let mut gen = tealr::RecordGenerator::new::<Self>(false);
        gen.is_user_data = true;
        <Self as TealData>::add_fields(&mut gen);
        <Self as TealData>::add_methods(&mut gen);
        gen.into()
    }
}
