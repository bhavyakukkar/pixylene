use crate::{
    types::{UCoord, PCoord, Pixel, TruePixel},
    project::{Scene},
};

use std::{fmt, fs::File, path::PathBuf, io::BufWriter};
use png::{Decoder, ColorType, BitDepth};
use resize::{Type::{Lanczos3, Mitchell}};


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

        let file = File::open(path)
            .map_err(|err| FileNotFoundError(path.clone(), err))?;
        let decoder = Decoder::new(file);
        let mut reader = decoder.read_info()
            .map_err(|err| DecodingError(path.clone(), err))?;
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)
            .map_err(|err| DecodingError(path.clone(), err))?;
        let bytes = buf[..info.buffer_size()].to_vec();

        Ok(Self {
            height: info.height,
            width: info.width,
            color_type: info.color_type,
            bit_depth: info.bit_depth,
            bytes
        })
    }

    pub fn write(&self, path: &PathBuf) -> Result<(), PngFileError> {
        use PngFileError::{EncodingError, DirectoryNotFoundError};

        let file = File::create(path)
            .map_err(|err| DirectoryNotFoundError(path.clone(), err))?;
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.width, self.height);
        encoder.set_color(self.color_type);
        encoder.set_depth(self.bit_depth);
        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455));

        let mut writer = encoder.write_header()
            .map_err(|err| EncodingError(path.clone(), err))?;
        writer.write_image_data(&self.bytes)
            .map(|_| ())
            .map_err(|err| EncodingError(path.clone(), err))
    }

    //over here
    pub fn from_scene(scene: &Scene<TruePixel>, color_type: ColorType, bit_depth: BitDepth)
        -> Result<Self, PngFileError>
    {
        use PngFileError::Unsupported;
        use ColorType::*;
        use BitDepth::*;

        let mut bytes;
        match (color_type, bit_depth) {
            (Rgb, Eight) => {
                bytes = vec![0; scene.dim().area() as usize*3];
                for x in 0..scene.dim().x() {
                    for y in 0..scene.dim().y() {
                        let TruePixel{ r, g, b, .. } = scene.get_pixel(UCoord{ x, y })
                            .unwrap() //cant fail because iterating over same scene's dim
                            .unwrap_or(TruePixel::empty());
                        let index = (x*scene.dim().y() + y) as usize;
                        bytes[index*3 + 0] = r;
                        bytes[index*3 + 1] = g;
                        bytes[index*3 + 2] = b;
                    }
                }
            },
            (Rgba, Eight) => {
                bytes = vec![0; scene.dim().area() as usize*4];
                for x in 0..scene.dim().x() {
                    for y in 0..scene.dim().y() {
                        let TruePixel{ r, g, b, a } = scene.get_pixel(UCoord{ x, y })
                            .unwrap() //cant fail because iterating over same scene's dim
                            .unwrap_or(TruePixel::empty());
                        let index = (x*scene.dim().y() + y) as usize;
                        bytes[index*4 + 0] = r;
                        bytes[index*4 + 1] = g;
                        bytes[index*4 + 2] = b;
                        bytes[index*4 + 3] = a;
                    }
                }
            }
            (Indexed, _) => { return Err(Unsupported(color_type, bit_depth)); },
            (_, _) => { return Err(Unsupported(color_type, bit_depth)); },
        }

        Ok(Self {
            height: scene.dim().x().into(),
            width: scene.dim().y().into(),
            color_type,
            bit_depth,
            bytes,
        })
    }

    //pub fn resize(&mut self, new: PCoord<u32>) {
    //    let Self {
    //        ref mut height,
    //        ref mut width,
    //        ref color_type,
    //        ref bit_depth,
    //        ref mut bytes,
    //    } = self;

    //    let mut resizer = Resizer::new(
    //        width as usize,
    //        height as usize,
    //        new.y() as usize,
    //        new.x() as usize,
    //        match (color_type, bit_depth) {
    //            (Rgb, Eight) => Pixel::RGB8,
    //            (Rgb, Sixteen) => Pixel::RGB16,
    //            (Rgba, Eight) => Pixel::RGBA8,
    //            (Rgba, Sixteen) => Pixel::RGBA16,
    //            _ => { return Err(Unsupported(color_type, bit_depth)); }
    //        },
    //        if height*width < new.area() { Mitchell } else { Lanczos3 },
    //    );

    //    resizer.resize(&bytes.clone(), &mut bytes);
    //    height = new.x();
    //    width = new.y();
    //}

    pub fn to_scene(&self) -> Result<Scene<TruePixel>, PngFileError> {
        use PngFileError::{Unsupported, SceneSizeError, ZeroDimError};
        use ColorType::*;
        use BitDepth::*;

        let dim = PCoord::new(
            u16::try_from(self.height).or(Err(SceneSizeError(self.height, self.width)))?,
            u16::try_from(self.width).or(Err(SceneSizeError(self.height, self.width)))?,
        )
        .or(Err(ZeroDimError(self.height, self.width)))?;

        let mut scene = Scene::<TruePixel>::new(dim, vec![None; dim.area() as usize])
            .unwrap();

        match (self.color_type, self.bit_depth) {
            (Rgb, Eight) => {
                for i in 0..scene.dim().x() {
                    for j in 0..scene.dim().y() {
                        scene.set_pixel(
                            UCoord{ x: i, y: j },
                            Some(TruePixel {
                                r: self.bytes[((3*i*scene.dim().y()) + (3*j) + 0) as usize],
                                g: self.bytes[((3*i*scene.dim().y()) + (3*j) + 1) as usize],
                                b: self.bytes[((3*i*scene.dim().y()) + (3*j) + 2) as usize],
                                a: 255,
                            })
                        ).unwrap();
                    }
                }
                Ok(scene)
            },
            //over here
            (Indexed, _) => Err(Unsupported(self.color_type, self.bit_depth)),
            (Rgba, Eight) => {
                for i in 0..scene.dim().x() {
                    for j in 0..scene.dim().y() {
                        scene.set_pixel(
                            UCoord{ x: i, y: j },
                            Some(TruePixel {
                                r: self.bytes[((4*i*scene.dim().y()) + (4*j) + 0) as usize],
                                g: self.bytes[((4*i*scene.dim().y()) + (4*j) + 1) as usize],
                                b: self.bytes[((4*i*scene.dim().y()) + (4*j) + 2) as usize],
                                a: self.bytes[((4*i*scene.dim().y()) + (4*j) + 3) as usize],
                            })
                        ).unwrap();
                    }
                }
                Ok(scene)
            },
            (_, _) => Err(Unsupported(self.color_type, self.bit_depth)),
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
}
impl fmt::Display for PngFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use PngFileError::*;
        match self {
            Unsupported(color_type, bit_depth) => write!(
                f,
                "{:?}-bit {:?} PNGs are current not supported",
                bit_depth,
                color_type,
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
            FileNotFoundError(path, io_error) => write!(
                f,
                "file '{}' was not found: {}",
                path.display(),
                io_error,
            ),
            DirectoryNotFoundError(path, io_error) => write!(
                f,
                "file '{}' could not be created (hint: the enclosing directory may not exist): {}",
                path.display(),
                io_error,
            ),
            SceneSizeError(height, width) => write!(
                f,
                "cannot convert the given png of dimensions {}x{}px as it is too large to be \
                contained inside a Scene, whose maximum possible dimensions are {}x{}",
                width,
                height,
                PCoord::<u16>::MAX,
                PCoord::<u16>::MAX,
            ),
            ZeroDimError(height, width) => write!(
                f,
                "found png of dimensions {}x{}px which cannot be converted to a Scene",
                width,
                height,
            ),
        }
    }
}
