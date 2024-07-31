use super::LogType;

use std::rc::Rc;
use tealr::{
    mlu::{
        mlua::{UserData, UserDataFields, UserDataMethods},
        TealData, TealDataMethods, UserDataWrapper,
    },
    mlua_create_named_parameters, ToTypename, TypeBody,
};

/// Lua interface to pixylene-actions's [`Console`][pixylene_actions::Console] type
pub struct Console(pub Rc<dyn pixylene_actions::Console>);

// No FromLua impl because Console never needs to be constructed from lua

impl TealData for Console {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type(
            "A portable type to enable basic I/O from an Action to a defined \
                              Pixylene User Interface.",
        );

        //Lua interface to call Console.cmdin
        {
            mlua_create_named_parameters!(
                ConsoleCmdinArgs with
                    message: Option<String>,
            );
            methods.document(
                "Asks the user to reply to a passed message and returns the user's \
                             input, nil if the user didn't reply.",
            );
            methods.add_method("cmdin", |_, this, a: ConsoleCmdinArgs| {
                match (*this.0).cmdin(&a.message.unwrap_or("".to_owned())) {
                    Some(reply) => Ok(Some(reply)),
                    None => Ok(None),
                }
            });
        }

        //Lua interface to call Console.cmdout
        {
            mlua_create_named_parameters!(
                ConsoleCmdoutArgs with
                    message: Option<String>,
                    log_type: Option<LogType>,
            );
            methods.document("Sends a message to the user, with an optional LogType.");
            methods.add_method("cmdout", |_, this, a: ConsoleCmdoutArgs| {
                Ok((*this.0).cmdout(
                    &a.message.unwrap_or("".to_owned()),
                    &a.log_type
                        .map(|l| l.0)
                        .unwrap_or(pixylene_actions::LogType::Info),
                ))
            });
        }

        methods.generate_help();
    }
}

impl ToTypename for Console {
    fn to_typename() -> tealr::Type {
        tealr::Type::new_single("Console", tealr::KindOfType::External)
    }
}

impl UserData for Console {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        let mut wrapper = UserDataWrapper::from_user_data_methods(methods);
        <Self as TealData>::add_methods(&mut wrapper)
    }
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        let mut wrapper = UserDataWrapper::from_user_data_fields(fields);
        <Self as TealData>::add_fields(&mut wrapper)
    }
}

impl TypeBody for Console {
    fn get_type_body() -> tealr::TypeGenerator {
        let mut gen = tealr::RecordGenerator::new::<Self>(false);
        gen.is_user_data = true;
        <Self as TealData>::add_fields(&mut gen);
        <Self as TealData>::add_methods(&mut gen);
        gen.into()
    }
}
