// havent started this
use tealr::{
    mlu::{
        self,
        mlua::{
            self,
            prelude::{ LuaValue, LuaUserData },
            FromLua, Value, Lua, Result, UserData, UserDataFields, UserDataMethods, MetaMethod,
        },
        TealData, TealDataMethods, UserDataWrapper,
    },
    ToTypename, TypeBody, TypeWalker, mlua_create_named_parameters,
};

use libpixylene::project;


/// Lua interface to libpixylene's [`Project`][project::Project] type
#[derive(Copy, Clone)]
pub struct Project(pub project::Project);

impl<'lua> FromLua<'lua> for Project {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> Result<Self> {
        match value.as_userdata() {
            Some(ud) => Ok(*ud.borrow::<Self>()?),
            None => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Project",
                message: None,
            }),
        }
    }
}

impl TealData for Project {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("An unsigned integer coordinate type composed of two 16-bit unsigned \
                             integers.");

        //Flexible Lua metamethod Call interface to construct a new Project
        //
        // u = Project(1, 2)   -- ->(1,2). or,
        // u = Project(1)      -- ->(1,0). or,
        // u = Project(nil, 1) -- ->(0,1). or,
        // u = Project()       -- ->(0,0)
        {
            mlua_create_named_parameters!(
                ProjectArgs with
                    x : Option<u16>,
                    y : Option<u16>,
            );
            methods.document("Create & return a new Project with optional 'x' and 'y' coordinates \
                             that default to 0");
            methods.add_meta_method(MetaMethod::Call, |_, _, a: ProjectArgs| {
                Ok(Project(project::Project{
                    x: a.x.unwrap_or(0),
                    y: a.y.unwrap_or(0),
                }))
            });
        }

        //Lua interface to construct a new Project
        {
            mlua_create_named_parameters!(
                ProjectNewArgs with
                    x : u16,
                    y : u16,
            );
            methods.document("Create & return a new Project with 'x' and 'y' coordinates");
            methods.add_function("new", |_, a: ProjectNewArgs| {
                Ok(Project(project::Project{x: a.x, y: a.y}))
            });
        }

        //Lua interface to Project::zero
        {
            methods.document("Create & return a new (0,0) Project");
            methods.add_function("zero", |_, _: ()| {
                Ok(Project(project::Project::zero()))
            });
        }

        //Lua interface to Project::area
        {
            methods.document("Return the 'area' of a Project, i.e., product of x and y");
            methods.add_method("area", |_, this, _: ()| -> Result<u32> {
                Ok(this.0.area())
            });
        }

        //Lua metamethod '+' interface to Project::add
        {
            mlua_create_named_parameters!(
                ProjectAddArgs with
                    first : Project,
                    second : Project,
            );
            methods.document("Return a Project composed of the overflowing sums of two Project's \
                             coordinates");
            methods.add_meta_function(MetaMethod::Add, |_, a: ProjectAddArgs| {
                Ok(Project(project::Project{
                    x: a.first.0.x.overflowing_add(a.second.0.x).0,
                    y: a.first.0.y.overflowing_add(a.second.0.y).0,
                }))
            });
        }

        methods.generate_help();
    }
    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {

        fields.document("the 'x' coordinate of the Project");
        fields.add_field_method_get("x", |_, this| Ok(this.0.x));
        fields.add_field_method_set("x", |_, this, value| {
            this.0.x = value;
            Ok(())
        });

        fields.document("the 'y' coordinate of the Project");
        fields.add_field_method_get("y", |_, this| Ok(this.0.y));
        fields.add_field_method_set("y", |_, this, value| {
            this.0.y = value;
            Ok(())
        });
    }
}

impl ToTypename for Project {
    fn to_typename() -> tealr::Type {
        tealr::Type::new_single("Project", tealr::KindOfType::External)
    }
}

impl UserData for Project {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        let mut wrapper = UserDataWrapper::from_user_data_methods(methods);
        <Self as TealData>::add_methods(&mut wrapper)
    }
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        let mut wrapper = UserDataWrapper::from_user_data_fields(fields);
        <Self as TealData>::add_fields(&mut wrapper)
    }
}

impl TypeBody for Project {
    fn get_type_body() -> tealr::TypeGenerator {
        let mut gen = tealr::RecordGenerator::new::<Self>(false);
        gen.is_user_data = true;
        <Self as TealData>::add_fields(&mut gen);
        <Self as TealData>::add_methods(&mut gen);
        gen.into()
    }
}
