use std::fs::File;
use std::path::Path;
use std::io::BufWriter;
use png::{ Decoder, ColorType, BitDepth };
use ColorType::*;
use BitDepth::*;

use crate::grammar::Decorate;
use crate::elements::common::{ Coord, Pixel };
use crate::elements::layer::{ Scene };

#[derive(Debug)]
enum ReadError {
    FileNotFound(std::io::Error),
    DecodingError(png::DecodingError),
}

#[derive(Debug)]
enum WriteError {
    DirectoryNotFound(std::io::Error),
    EncodingError(png::EncodingError),
}

#[derive(Debug)]
pub enum PngFileError {
    ReadError(String, ReadError),
    WriteError(String, WriteError),
    Unsupported(String),
    SceneError(String), //todo: replace String with SceneError
}
impl std::fmt::Display for PngFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use PngFileError::*;
        match self {
            ReadError(path, error) => write!(f, "{}", Decorate::output(
                "PngFileError".to_string(),
                None,
                Some(format!("{}: {:?}", path, error)),
            )),
            WriteError(path, error) => write!(f, "{}", Decorate::output(
                "PngFileError".to_string(),
                None,
                Some(format!("{}: {:?}", path, error)),
            )),
            Unsupported(desc) => write!(f, "{}", Decorate::output(
                "PngFileError".to_string(),
                None,
                Some(desc.to_string()),
            )),
            SceneError(desc) => write!(f, "{}", Decorate::output(
                "PngFileError".to_string(),
                None,
                Some(desc.to_string()),
            )),
        }
    }
}

pub struct PngFile {
    height: u32,
    width: u32,
    color_type: ColorType,
    bit_depth: BitDepth,
    bytes: Vec<u8>
}

