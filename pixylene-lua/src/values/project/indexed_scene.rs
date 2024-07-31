use crate::{
    utils::{CanvasMismatch, ContextExpired, BOXED_ERROR, CANVAS_MISMATCH_INDEXED, LAYER_GONE},
    values::types::{IndexedPixel, PCoord, UCoord},
    Context,
};

use libpixylene::{project, types};
use std::sync::Arc;
use tealr::{
    mlu::{
        mlua::{
            self, prelude::LuaValue, Error::ExternalError, FromLua, Lua, MetaMethod, UserData,
            UserDataFields, UserDataMethods,
        },
        TealData, TealDataMethods, UserDataWrapper,
    },
    mlua_create_named_parameters, ToTypename, TypeBody,
};

/// Lua interface to libpixylene's [`Scene`][S] type over IndexedPixel
///
/// `Note`: While libpixylene's [`Scene`][S] is a grid of optional pixels, this
/// interface acts as a grid of just pixels.
///
/// [S]: project::Scene
#[derive(Clone)]
pub struct IndexedScene(pub Context<project::Scene<types::IndexedPixel>, u16>);

impl<'lua> FromLua<'lua> for IndexedScene {
    fn from_lua(value: LuaValue<'lua>, _: &'lua Lua) -> mlua::Result<IndexedScene> {
        match value.as_userdata() {
            Some(ud) => Ok((*ud.borrow::<IndexedScene>()?).clone()),
            None => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "IndexedScene",
                message: None,
            }),
        }
    }
}

impl TealData for IndexedScene {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("A two-dimensional grid of pixels.");

        //todo: make more flexible
        //Lua interface to Scene::new()
        {
            mlua_create_named_parameters!(
                IndexedSceneNewArgs with
                    dimensions: PCoord,
                    buffer: Vec<IndexedPixel>,
            );
            methods.document("Create a new scene with given dimensions and buffer of Pixels");
            methods.add_meta_method(MetaMethod::Call, |_, _, a: IndexedSceneNewArgs| {
                project::Scene::new(
                    a.dimensions.0,
                    a.buffer.iter().map(|lua_pixel| Some(lua_pixel.0)).collect(),
                )
                .map(|scene| IndexedScene(Context::Solo(scene)))
                .map_err(|err| ExternalError(Arc::from(BOXED_ERROR(&err.to_string()))))
            });
        }

        //Lua interface to Scene::get_pixel()
        {
            mlua_create_named_parameters!(
                IndexedSceneGetPixelArgs with
                    coordinate: UCoord,
            );
            methods.document("Get the pixel at a particular coordinate on the scene");
            methods.add_method("get", |_, this, a: IndexedSceneGetPixelArgs| {
                use types::Pixel;
                this.0.do_imt::<_, _, CanvasMismatch<
                    ContextExpired<Result<Option<types::IndexedPixel>, project::SceneError>>,
                >>(|scene| Ok(Ok(scene.get_pixel(a.coordinate.0))))(
                    |pixylene, index| {
                        pixylene.project.canvas.layers.to_indexed().map(|layers| {
                            layers
                                .get_layer(*index)
                                .map(|layer| layer.scene.get_pixel(a.coordinate.0))
                                .map_err(|_| ())
                        })
                    },
                )
                .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_INDEXED))))?
                .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(LAYER_GONE))))?
                .map_err(|err| ExternalError(Arc::from(BOXED_ERROR(&err.to_string()))))
                .map(|pixel| IndexedPixel(pixel.unwrap_or(types::IndexedPixel::empty())))
            });
        }

        //Lua interface to Scene::set_pixel()
        {
            mlua_create_named_parameters!(
                IndexedSceneSetPixelArgs with
                    coordinate: UCoord,
                    new_pixel: IndexedPixel,
            );
            methods.document("Set the pixel at a particular coordinate on the scene");
            methods.add_method_mut("set", |_, this, a: IndexedSceneSetPixelArgs| {
                this.0.do_mut::<_, _, CanvasMismatch<ContextExpired<
                    Result<(), project::SceneError>
                >>>
                    (|scene| Ok(Ok(scene.set_pixel(a.coordinate.0, Some(a.new_pixel.0)))))
                    (|mut pixylene, index| pixylene.project.canvas.layers.to_indexed_mut()
                        .map(|layers| layers.get_layer_mut(*index)
                            .map(|layer| layer.scene.set_pixel(a.coordinate.0, Some(a.new_pixel.0)))
                            .map_err(|_| ())))
                    .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_INDEXED))))?
                    .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(LAYER_GONE))))?
                    .map_err(|err| ExternalError(Arc::from(BOXED_ERROR(&err.to_string()))))
            });
        }

        methods.generate_help();
    }

    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {
        fields.document("the dimensions of this scene");
        fields.add_field_method_get("dim", |_, this| {
            Ok(PCoord(
                this.0
                    .do_imt::<_, _, CanvasMismatch<ContextExpired<types::PCoord>>>(|scene| {
                        Ok(Ok(scene.dim()))
                    })(|pixylene, index| {
                    pixylene.project.canvas.layers.to_indexed().map(|layers| {
                        layers
                            .get_layer(*index)
                            .map(|layer| layer.scene.dim())
                            .map_err(|_| ())
                    })
                })
                .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(CANVAS_MISMATCH_INDEXED))))?
                .map_err(|_| ExternalError(Arc::from(BOXED_ERROR(LAYER_GONE))))?,
            ))
        });
    }
}

impl ToTypename for IndexedScene {
    fn to_typename() -> tealr::Type {
        tealr::Type::new_single("IndexedScene", tealr::KindOfType::External)
    }
}

impl UserData for IndexedScene {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(methods: &mut T) {
        let mut wrapper = UserDataWrapper::from_user_data_methods(methods);
        <Self as TealData>::add_methods(&mut wrapper)
    }
    fn add_fields<'lua, F: UserDataFields<'lua, Self>>(fields: &mut F) {
        let mut wrapper = UserDataWrapper::from_user_data_fields(fields);
        <Self as TealData>::add_fields(&mut wrapper)
    }
}

impl TypeBody for IndexedScene {
    fn get_type_body() -> tealr::TypeGenerator {
        let mut gen = tealr::RecordGenerator::new::<Self>(false);
        gen.is_user_data = true;
        <Self as TealData>::add_fields(&mut gen);
        <Self as TealData>::add_methods(&mut gen);
        gen.into()
    }
}
