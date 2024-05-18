use crate::{
    project::Scene,
    types::{PCoord, Pixel, TruePixel, UCoord},
};

use png::{BitDepth, ColorType, Decoder};
use std::{fmt, fs::File, io::BufWriter, path::PathBuf};

pub struct PngFile {
    height: u32,
    width: u32,
    color_type: ColorType,
    bit_depth: BitDepth,
    bytes: Vec<u8>,
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

        let mut writer = encoder
            .write_header()
            .map_err(|err| EncodingError(path.clone(), err))?;
        writer
            .write_image_data(&self.bytes)
            .map(|_| ())
            .map_err(|err| EncodingError(path.clone(), err))
    }

    pub fn from_scene(
        scene: &Scene<TruePixel>,
        color_type: ColorType,
        bit_depth: BitDepth,
    ) -> Result<Self, PngFileError> {
        use BitDepth::*;
        use ColorType::*;
        use PngFileError::Unsupported;

        let mut bytes;
        match (color_type, bit_depth) {
            (Rgb, Eight) => {
                bytes = vec![0; scene.dim().area() as usize * 3];
                for x in 0..scene.dim().x() {
                    for y in 0..scene.dim().y() {
                        let TruePixel { r, g, b, .. } = scene
                            .get_pixel(UCoord { x, y })
                            .unwrap() //cant fail because iterating over same scene's dim
                            .unwrap_or(TruePixel::empty());
                        let index = (x * scene.dim().y() + y) as usize;
                        bytes[index * 3 + 0] = r;
                        bytes[index * 3 + 1] = g;
                        bytes[index * 3 + 2] = b;
                    }
                }
            }
            (Rgba, Eight) => {
                bytes = vec![0; scene.dim().area() as usize * 4];
                for x in 0..scene.dim().x() {
                    for y in 0..scene.dim().y() {
                        let TruePixel { r, g, b, a } = scene
                            .get_pixel(UCoord { x, y })
                            .unwrap() //cant fail because iterating over same scene's dim
                            .unwrap_or(TruePixel::empty());
                        let index = (x * scene.dim().y() + y) as usize;
                        bytes[index * 4 + 0] = r;
                        bytes[index * 4 + 1] = g;
                        bytes[index * 4 + 2] = b;
                        bytes[index * 4 + 3] = a;
                    }
                }
            }
            (Indexed, _) => {
                return Err(Unsupported(color_type, bit_depth));
            }
            (_, _) => {
                return Err(Unsupported(color_type, bit_depth));
            }
        }

        Ok(Self {
            height: scene.dim().x().into(),
            width: scene.dim().y().into(),
            color_type,
            bit_depth,
            bytes,
        })
    }

    pub fn to_scene(&self) -> Result<Scene<TruePixel>, PngFileError> {
        use BitDepth::*;
        use ColorType::*;
        use PngFileError::{SceneSizeError, Unsupported};

        self.check_dimensions()?;
        let dim = PCoord::new(
            u16::try_from(self.height).or(Err(SceneSizeError(self.height, self.width)))?,
            u16::try_from(self.width).or(Err(SceneSizeError(self.height, self.width)))?,
        )
        .unwrap(); //wont fail because check_dimensions

        let mut scene = Scene::<TruePixel>::new(dim, vec![None; dim.area() as usize]).unwrap();

        match (self.color_type, self.bit_depth) {
            (Rgb, Eight) => {
                for i in 0..scene.dim().x() {
                    for j in 0..scene.dim().y() {
                        scene
                            .set_pixel(
                                UCoord { x: i, y: j },
                                Some(TruePixel {
                                    r: self.bytes
                                        [((3 * i * scene.dim().y()) + (3 * j) + 0) as usize],
                                    g: self.bytes
                                        [((3 * i * scene.dim().y()) + (3 * j) + 1) as usize],
                                    b: self.bytes
                                        [((3 * i * scene.dim().y()) + (3 * j) + 2) as usize],
                                    a: 255,
                                }),
                            )
                            .unwrap();
                    }
                }
                Ok(scene)
            }
            //over here
            (Indexed, _) => Err(Unsupported(self.color_type, self.bit_depth)),
            (Rgba, Eight) => {
                for i in 0..scene.dim().x() {
                    for j in 0..scene.dim().y() {
                        scene
                            .set_pixel(
                                UCoord { x: i, y: j },
                                Some(TruePixel {
                                    r: self.bytes
                                        [((4 * i * scene.dim().y()) + (4 * j) + 0) as usize],
                                    g: self.bytes
                                        [((4 * i * scene.dim().y()) + (4 * j) + 1) as usize],
                                    b: self.bytes
                                        [((4 * i * scene.dim().y()) + (4 * j) + 2) as usize],
                                    a: self.bytes
                                        [((4 * i * scene.dim().y()) + (4 * j) + 3) as usize],
                                }),
                            )
                            .unwrap();
                    }
                }
                Ok(scene)
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
            },
            Err(3) => {
                panic!("Something went wrong with Png in-memory bytes management");
            },
            _ => { //Includes Ok(()) and Err(2)
                   //Err(2) can't happen because we have check_dimensions'ed
                _ = replace(
                    &mut self.bytes,
                    folded_bytes.into_iter().flatten().collect(),
                );
                self.width = new_width;
                self.height = new_height;
                Ok(())
            },
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

        _ = matrix.get(
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
                    out.iter()
                        .map(|p| vec![p.r, p.g, p.b])
                        .flatten()
                        .collect(),
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