impl PngFile {
    pub fn read(path: String) -> Result<Self, PngFileError> {
        match File::open(&path) {
            Ok(file) => {
                let decoder = Decoder::new(file);
                match decoder.read_info() {
                    Ok(mut reader) => {
                        let mut buf = vec![0; reader.output_buffer_size()];
                        let info: png::OutputInfo;
                        match reader.next_frame(&mut buf) {
                            Ok(info) => {
                                let bytes = buf[..info.buffer_size()].to_vec();
                                return Ok(PngFile {
                                    height: info.height,
                                    width: info.width,
                                    color_type: info.color_type,
                                    bit_depth: info.bit_depth,
                                    bytes: bytes
                                });
                            },
                            Err(error) => {
                                return Err(PngFileError::ReadError(
                                    path,
                                    ReadError::DecodingError(error)
                                ));
                            },
                        }
                    },
                    Err(error) => {
                        return Err(PngFileError::ReadError(
                            path,
                            ReadError::DecodingError(error)
                        ));
                    },
                }
            },
            Err(error) => {
                return Err(PngFileError::ReadError(path, ReadError::FileNotFound(error)));
            },
        }
    }
    pub fn write(&self, file_path: String) -> Result<(), PngFileError> {
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
                            Err(error) => Err(PngFileError::WriteError(
                                file_path,
                                WriteError::EncodingError(error)
                            )),
                        }
                    },
                    Err(error) => Err(PngFileError::WriteError(
                        file_path,
                        WriteError::EncodingError(error)
                    )),
                };
                x
            },
            Err(error) => Err(PngFileError::WriteError(
                file_path,
                WriteError::DirectoryNotFound(error)
            )),
        }
    }
    pub fn to_scene(self) -> Result<Scene, PngFileError> {
        let dim: Coord = Coord{ x: self.height as isize, y: self.width as isize };
        let mut scene: Scene = Scene::new(dim, vec![None; dim.area() as usize]).unwrap();
        //todo: remove above unwrap when scene error literature
        match self.color_type {
            Grayscale => {
                match self.bit_depth {
                    One => {
                        return Err(PngFileError::Unsupported(
                            "One-bit Grayscale PNGs currently not supported".to_string()
                        ));
                    },
                    Two => {
                        return Err(PngFileError::Unsupported(
                            "Two-bit Grayscale PNGs currently not supported".to_string()
                        ));
                    },
                    Four => {
                        return Err(PngFileError::Unsupported(
                            "Four-bit Grayscale PNGs currently not supported".to_string()
                        ));
                    },
                    Eight => {
                        return Err(PngFileError::Unsupported(
                            "Eight-bit Grayscale PNGs currently not supported".to_string()
                        ));
                    },
                    Sixteen => {
                        return Err(PngFileError::Unsupported(
                            "Sixteen-bit Grayscale PNGs currently not supported".to_string()
                        ));
                    }
                }
            },
            Rgb => {
                match self.bit_depth {
                    One => {
                        return Err(PngFileError::Unsupported(
                            "One-bit RGB PNGs currently not supported".to_string()
                        ));
                    },
                    Two => {
                        return Err(PngFileError::Unsupported(
                            "Two-bit RGB PNGs currently not supported".to_string()
                        ));
                    },
                    Four => {
                        return Err(PngFileError::Unsupported(
                            "Four-bit RGB PNGs currently not supported".to_string()
                        ));
                    },
                    Eight => {
                        return Err(PngFileError::Unsupported(
                            "Eight-bit RGB PNGs currently not supported".to_string()
                        ));
                    },
                    Sixteen => {
                        return Err(PngFileError::Unsupported(
                            "Sixteen-bit RGB PNGs currently not supported".to_string()
                        ));
                    }
                }
            },
            Indexed => {
                match self.bit_depth {
                    One => {
                        return Err(PngFileError::Unsupported(
                            "One-bit Indexed PNGs currently not supported".to_string()
                        ));
                    },
                    Two => {
                        return Err(PngFileError::Unsupported(
                            "Two-bit Indexed PNGs currently not supported".to_string()
                        ));
                    },
                    Four => {
                        return Err(PngFileError::Unsupported(
                            "Four-bit Indexed PNGs currently not supported".to_string()
                        ));
                    },
                    Eight => {
                        return Err(PngFileError::Unsupported(
                            "Eight-bit Indexed PNGs currently not supported".to_string()
                        ));
                    },
                    Sixteen => {
                        return Err(PngFileError::Unsupported(
                            "Sixteen-bit Indexed PNGs currently not supported".to_string()
                        ));
                    }
                }
            },
            GrayscaleAlpha => {
                match self.bit_depth {
                    One => {
                        return Err(PngFileError::Unsupported(
                            "One-bit Grayscale-Alpha PNGs currently not supported".to_string()
                        ));
                    },
                    Two => {
                        return Err(PngFileError::Unsupported(
                            "Two-bit Grayscale-Alpha PNGs currently not supported".to_string()
                        ));
                    },
                    Four => {
                        return Err(PngFileError::Unsupported(
                            "Four-bit Grayscale-Alpha PNGs currently not supported".to_string()
                        ));
                    },
                    Eight => {
                        return Err(PngFileError::Unsupported(
                            "Eight-bit Grayscale-Alpha PNGs currently not supported".to_string()
                        ));
                    },
                    Sixteen => {
                        return Err(PngFileError::Unsupported(
                            "Sixteen-bit Grayscale-Alpha PNGs currently not supported".to_string()
                        ));
                    }
                }
            },
            Rgba => {
                match self.bit_depth {
                    One => {
                        return Err(PngFileError::Unsupported(
                            "One-bit RGBA PNGs currently not supported".to_string()
                        ));
                    },
                    Two => {
                        return Err(PngFileError::Unsupported(
                            "Two-bit RGBA PNGs currently not supported".to_string()
                        ));
                    },
                    Four => {
                        return Err(PngFileError::Unsupported(
                            "Four-bit RGBA PNGs currently not supported".to_string()
                        ));
                    },
                    Eight => {
                        for i in 0..self.height {
                            for j in 0..self.width {
                                scene.set_pixel(
                                    Coord{ x: i as isize, y: j as isize },
                                    Some(Pixel {
                                        r: self.bytes[((4*i*self.width) + (4*j) + 0) as usize],
                                        g: self.bytes[((4*i*self.width) + (4*j) + 1) as usize],
                                        b: self.bytes[((4*i*self.width) + (4*j) + 2) as usize],
                                        a: self.bytes[((4*i*self.width) + (4*j) + 3) as usize],
                                    })
                                ).unwrap();
                                //todo: remove above unwrap when scene error literature
                            }
                        }
                    },
                    Sixteen => {
                        return Err(PngFileError::Unsupported(
                            "Sixteen-bit RGBA PNGs currently not supported".to_string()
                        ));
                    }
                }
            }
        }
        Ok(scene)
    }
    pub fn from_scene(
        scene: &Scene,
        color_type: ColorType,
        bit_depth: BitDepth
    ) -> Result<Self, PngFileError> {
        let mut png = PngFile {
            height: scene.dim.x as u32,
            width: scene.dim.y as u32,
            color_type: color_type,
            bit_depth: bit_depth,
            bytes: Vec::new(),
        };
        match color_type {
            Grayscale => {
                match bit_depth {
                    One => {
                        return Err(PngFileError::Unsupported(
                            "One-bit Grayscale PNGs currently not supported".to_string()
                        ));
                    },
                    Two => {
                        return Err(PngFileError::Unsupported(
                            "Two-bit Grayscale PNGs currently not supported".to_string()
                        ));
                    },
                    Four => {
                        return Err(PngFileError::Unsupported(
                            "Four-bit Grayscale PNGs currently not supported".to_string()
                        ));
                    },
                    Eight => {
                        return Err(PngFileError::Unsupported(
                            "Eight-bit Grayscale PNGs currently not supported".to_string()
                        ));
                    },
                    Sixteen => {
                        return Err(PngFileError::Unsupported(
                            "Sixteen-bit Grayscale PNGs currently not supported".to_string()
                        ));
                    }
                }
            },
            Rgb => {
                match bit_depth {
                    One => {
                        return Err(PngFileError::Unsupported(
                            "One-bit RGB PNGs currently not supported".to_string()
                        ));
                    },
                    Two => {
                        return Err(PngFileError::Unsupported(
                            "Two-bit RGB PNGs currently not supported".to_string()
                        ));
                    },
                    Four => {
                        return Err(PngFileError::Unsupported(
                            "Four-bit RGB PNGs currently not supported".to_string()
                        ));
                    },
                    Eight => {
                        return Err(PngFileError::Unsupported(
                            "Eight-bit RGB PNGs currently not supported".to_string()
                        ));
                    },
                    Sixteen => {
                        return Err(PngFileError::Unsupported(
                            "Sixteen-bit RGB PNGs currently not supported".to_string()
                        ));
                    }
                }
            },
            Indexed => {
                match bit_depth {
                    One => {
                        return Err(PngFileError::Unsupported(
                            "One-bit Indexed PNGs currently not supported".to_string()
                        ));
                    },
                    Two => {
                        return Err(PngFileError::Unsupported(
                            "Two-bit Indexed PNGs currently not supported".to_string()
                        ));
                    },
                    Four => {
                        return Err(PngFileError::Unsupported(
                            "Four-bit Indexed PNGs currently not supported".to_string()
                        ));
                    },
                    Eight => {
                        return Err(PngFileError::Unsupported(
                            "Eight-bit Indexed PNGs currently not supported".to_string()
                        ));
                    },
                    Sixteen => {
                        return Err(PngFileError::Unsupported(
                            "Sixteen-bit Indexed PNGs currently not supported".to_string()
                        ));
                    }
                }
            },
            GrayscaleAlpha => {
                match bit_depth {
                    One => {
                        return Err(PngFileError::Unsupported(
                            "One-bit Grayscale-Alpha PNGs currently not supported".to_string()
                        ));
                    },
                    Two => {
                        return Err(PngFileError::Unsupported(
                            "Two-bit Grayscale-Alpha PNGs currently not supported".to_string()
                        ));
                    },
                    Four => {
                        return Err(PngFileError::Unsupported(
                            "Four-bit Grayscale-Alpha PNGs currently not supported".to_string()
                        ));
                    },
                    Eight => {
                        return Err(PngFileError::Unsupported(
                            "Eight-bit Grayscale-Alpha PNGs currently not supported".to_string()
                        ));
                    },
                    Sixteen => {
                        return Err(PngFileError::Unsupported(
                            "Sixteen-bit Grayscale-Alpha PNGs currently not supported".to_string()
                        ));
                    }
                }
            },
            Rgba => {
                match bit_depth {
                    One => {
                        return Err(PngFileError::Unsupported(
                            "One-bit RGBA PNGs currently not supported".to_string()
                        ));
                    },
                    Two => {
                        return Err(PngFileError::Unsupported(
                            "Two-bit RGBA PNGs currently not supported".to_string()
                        ));
                    },
                    Four => {
                        return Err(PngFileError::Unsupported(
                            "Four-bit RGBA PNGs currently not supported".to_string()
                        ));
                    },
                    Eight => {
                        for i in 0..scene.dim.x {
                            for j in 0..scene.dim.y {
                                let Pixel {
                                    r: red,
                                    g: green,
                                    b: blue,
                                    a: alpha
                                } = Pixel::get_certain(scene.get_pixel(
                                    Coord{ x: i as isize, y: j as isize }
                                ).unwrap());
                                //todo: remove above unwrap when scene error literature
                                png.bytes.push(red);
                                png.bytes.push(green);
                                png.bytes.push(blue);
                                png.bytes.push(alpha);
                            }
                        }
                    },
                    Sixteen => {
                        return Err(PngFileError::Unsupported(
                            "Sixteen-bit RGBA PNGs currently not supported".to_string()
                        ));
                    }
                }
            }
        }
        Ok(png)
    }
}
