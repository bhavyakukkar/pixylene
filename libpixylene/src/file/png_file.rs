use crate::{
    project::{Layer, Scene, LayersType, Palette, Canvas},
    types::{UCoord, PCoord, Pixel, IndexedPixel, TruePixel, BlendMode},
};

use png::{BitDepth, ColorType, Decoder};
use std::{fmt, fs::File, io::BufWriter, path::PathBuf, collections::HashMap};

pub struct PngFile {
    height: u32,
    width: u32,
    color_type: ColorType,
    bit_depth: BitDepth,
    bytes: Vec<u8>,
    palette: Option<Vec<u8>>,
}

impl PngFile {
    pub fn read(path: &PathBuf) -> Result<Self, PngFileError> {
        use PngFileError::{DecodingError, FileNotFoundError};

        let file = File::open(path).map_err(|err| FileNotFoundError(path.clone(), err))?;
        let decoder = Decoder::new(file);
        let mut reader = decoder
            .read_info()
            .map_err(|err| DecodingError(path.clone(), err))?;
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader
            .next_frame(&mut buf)
            .map_err(|err| DecodingError(path.clone(), err))?;
        let bytes = buf[..info.buffer_size()].to_vec();

        Ok(Self {
            height: info.height,
            width: info.width,
            color_type: info.color_type,
            bit_depth: info.bit_depth,
            bytes,
            palette: reader.info().palette.clone().map(|p| Vec::from(p)),
        })
    }

