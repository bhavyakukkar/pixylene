use crate::{
    Context,
    utils::LAYER_GONE,
    values::types::{ UCoord, PCoord, TruePixel }
};

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
use libpixylene::{ types, project };


/// Lua interface to libpixylene's [`Scene`][S] type
///
/// `Note`: While libpixylene's [`Scene`][S] is a grid of optional [`pixels`](types::Project), this
/// interface acts as a grid of just pixels.
///
/// [S]: project::Scene
#[derive(Clone)]
pub struct Scene(pub Context<project::Scene, u16>);

impl<'lua> FromLua<'lua> for Scene {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> Result<Scene> {
        match value.as_userdata() {
            Some(ud) => Ok((*ud.borrow::<Scene>()?).clone()),
            None => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Scene",
                message: None,
            }),
        }
    }
}

impl TealData for Scene {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("A two-dimensional grid of pixels.");

        //todo: make more flexible
        //Lua interface to Scene::new()
        {
            mlua_create_named_parameters!(
                SceneNewArgs with
                    dimensions: PCoord,
                    buffer: Vec<TruePixel>,
            );
            methods.document("Create a new scene with given dimensions and buffer of Pixels");
            methods.add_meta_method(MetaMethod::Call, |_, _, a: SceneNewArgs| {
                use mlua::Error::{ ExternalError };
                let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

                match project::Scene::new(a.dimensions.0, a.buffer.iter()
                    .map(|lua_pixel| Some(lua_pixel.0))
                    .collect()
                ) {
                    Ok(scene) => Ok(Scene(Context::Solo(scene))),
                    Err(err) => Err(ExternalError(Arc::from(
                        boxed_error(&err.to_string())
                    ))),
                }
            });
        }

        //Lua interface to Scene::get_pixel()
        {
            mlua_create_named_parameters!(
                SceneGetPixelArgs with
                    coordinate: UCoord,
            );
            methods.document("Get the pixel at a particular coordinate on the scene");
            methods.add_method("get", |_, this, a: SceneGetPixelArgs| {
                use types::Pixel;
                use mlua::Error::{ ExternalError };
                let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);
                match this.0.do_imt(
                    |scene| Ok(scene.get_pixel(a.coordinate.0)),
                    |pixylene, index| match pixylene.project.canvas() {
                        CanvasType::True(ref canvas) => canvas.layers().get_layer(*index)
                            .map(|layer| layer.scene.get_pixel(a.coordinate.0)),
                        CanvasType::Indexed(ref canvas) => canvas.layers().get_layer(*index)
                            .map(|layer| layer.scene.get_pixel(a.coordinate.0)),
                    }
                ).map_err(|_| ExternalError(Arc::from(boxed_error(LAYER_GONE))))?
                {
                    Ok(pixel) => Ok(TruePixel(pixel.unwrap_or(types::TruePixel::empty()))),
                    Err(err) => Err(ExternalError(Arc::from(
                        boxed_error(&err.to_string())
                    ))),
                }
            });
        }

        //Lua interface to Scene::set_pixel()
        {
            mlua_create_named_parameters!(
                SceneSetPixelArgs with
                    coordinate: UCoord,
                    new_pixel: TruePixel,
            );
            methods.document("Set the pixel at a particular coordinate on the scene");
            methods.add_method_mut("set", |_, this, a: SceneSetPixelArgs| {
                use mlua::Error::{ ExternalError };
                let boxed_error = |s: &str| Box::<dyn std::error::Error + Send + Sync>::from(s);

                match this.0.do_mut(
                    |scene| Ok(scene.set_pixel(a.coordinate.0, Some(a.new_pixel.0))),
                    |mut pixylene, index| pixylene.project.canvas
                        .get_layer_mut(*index)
                        .map(|layer| layer.scene.set_pixel(a.coordinate.0, Some(a.new_pixel.0)))
                ).map_err(|_| ExternalError(Arc::from(boxed_error(LAYER_GONE))))?
                {
                    Ok(()) => Ok(()),
                    Err(err) => Err(ExternalError(Arc::from(
                        boxed_error(&err.to_string())
                    ))),
                }
            });
        }

        methods.generate_help();
    }

    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {
        use mlua::Error::{ ExternalError };

        fields.document("the dimensions of this scene");
        fields.add_field_method_get("dim", |_, this| Ok(PCoord(this.0.do_imt(
            |scene| Ok(scene.dim()),
            |pixylene, index| pixylene.project.canvas
                .get_layer(*index)
                .map(|layer| layer.scene.dim())
        ).map_err(|_| ExternalError(Arc::from(
            Box::<dyn std::error::Error + Send + Sync>::from(LAYER_GONE)
        )))?)));
    }
}

impl ToTypename for Scene {
    fn to_typename() -> tealr::Type {
        tealr::Type::new_single("Scene", tealr::KindOfType::External)
    }
}

impl UserData for Scene {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        let mut wrapper = UserDataWrapper::from_user_data_methods(methods);
        <Self as TealData>::add_methods(&mut wrapper)
    }
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        let mut wrapper = UserDataWrapper::from_user_data_fields(fields);
        <Self as TealData>::add_fields(&mut wrapper)
    }
}

impl TypeBody for Scene {
    fn get_type_body() -> tealr::TypeGenerator {
        let mut gen = tealr::RecordGenerator::new::<Self>(false);
        gen.is_user_data = true;
        <Self as TealData>::add_fields(&mut gen);
        <Self as TealData>::add_methods(&mut gen);
        gen.into()
    }
}
