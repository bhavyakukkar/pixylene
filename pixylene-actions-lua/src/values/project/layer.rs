use crate::values::{ types::{ PCoord, BlendMode }, project::{ Scene } };

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


/// Lua interface to libpixylene's [`Layer`][project::Layer] type
#[derive(Clone)]
pub struct Layer(pub project::Layer);

impl<'lua> FromLua<'lua> for Layer {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> Result<Layer> {
        match value.as_userdata() {
            Some(ud) => Ok((*ud.borrow::<Layer>()?).clone()),
            None => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Layer",
                message: None,
            }),
        }
    }
}

impl TealData for Layer {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("A Scene with extra opacity, mute and blend-mode information.");

        //Lua interface to construct a new Layer
        {
            mlua_create_named_parameters!(
                LayerArgs with
                    scene: Scene,
                    opacity: u8,
                    mute: bool,
                    blend_mode: BlendMode,
            );
            methods.document("Create & return a new Layer by providing a Scene (which will exist \
                             as a clone in the Layer), an opacity (0-255), whether it is muted \
                             (boolean), and its BlendMode");
            methods.add_meta_method(MetaMethod::Call, |_, _, a: LayerArgs| {
                Ok(Layer(project::Layer {
                    scene: a.scene.0,
                    opacity: a.opacity,
                    mute: a.mute,
                    blend_mode: a.blend_mode.0,
                }))
            });
        }

        //Lua interface to Layer::merge
        {
            mlua_create_named_parameters!(
                LayerMergeArgs with
                    dimensions: PCoord,
                    top: Layer,
                    bottom: Layer,
                    blend_mode: BlendMode,
            );
            methods.document("Create & return a new Layer of given dimensions by merging the two \
                             existing layers with the given blend_mode");
            methods.add_function("merge", |_, a: LayerMergeArgs| {
                use mlua::Error::{ ExternalError };
                let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

                match project::Layer::merge(a.dimensions.0, &a.top.0, &a.bottom.0, a.blend_mode.0){
                    Ok(scene) => Ok(Scene(scene)),
                    Err(err) => Err(ExternalError(Arc::from(
                        boxed_error(&err.to_string())
                    ))),
                }
            });
        }

        // todo: + metamethod alias for merge that checks for consistent sizes and uses top layer's
        //       blend_mode

        methods.generate_help();
    }

    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {

        fields.document("the Scene being composed by this Layer");
        fields.add_field_method_get("scene", |_, this| Ok(Scene(this.0.scene.clone())));
        fields.add_field_method_set("scene", |_, this, value: Scene| {
            this.0.scene = value.0;
            Ok(())
        });

        fields.document("the opacity of this Layer");
        fields.add_field_method_get("opacity", |_, this| Ok(this.0.opacity));
        fields.add_field_method_set("opacity", |_, this, value| {
            this.0.opacity = value;
            Ok(())
        });

        fields.document("whether this Layer is muted");
        fields.add_field_method_get("mute", |_, this| Ok(this.0.mute));
        fields.add_field_method_set("mute", |_, this, value| {
            this.0.mute = value;
            Ok(())
        });

        fields.document("the blend_mode of this Layer");
        fields.add_field_method_get("blend_mode", |_, this| Ok(BlendMode(this.0.blend_mode)));
        fields.add_field_method_set("blend_mode", |_, this, value: BlendMode| {
            this.0.blend_mode = value.0;
            Ok(())
        });
    }
}

impl ToTypename for Layer {
    fn to_typename() -> tealr::Type {
        tealr::Type::new_single("Layer", tealr::KindOfType::External)
    }
}

impl UserData for Layer {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        let mut wrapper = UserDataWrapper::from_user_data_methods(methods);
        <Self as TealData>::add_methods(&mut wrapper)
    }
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        let mut wrapper = UserDataWrapper::from_user_data_fields(fields);
        <Self as TealData>::add_fields(&mut wrapper)
    }
}

impl TypeBody for Layer {
    fn get_type_body() -> tealr::TypeGenerator {
        let mut gen = tealr::RecordGenerator::new::<Self>(false);
        gen.is_user_data = true;
        <Self as TealData>::add_fields(&mut gen);
        <Self as TealData>::add_methods(&mut gen);
        gen.into()
    }
}