    pub fn write(&self, path: &PathBuf) -> Result<(), PngFileError> {
        use PngFileError::{DirectoryNotFoundError, EncodingError};

        let file = File::create(path).map_err(|err| DirectoryNotFoundError(path.clone(), err))?;
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(self.color_type);
        encoder.set_depth(self.bit_depth);
        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455));
        if let Some(palette) = &self.palette {
            encoder.set_palette(palette);
        }

        let mut writer = encoder
            .write_header()
            .map_err(|err| EncodingError(path.clone(), err))?;
        writer
            .write_image_data(&self.bytes)
            .map(|_| ())
            .map_err(|err| EncodingError(path.clone(), err))
    }

    pub fn from_canvas(canvas: &Canvas) -> Result<Self, PngFileError> {
        let mut bytes;
        let dim = canvas.layers.dim();
        match &canvas.layers {
            LayersType::True(_) => {
                bytes = vec![0; dim.area() as usize * 4];
                for x in 0..dim.x() {
                    for y in 0..dim.y() {
                        let TruePixel { r, g, b, a } = canvas.merged_true_scene(None)
                            .get_pixel(UCoord{ x, y })
                            .unwrap() //cant fail because iterating over same scene's dim
                            .unwrap_or(TruePixel::empty());
                        let index = (x * dim.y() + y) as usize;
                        bytes[index * 4 + 0] = r;
                        bytes[index * 4 + 1] = g;
                        bytes[index * 4 + 2] = b;
                        bytes[index * 4 + 3] = a;
                    }
                }

                Ok(Self {
                    height: dim.x().into(),
                    width: dim.y().into(),
                    color_type: ColorType::Rgba,
                    bit_depth: BitDepth::Eight,
                    bytes,
                    palette: None,
                })
            },
            LayersType::Indexed(_) => {
                bytes = vec![0; dim.area() as usize];
                for x in 0..dim.x() {
                    for y in 0..dim.y() {
                        //over here 1.1
                        //using placeholder layers[0]
                        //create merged_scene_indexed that just overwrites top layer on bottom
                        //layer where not None
                        //let IndexedPixel(p) = canvas.merged_scene_indexed(
                        let IndexedPixel(p) = canvas.merged_indexed_scene(None)
                            .unwrap() //cant fail because this is an indexed canvas
                            .get_pixel(UCoord{ x, y })
                            .unwrap() //cant fail because iterating over same scene's dim
                            .unwrap_or(IndexedPixel::empty());
                        bytes[x as usize * dim.y() as usize + y as usize] = p;
                    }
                }

                let palette_map = canvas.palette.colors()
                    .map(|(id, col, _)| (*id, *col))
                    .collect::<HashMap<u8, TruePixel>>();
                let palette_len = palette_map.iter()
                    .map(|(id, _)| *id)
                    .max()
                    .unwrap_or(0);

                let mut palette = Vec::new();
                for i in 0..(palette_len + 1) {
                    if let Some(TruePixel{ r, g, b, .. }) = palette_map.get(&i) {
                        palette.push(*r);
                        palette.push(*g);
                        palette.push(*b);
                    } else {
                        palette.extend_from_slice(&[0, 0, 0]);
                    }
                }

                Ok(Self {
                    height: dim.x().into(),
                    width: dim.y().into(),
                    color_type: ColorType::Indexed,
                    bit_depth: BitDepth::Eight,
                    bytes,
                    palette: Some(palette),
                })
            },
        }
    }

    pub fn to_canvas(&self) -> Result<Canvas, PngFileError> {
        use BitDepth::*;
        use ColorType::*;
        use PngFileError::{SceneSizeError, Unsupported};
        use itertools::Itertools;

        self.check_dimensions()?;
        let dim = PCoord::new(
            u16::try_from(self.height).or(Err(SceneSizeError(self.height, self.width)))?,
            u16::try_from(self.width).or(Err(SceneSizeError(self.height, self.width)))?,
        )
        .unwrap(); //wont fail because check_dimensions

        match (self.color_type, self.bit_depth) {
            (Rgb, Eight) => {
                let mut scene =
                    Scene::<TruePixel>::new(dim, vec![None; dim.area() as usize]).unwrap(); //wont fail because same dim used in both parameters
                for i in 0..scene.dim().x() as usize {
                    for j in 0..scene.dim().y() as usize {
                        scene
                            .set_pixel(
                                UCoord {
                                    x: i as u16,
                                    y: j as u16,
                                },
                                Some(TruePixel {
                                    r: self.bytes[(3 * i * scene.dim().y() as usize) + (3 * j) + 0],
                                    g: self.bytes[(3 * i * scene.dim().y() as usize) + (3 * j) + 1],
                                    b: self.bytes[(3 * i * scene.dim().y() as usize) + (3 * j) + 2],
                                    a: 255,
                                }),
                            )
                            .unwrap();
                    }
                }
                Ok(Canvas{
                    layers: LayersType::True(
                        vec![Layer::<TruePixel>{
                            scene,
                            opacity: 255,
                            mute: false,
                            blend_mode: BlendMode::Normal,
                        }]
                        .try_into()
                        .unwrap(),
                    ),
                    palette: Palette::new(),
                })
            }
            (Rgba, Eight) => {
                let mut scene =
                    Scene::<TruePixel>::new(dim, vec![None; dim.area() as usize]).unwrap(); //wont fail because same dim used in both parameters
                for i in 0..scene.dim().x() as usize {
                    for j in 0..scene.dim().y() as usize {
                        scene
                            .set_pixel(
                                UCoord {
                                    x: i as u16,
                                    y: j as u16,
                                },
                                Some(TruePixel {
                                    r: self.bytes[(4 * i * scene.dim().y() as usize) + (4 * j) + 0],
                                    g: self.bytes[(4 * i * scene.dim().y() as usize) + (4 * j) + 1],
                                    b: self.bytes[(4 * i * scene.dim().y() as usize) + (4 * j) + 2],
                                    a: self.bytes[(4 * i * scene.dim().y() as usize) + (4 * j) + 3],
                                }),
                            )
                            .unwrap();
                    }
                }
                Ok(Canvas{
                    layers: LayersType::True(
                        vec![Layer::<TruePixel>{
                            scene,
                            opacity: 255,
                            mute: false,
                            blend_mode: BlendMode::Normal,
                        }]
                        .try_into()
                        .unwrap(),
                    ),
                    palette: Palette::new(),
                })
            }
            (Indexed, Eight) => {
                let mut scene =
                    Scene::<IndexedPixel>::new(dim, vec![None; dim.area() as usize]).unwrap(); //wont fail because same dim used in both parameters
                for i in 0..scene.dim().x() as usize {
                    for j in 0..scene.dim().y() as usize {
                        scene
                            .set_pixel(
                                UCoord {
                                    x: i as u16,
                                    y: j as u16,
                                },
                                Some(IndexedPixel(self.bytes[(i * scene.dim().y() as usize) + j])),
                            )
                            .unwrap();
                    }
                }
                Ok(Canvas{
                    layers: LayersType::Indexed(
                        vec![Layer::<IndexedPixel>{
                            scene,
                            opacity: 255,
                            mute: false,
                            blend_mode: BlendMode::Normal,
                        }]
                        .try_into()
                        .unwrap(),
                    ),
                    palette: <Palette as From<&Vec<TruePixel>>>::from(
                        &self.palette.clone().unwrap().iter()
                            .chunks(3)
                            .into_iter()
                            .map(|mut p| TruePixel {
                                r: *p.next().unwrap_or(&0),
                                g: *p.next().unwrap_or(&0),
                                b: *p.next().unwrap_or(&0),
                                a: 255,
                            })
                            .collect::<Vec<TruePixel>>()
                    ),
                })
            }
            (_, _) => Err(Unsupported(self.color_type, self.bit_depth)),
        }
    }

    #[cfg(feature = "resize")]
    pub fn resize(&mut self, new_dim: PCoord<u32>) -> Result<(), PngFileError> {
        use PngFileError::{NonAbsoluteUpscale, TryingToDownscale};

        //good to take advantage of legacy fn even when 'resize' crate enabled because of known
        //issue in resize:
        //https://github.com/PistonDevelopers/resize/issues/30
        match self.resize_legacy(new_dim) {
            Ok(()) => Ok(()),
            Err(NonAbsoluteUpscale(..)) | Err(TryingToDownscale(..)) => self.resize_crate(new_dim),
            Err(err) => Err(err),
        }
    }

    #[cfg(not(feature = "resize"))]
    pub fn resize(&mut self, new_dim: PCoord<u32>) -> Result<(), PngFileError> {
        self.resize_legacy(new_dim)
    }

    fn resize_legacy(&mut self, new_dim: PCoord<u32>) -> Result<(), PngFileError> {
        use itertools::Itertools;
        use std::mem::replace;
        use BitDepth::*;
        use ColorType::*;
        use PngFileError::{NonAbsoluteUpscale, TryingToDownscale, Unsupported};

        self.check_dimensions()?;
        let new_height = new_dim.x();
        let new_width = new_dim.y();

        if new_width % self.width != 0 || new_height % self.height != 0 {
            return Err(NonAbsoluteUpscale(
                self.width,
                self.height,
                new_width,
                new_height,
            ));
        }
        if new_width / self.width == 0 || new_height / self.height == 0 {
            return Err(TryingToDownscale(
                self.width,
                self.height,
                new_width,
                new_height,
            ));
        }

        let chunk_size = match (self.color_type, self.bit_depth) {
            (Rgb, Eight) => 3 * 1,
            (Rgb, Sixteen) => 3 * 2,
            (Rgba, Eight) => 4 * 1,
            (Rgba, Sixteen) => 4 * 2,
            (ct, bd) => {
                return Err(Unsupported(ct, bd));
            }
        };

        let mut folded_bytes: Vec<Vec<u8>> = self
            .bytes
            .iter()
            .chunks(chunk_size)
            .into_iter()
            .map(|p| p.take(chunk_size).map(|p| *p).collect())
            .collect();

        match Self::enlarge_matrix(
            &mut folded_bytes,
            self.width,
            self.height,
            (new_width / self.width, new_height / self.height),
        ) {
            Err(1) => {
                panic!("Png dimensions cannot be multiplied in a usize on this architecture");
            }
            Err(3) => {
                panic!("Something went wrong with Png in-memory bytes management");
            }
            _ => {
                //Includes Ok(()) and Err(2)
                //Err(2) can't happen because we have check_dimensions'ed
                _ = replace(
                    &mut self.bytes,
                    folded_bytes.into_iter().flatten().collect(),
                );
                self.width = new_width;
                self.height = new_height;
                Ok(())
            }
        }
    }

    fn enlarge_matrix<T: Clone>(
        matrix: &mut Vec<T>,
        height: u32,
        width: u32,
        factor: (u32, u32),
    ) -> Result<(), u8> {
        use std::mem::replace;
        let mut enlarged: Vec<T> = Vec::new();

        _ = matrix
            .get(
                (height as usize)
                    .checked_mul(width as usize)
                    .ok_or(1)?
                    .checked_sub(1)
                    .ok_or(2)? as usize,
            )
            .ok_or(3)?;
        for i in 0..height {
            for _ in 0..factor.1 {
                for j in 0..width {
                    for _ in 0..factor.0 {
                        //unwrap cant fail because already checked last possible element before for
                        //loop
                        enlarged.push(matrix.get((i * width + j) as usize).unwrap().clone());
                    }
                }
            }
        }

        _ = replace(matrix, enlarged);
        Ok(())
    }

    #[cfg(feature = "resize")]
    fn resize_crate(&mut self, new_dim: PCoord<u32>) -> Result<(), PngFileError> {
        use itertools::Itertools;
        use resize::{
            px::{RGB, RGBA},
            Pixel::{RGB16, RGB8, RGBA16, RGBA8},
            Resizer, Type,
        };
        use std::mem::replace;
        use BitDepth::*;
        use ColorType::*;
        use PngFileError::{ResizeError, Unsupported};

        self.check_dimensions()?;
        let new_height = new_dim.x();
        let new_width = new_dim.y();

        match (self.color_type, self.bit_depth) {
            (Rgb, Eight) => {
                let folded_bytes = self
                    .bytes
                    .iter()
                    //take 3 u8s and make an RGB
                    .chunks(3)
                    .into_iter()
                    .map(|mut p| {
                        RGB::new(
                            p.next().unwrap().clone(),
                            p.next().unwrap().clone(),
                            p.next().unwrap().clone(),
                        )
                    })
                    .collect::<Vec<RGB<u8>>>();
                let mut resizer = Resizer::new(
                    self.width as usize,
                    self.height as usize,
                    new_width as usize,
                    new_height as usize,
                    RGB8,
                    Type::Point,
                )
                .unwrap();
                let mut out = vec![RGB::<u8>::new(0, 0, 0); (new_width * new_height) as usize];
                resizer
                    .resize(folded_bytes.as_slice(), &mut out)
                    .map_err(|err| ResizeError(err))?;
                _ = replace(
                    &mut self.bytes,
                    out.iter().map(|p| vec![p.r, p.g, p.b]).flatten().collect(),
                );
                self.width = new_width;
                self.height = new_height;
                Ok(())
            }
            (Rgb, Sixteen) => {
                let folded_bytes = self
                    .bytes
                    .iter()
                    //take u8 & u8 and combine to u16
                    .chunks(2)
                    .into_iter()
                    .map(|mut e| {
                        (e.next().unwrap().clone() as u16) * 256
                            + (e.next().unwrap().clone() as u16)
                    })
                    //take 3 u8s and make an RGB
                    .chunks(3)
                    .into_iter()
                    .map(|mut p| {
                        RGB::new(
                            p.next().unwrap().clone(),
                            p.next().unwrap().clone(),
                            p.next().unwrap().clone(),
                        )
                    })
                    .collect::<Vec<RGB<u16>>>();
                let mut resizer = Resizer::new(
                    self.width as usize,
                    self.height as usize,
                    new_width as usize,
                    new_height as usize,
                    RGB16,
                    Type::Point,
                )
                .unwrap();
                let mut out = vec![RGB::<u16>::new(0, 0, 0); (new_width * new_height) as usize];
                resizer
                    .resize(folded_bytes.as_slice(), &mut out)
                    .map_err(|err| ResizeError(err))?;
                _ = replace(
                    &mut self.bytes,
                    out.iter()
                        .map(|p| vec![p.r, p.g, p.b])
                        .flatten()
                        .map(|sixteen| vec![(sixteen) as u8, (sixteen >> 8) as u8])
                        .flatten()
                        .collect(),
                );
                self.width = new_width;
                self.height = new_height;
                Ok(())
            }
            (Rgba, Eight) => {
                let folded_bytes = self
                    .bytes
                    .iter()
                    //take 4 u8s and make an RGBA
                    .chunks(4)
                    .into_iter()
                    .map(|mut p| {
                        RGBA::new(
                            p.next().unwrap().clone(),
                            p.next().unwrap().clone(),
                            p.next().unwrap().clone(),
                            p.next().unwrap().clone(),
                        )
                    })
                    .collect::<Vec<RGBA<u8>>>();
                let mut resizer = Resizer::new(
                    self.width as usize,
                    self.height as usize,
                    new_width as usize,
                    new_height as usize,
                    RGBA8,
                    Type::Point,
                )
                .unwrap();
                let mut out = vec![RGBA::<u8>::new(0, 0, 0, 0); (new_width * new_height) as usize];
                resizer
                    .resize(folded_bytes.as_slice(), &mut out)
                    .map_err(|err| ResizeError(err))?;
                _ = replace(
                    &mut self.bytes,
                    out.iter()
                        .map(|p| vec![p.r, p.g, p.b, p.a])
                        .flatten()
                        .collect(),
                );
                self.width = new_width;
                self.height = new_height;
                Ok(())
            }
            (Rgba, Sixteen) => {
                let folded_bytes = self
                    .bytes
                    .iter()
                    //take u8 & u8 and combine to u16
                    .chunks(2)
                    .into_iter()
                    .map(|mut e| {
                        (e.next().unwrap().clone() as u16) * 256
                            + (e.next().unwrap().clone() as u16)
                    })
                    //take 4 u16s and make an RGBA
                    .chunks(4)
                    .into_iter()
                    .map(|mut p| {
                        RGBA::new(
                            p.next().unwrap().clone(),
                            p.next().unwrap().clone(),
                            p.next().unwrap().clone(),
                            p.next().unwrap().clone(),
                        )
                    })
                    .collect::<Vec<RGBA<u16>>>();
                let mut resizer = Resizer::new(
                    self.width as usize,
                    self.height as usize,
                    new_width as usize,
                    new_height as usize,
                    RGBA16,
                    Type::Point,
                )
                .unwrap(); //cant fail because
                let mut out = vec![RGBA::<u16>::new(0, 0, 0, 0); (new_width * new_height) as usize];
                resizer
                    .resize(folded_bytes.as_slice(), &mut out)
                    .map_err(|err| ResizeError(err))?;
                _ = replace(
                    &mut self.bytes,
                    out.iter()
                        .map(|p| vec![p.r, p.g, p.b, p.a])
                        .flatten()
                        .map(|sixteen| vec![(sixteen) as u8, (sixteen >> 8) as u8])
                        .flatten()
                        .collect(),
                );
                self.width = new_width;
                self.height = new_height;
                Ok(())
            }
            (ct, bd) => Err(Unsupported(ct, bd)),
        }
    }

    fn check_dimensions(&self) -> Result<(), PngFileError> {
        if self.width == 0 || self.height == 0 {
            Err(PngFileError::ZeroDimError(self.height, self.width))
        } else {
            Ok(())
        }
    }
}

