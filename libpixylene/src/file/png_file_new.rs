use crate::project::Canvas;

use std::path::PathBuf;
use png::{Decoder, ColorType, BitDepth};


pub struct PngFile {
    height: u32,
    width: u32,
    color_type: ColorType,
    bit_depth: BitDepth,
    bytes: Vec<u8>,
}

impl PngFile {
    pub fn read(path: &PathBuf) -> Result<Self, PngFileError> {
        use PngFileError::{DecodingError, SizeError, FileNotFoundError};

        let file = File::open(path)
            .map_err(|err| => FileNotFoundError(path.clone(), err))?;
        let decoder = Decoder::new(file);
        let reader = decoder.read_info()
            .map_err(|err| DecodingError(path.clone(), err))?;
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)
            .map_err(|err| DecodingError(path.clone(), decoding_error))?;
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

        let writer = encoder.write_header()
            .map_err(|err| EncodingError(path.clone(), err))?;
        writer.write_image_data(&self.bytes)
            .map(|_| ())
            .map_err(|err| EncodingError(path.clone(), err))
    }

    //over here
    pub fn from_scene(scene: &Scene, scale_up: u16, color_type: ColorType, bit_depth: BitDepth)
        -> Result<Self, PngFileError>
    {

    }

    pub fn to_scene(&self, scale_down: u16) -> Result<Scene, PngFileError> {
        use PngFileError::Unsupported;

        let dim = PCoord::new(self.height as u16, self.width as u16).unwrap();
        //over here
        match self.color_type {
            Grayscale => {
                return Err(Unsupported(self.color_type, self.bit_depth));
            },
            Rgb => {
                let mut scene = Scene::<TruePixel>::new(
                    dim,
                    vec![None; usize::try_from(dim.area()).expect(U32TOUSIZE)]
                ).unwrap();
                match self.bit_depth {
                    Eight => {
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
                    },
                    _ => {
                        return Err(Unsupported(self.color_type, self.bit_depth));
                    }
                }
                Ok(scene)
            },
            //over here
            Indexed => {
                return Err(Unsupported(self.color_type, self.bit_depth));
            },
            GrayscaleAlpha => {
                return Err(Unsupported(self.color_type, self.bit_depth));
            },
            Rgba => {
                let mut scene = Scene::<TruePixel>::new(
                    dim,
                    vec![None; usize::try_from(dim.area()).expect(U32TOUSIZE)]
                ).unwrap();
                match self.bit_depth {
                    Eight => {
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
                    },
                    _ => {
                        return Err(Unsupported(self.color_type, self.bit_depth));
                    }
                }
                Ok(scene)
            }
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
                PCoord::MAX,
                PCoord::MAX,
            ),
        }
    }
}
