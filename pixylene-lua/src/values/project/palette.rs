use crate::{utils::BOXED_ERROR, values::types::TruePixel, Context};

use libpixylene::project;
use std::sync::Arc;
use tealr::{
    mlu::{
        mlua::{
            self, prelude::LuaValue, Error::ExternalError, FromLua, Lua, MetaMethod, Result,
            UserData, UserDataFields, UserDataMethods,
        },
        TealData, TealDataMethods, UserDataWrapper,
    },
    mlua_create_named_parameters, ToTypename, TypeBody,
};

/// Lua interface to libpixylene's [`Palette`][project::Palette] type
#[derive(Clone)]
pub struct Palette(pub Context<project::Palette, ()>);

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
        methods.document_type(
            "A color palette of indexed colors where the significant color at \
                              any time can be chosen and equipped.",
        );

        //Lua interface to Palette::from()
        {
            methods.document(
                "Creates & returns a new Palette from a table of pairs of color \
                             indexes and hex-strings, \
                             e.g.: Palette{{1, \"#ffffff\"}, {2, \"#000000\"}}",
            );
            methods.add_meta_method(MetaMethod::Call, |_, _, a: mlua::Table| {
                let mut palette = project::Palette::new();

                for color_pair in a.pairs::<u8, String>() {
                    let (index, color) = color_pair?;
                    match palette.set_color(index, &color) {
                        Ok(()) => (),
                        Err(err) => {
                            return Err(ExternalError(Arc::from(BOXED_ERROR(&err.to_string()))));
                        }
                    }
                }

                Ok(Palette(Context::Solo(palette)))
            });
        }

        //Flexible Lua interface to get_equipped() and get_color()
        {
            methods.document(
                "Gets the equipped color of the Palette, or the color at the optional
                             index",
            );
            mlua_create_named_parameters!(
                PaletteGetArgs with
                    index: Option<u8>,
            );
            methods.add_method("get", |_, this, a: PaletteGetArgs| {
                Ok(TruePixel(match a.index {
                    Some(index) => this
                        .0
                        .do_imt(|palette| palette.get_color(index).map(|p| p.clone()))(
                        |pixylene, _| {
                            pixylene
                                .project
                                .canvas
                                .palette
                                .get_color(index)
                                .map(|p| p.clone())
                        },
                    )
                    .map_err(|err| ExternalError(Arc::from(BOXED_ERROR(&err.to_string())))),
                    None => Ok(this.0.do_imt(|palette| palette.get_equipped().clone())(
                        |pixylene, _| pixylene.project.canvas.palette.get_equipped().clone(),
                    )),
                }?))
            });
        }

        //Lua interface to Palette::equip() - replaced by setter to field 'equipped'
        //{
        //    mlua_create_named_parameters!(
        //        PaletteEquipArgs with
        //            index: u8,
        //    );
        //    methods.document("Equip the color at a particular index of the Palette");
        //    methods.add_method_mut("equip", |_, this, a: PaletteEquipArgs| this.0.do_mut
        //        (|palette| palette.equip(a.index))
        //        (|mut pixylene, _| pixylene.project.canvas.palette.equip(a.index))
        //        .map_err(|err| ExternalError(Arc::from(BOXED_ERROR(&err.to_string()))))
        //    );
        //}

        //Lua interface to Palette::set_color()
        {
            mlua_create_named_parameters!(
                PaletteSetArgs with
                    index: u8,
                    color: String,
            );
            methods.document("Set the color at a particular index of the Palette");
            methods.add_method_mut("set", |_, this, a: PaletteSetArgs| {
                this.0
                    .do_mut(|palette| palette.set_color(a.index, &a.color))(
                    |mut pixylene, _| pixylene.project.canvas.palette.set_color(a.index, &a.color),
                )
                .map_err(|err| ExternalError(Arc::from(BOXED_ERROR(&err.to_string()))))
            });
        }

        //Lua interface to Palette::unset_color()
        {
            mlua_create_named_parameters!(
                PaletteUnsetColorArgs with
                    index: u8,
            );
            methods.document("Unset the color at a particular index of the Palette");
            methods.add_method_mut("unset", |_, this, a: PaletteUnsetColorArgs| {
                Ok(this.0.do_mut(|palette| palette.unset_color(a.index))(
                    |mut pixylene, _| pixylene.project.canvas.palette.unset_color(a.index),
                ))
            });
        }

        methods.generate_help();
    }

    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {
        //Lua interface to Canvas field palette
        fields.document("the equipped index of the Palette");
        fields.add_field_method_get("equipped", |_, this| {
            Ok(this.0.do_imt(|palette| palette.equipped())(
                |pixylene, _| pixylene.project.canvas.palette.equipped(),
            ))
        });
        fields.add_field_method_set("equipped", |_, this, index: u8| {
            this.0.do_mut(|palette| palette.equip(index))(|mut pixylene, _| {
                pixylene.project.canvas.palette.equip(index)
            })
            .map_err(|err| ExternalError(Arc::from(BOXED_ERROR(&err.to_string()))))
        });
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