// Error Types

#[derive(Debug)]
pub enum PngFileError {
    Unsupported(ColorType, BitDepth),
    DecodingError(PathBuf, png::DecodingError),
    EncodingError(PathBuf, png::EncodingError),
    FileNotFoundError(PathBuf, std::io::Error),
    DirectoryNotFoundError(PathBuf, std::io::Error),
    SceneSizeError(u32, u32),
    ZeroDimError(u32, u32),
    TryingToDownscale(u32, u32, u32, u32),
    NonAbsoluteUpscale(u32, u32, u32, u32),

    #[cfg(feature = "resize")]
    ResizeError(resize::Error),
}
impl fmt::Display for PngFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use PngFileError::*;
        match self {
            Unsupported(color_type, bit_depth) => write!(
                f,
                "{:?}-bit {:?} PNGs are current not supported",
                bit_depth, color_type,
            ),
            DecodingError(path, decoding_error) => write!(
                f,
                "failed to decode png from file '{}': {}",
                path.display(),
                decoding_error,
            ),
            EncodingError(path, encoding_error) => write!(
                f,
                "failed to encode png to file '{}': {}",
                path.display(),
                encoding_error,
            ),
            FileNotFoundError(path, io_error) => {
                write!(f, "file '{}' was not found: {}", path.display(), io_error)
            }
            DirectoryNotFoundError(path, io_error) => write!(
                f,
                "file '{}' could not be created (hint: the enclosing directory may not exist): {}",
                path.display(),
                io_error,
            ),
            SceneSizeError(height, width) => write!(
                f,
                "cannot convert the given png of dimensions {}x{}px as it is too large to be \
                contained inside a Scene, whose maximum possible dimensions are {}x{}. Try \
                resizing it",
                width,
                height,
                PCoord::<u16>::MAX,
                PCoord::<u16>::MAX,
            ),
            ZeroDimError(height, width) => write!(
                f,
                "found png of dimensions {}x{}px which cannot be converted to a Scene or resized",
                width, height,
            ),
            TryingToDownscale(ow, oh, nw, nh) => write!(
                f,
                "cannot resize ({},{}) to ({},{}) as it is a downscaling operation. enable crate \
                feature 'resize' to overcome this",
                ow, oh, nw, nh,
            ),
            NonAbsoluteUpscale(ow, oh, nw, nh) => write!(
                f,
                "cannot resize ({},{}) to ({},{}) as it is not an absolute upscaling. enable \
                crate feature 'resize' to overcome this",
                ow, oh, nw, nh,
            ),

            #[cfg(feature = "resize")]
            ResizeError(error) => write!(f, "{}", error),
        }
    }
}
