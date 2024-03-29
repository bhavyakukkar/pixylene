use crate::{
    types::{ PCoord, UCoord, Pixel },
    project::{ Scene },
    utils::messages::U32TOUSIZE,
};

use std::fmt;
use std::fs::File;
use std::path::Path;
use std::io::BufWriter;
use png::{ Decoder, ColorType, BitDepth };
use ColorType::*;
use BitDepth::*;


const MAX_HEIGHT: u16 = u16::MAX;
const MAX_WIDTH: u16 = u16::MAX;

//can export to u32::MAX x u32::MAX PNGs but can only import from u16::MAX x u16::MAX PNGs
pub struct PngFile {
    height: u32,
    width: u32,
    color_type: ColorType,
    bit_depth: BitDepth,
    bytes: Vec<u8>
}

impl PngFile {
    pub fn read(path: String) -> Result<Self, PngFileError> {
        use PngFileError::{ DecodingError, SizeError, FileNotFoundError };
        match File::open(&path) {
            Ok(file) => {
                let decoder = Decoder::new(file);
                match decoder.read_info() {
                    Ok(mut reader) => {
                        let mut buf = vec![0; reader.output_buffer_size()];
                        match reader.next_frame(&mut buf) {
                            Ok(info) => {
                                let bytes = buf[..info.buffer_size()].to_vec();
                                return Ok(PngFile {
                                    height: u32::from(
                                        u16::try_from(info.height)
                                        .or(Err(SizeError(info.height, info.width)))?
                                    ),
                                    width: u32::from(
                                        u16::try_from(info.width)
                                        .or(Err(SizeError(info.height, info.width)))?
                                    ),
                                    color_type: info.color_type,
                                    bit_depth: info.bit_depth,
                                    bytes: bytes
                                });
                            },
                            Err(decoding_error) => {
                                return Err(DecodingError(path, decoding_error));
                            },
                        }
                    },
                    Err(decoding_error) => {
                        return Err(DecodingError(path, decoding_error));
                    },
                }
            },
            Err(io_error) => {
                return Err(FileNotFoundError(path, io_error));
            },
        }
    }
    pub fn write(&self, file_path: String) -> Result<(), PngFileError> {
        use PngFileError::{ EncodingError, DirectoryNotFoundError };
        let path = Path::new(&file_path);
        match File::create(path) {
            Ok(file) => {
                let ref mut w = BufWriter::new(file);
                let mut encoder = png::Encoder::new(w, self.width, self.height);
                encoder.set_color(self.color_type);
                encoder.set_depth(self.bit_depth);
                encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455));
                let x = match encoder.write_header() {
                    Ok(mut writer) => {
                        match writer.write_image_data(&self.bytes) {
                            Ok(_) => Ok(()),
                            Err(encoding_error) => {
                                return Err(EncodingError(file_path, encoding_error));
                            },
                        }
                    },
                    Err(encoding_error) => Err(EncodingError(file_path, encoding_error)),
                };
                x
            },
            Err(io_error) => Err(DirectoryNotFoundError(file_path, io_error)),
        }
    }
    pub fn to_scene(self) -> Result<Scene, PngFileError> {
        use PngFileError::Unsupported;
        let dim = PCoord::new(
            u16::try_from(self.height).unwrap(),
            u16::try_from(self.width).unwrap()
        ).unwrap();
        let mut scene: Scene = Scene::new(
            dim,
            vec![None; usize::try_from(dim.area()).expect(U32TOUSIZE)]
        ).unwrap();
        match self.color_type {
            Grayscale => {
                return Err(Unsupported(self.color_type, self.bit_depth));
            },
            Rgb => {
                return Err(Unsupported(self.color_type, self.bit_depth));
            },
            Indexed => {
                return Err(Unsupported(self.color_type, self.bit_depth));
            },
            GrayscaleAlpha => {
                return Err(Unsupported(self.color_type, self.bit_depth));
            },
            Rgba => {
                match self.bit_depth {
                    Eight => {
                        for i in 0..scene.dim().x() {
                            for j in 0..scene.dim().y() {
                                scene.set_pixel(
                                    UCoord{ x: i, y: j },
                                    Some(Pixel {
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
            }
        }
        Ok(scene)
    }
    pub fn from_scene(
        scene: &Scene,
        color_type: ColorType,
        bit_depth: BitDepth,
        scale_up: u16,
    ) -> Result<Self, PngFileError> {
        use PngFileError::Unsupported;
        let mut png = PngFile {
            height: u32::from(scene.dim().x()) * u32::from(scale_up),
            width: u32::from(scene.dim().y()) * u32::from(scale_up),
            color_type: color_type,
            bit_depth: bit_depth,
            bytes: Vec::new(),
        };
        match color_type {
            Grayscale => {
                return Err(Unsupported(color_type, bit_depth));
            },
            Rgb => {
                return Err(Unsupported(color_type, bit_depth));
            },
            Indexed => {
                return Err(Unsupported(color_type, bit_depth));
            },
            GrayscaleAlpha => {
                return Err(Unsupported(color_type, bit_depth));
            },
            Rgba => {
                match bit_depth {
                    Eight => {
                        for i in 0..scene.dim().x() {
                            for _ in 0..scale_up {
                                for j in 0..scene.dim().y() {
                                    let Pixel {
                                        r: red,
                                        g: green,
                                        b: blue,
                                        a: alpha
                                    } = scene.get_pixel(UCoord{ x: i, y: j })
                                            .unwrap()
                                            .unwrap_or(Pixel::empty());
                                    for _ in 0..scale_up {
                                        png.bytes.push(red);
                                        png.bytes.push(green);
                                        png.bytes.push(blue);
                                        png.bytes.push(alpha);
                                    }
                                }
                            }
                        }
                    },
                    _ => {
                        return Err(Unsupported(color_type, bit_depth));
                    }
                }
            }
        }
        Ok(png)
    }
}


// Error Types

#[derive(Debug)]
pub enum PngFileError {
    Unsupported(ColorType, BitDepth),
    DecodingError(String, png::DecodingError),
    EncodingError(String, png::EncodingError),
    FileNotFoundError(String, std::io::Error),
    DirectoryNotFoundError(String, std::io::Error),
    SizeError(u32, u32),
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
                path,
                decoding_error,
            ),
            EncodingError(path, encoding_error) => write!(
                f,
                "failed to encode png to file '{}': {}",
                path,
                encoding_error,
            ),
            FileNotFoundError(path, io_error) => write!(
                f,
                "file '{}' was not found: {}",
                path,
                io_error,
            ),
            DirectoryNotFoundError(path, io_error) => write!(
                f,
                "file '{}' could not be created (hint: the enclosing directory may not exist): {}",
                path,
                io_error,
            ),
            SizeError(height, width) => write!(
                f,
                "cannot encode the given file of dimensions {}x{}px as it is too large, largest \
                dimensions supported are {}x{}px",
                width,
                height,
                MAX_WIDTH,
                MAX_HEIGHT,
            ),
        }
    }
}
