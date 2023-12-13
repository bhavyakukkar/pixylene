use std::fs::File;
use std::path::Path;
use std::io::BufWriter;
use png::{ Decoder, ColorType, BitDepth };
use ColorType::*;
use BitDepth::*;

use crate::elements::common::{ Coord, Pixel };
use crate::elements::layer::{ Scene };

pub struct PngFile {
    height: u32,
    width: u32,
    color_type: ColorType,
    bit_depth: BitDepth,
    bytes: Vec<u8>
}

impl PngFile {
    pub fn open(path: String) -> Result<Self, String> {
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
                                return Err(format!("error decoding next frame for '{}': {:?}", path, error));
                            },
                        }
                    },
                    Err(error) => {
                        return Err(format!("error decoding file '{}': {:?}", path, error));
                    },
                }
            },
            Err(error) => {
                return Err(format!("could not open file '{}': {:?}", path, error));
            },
        }
    }
    pub fn export(&self, path: String) -> Result<(), String> {
        let path = Path::new(&path);
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
                            Err(error) => Err(format!("{:?}", error)),
                        }
                    },
                    Err(error) => Err(format!("{:?}", error)),
                };
                x
            },
            Err(error) => Err(format!("{:?}", error)),
        }
    }
    pub fn to_scene(self) -> Result<Scene, String> {
        let dim: Coord = Coord{ x: self.height as isize, y: self.width as isize };
        let mut scene: Scene = Scene::new(dim, vec![None; dim.area() as usize])?;
        match self.color_type {
            Grayscale => {
                match self.bit_depth {
                    One => {
                        return Err(String::from("One-bit Grayscale PNGs currently not supported"));
                    },
                    Two => {
                        return Err(String::from("Two-bit Grayscale PNGs currently not supported"));
                    },
                    Four => {
                        return Err(String::from("Four-bit Grayscale PNGs currently not supported"));
                    },
                    Eight => {
                        return Err(String::from("Eight-bit Grayscale PNGs currently not supported"));
                    },
                    Sixteen => {
                        return Err(String::from("Sixteen-bit Grayscale PNGs currently not supported"));
                    }
                }
            },
            Rgb => {
                match self.bit_depth {
                    One => {
                        return Err(String::from("One-bit RGB PNGs currently not supported"));
                    },
                    Two => {
                        return Err(String::from("Two-bit RGB PNGs currently not supported"));
                    },
                    Four => {
                        return Err(String::from("Four-bit RGB PNGs currently not supported"));
                    },
                    Eight => {
                        return Err(String::from("Eight-bit RGB PNGs currently not supported"));
                    },
                    Sixteen => {
                        return Err(String::from("Sixteen-bit RGB PNGs currently not supported"));
                    }
                }
            },
            Indexed => {
                match self.bit_depth {
                    One => {
                        return Err(String::from("One-bit Indexed PNGs currently not supported"));
                    },
                    Two => {
                        return Err(String::from("Two-bit Indexed PNGs currently not supported"));
                    },
                    Four => {
                        return Err(String::from("Four-bit Indexed PNGs currently not supported"));
                    },
                    Eight => {
                        return Err(String::from("Eight-bit Indexed PNGs currently not supported"));
                    },
                    Sixteen => {
                        return Err(String::from("Sixteen-bit Indexed PNGs currently not supported"));
                    }
                }
            },
            GrayscaleAlpha => {
                match self.bit_depth {
                    One => {
                        return Err(String::from("One-bit Grayscale-Alpha PNGs currently not supported"));
                    },
                    Two => {
                        return Err(String::from("Two-bit Grayscale-Alpha PNGs currently not supported"));
                    },
                    Four => {
                        return Err(String::from("Four-bit Grayscale-Alpha PNGs currently not supported"));
                    },
                    Eight => {
                        return Err(String::from("Eight-bit Grayscale-Alpha PNGs currently not supported"));
                    },
                    Sixteen => {
                        return Err(String::from("Sixteen-bit Grayscale-Alpha PNGs currently not supported"));
                    }
                }
            },
            Rgba => {
                match self.bit_depth {
                    One => {
                        return Err(String::from("One-bit RGBA PNGs currently not supported"));
                    },
                    Two => {
                        return Err(String::from("Two-bit RGBA PNGs currently not supported"));
                    },
                    Four => {
                        return Err(String::from("Four-bit RGBA PNGs currently not supported"));
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
                                )?;
                            }
                        }
                    },
                    Sixteen => {
                        return Err(String::from("Sixteen-bit RGBA PNGs currently not supported"));
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
    ) -> Result<Self, String> {
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
                        return Err(String::from("One-bit Grayscale PNGs currently not supported"));
                    },
                    Two => {
                        return Err(String::from("Two-bit Grayscale PNGs currently not supported"));
                    },
                    Four => {
                        return Err(String::from("Four-bit Grayscale PNGs currently not supported"));
                    },
                    Eight => {
                        return Err(String::from("Eight-bit Grayscale PNGs currently not supported"));
                    },
                    Sixteen => {
                        return Err(String::from("Sixteen-bit Grayscale PNGs currently not supported"));
                    }
                }
            },
            Rgb => {
                match bit_depth {
                    One => {
                        return Err(String::from("One-bit RGB PNGs currently not supported"));
                    },
                    Two => {
                        return Err(String::from("Two-bit RGB PNGs currently not supported"));
                    },
                    Four => {
                        return Err(String::from("Four-bit RGB PNGs currently not supported"));
                    },
                    Eight => {
                        return Err(String::from("Eight-bit RGB PNGs currently not supported"));
                    },
                    Sixteen => {
                        return Err(String::from("Sixteen-bit RGB PNGs currently not supported"));
                    }
                }
            },
            Indexed => {
                match bit_depth {
                    One => {
                        return Err(String::from("One-bit Indexed PNGs currently not supported"));
                    },
                    Two => {
                        return Err(String::from("Two-bit Indexed PNGs currently not supported"));
                    },
                    Four => {
                        return Err(String::from("Four-bit Indexed PNGs currently not supported"));
                    },
                    Eight => {
                        return Err(String::from("Eight-bit Indexed PNGs currently not supported"));
                    },
                    Sixteen => {
                        return Err(String::from("Sixteen-bit Indexed PNGs currently not supported"));
                    }
                }
            },
            GrayscaleAlpha => {
                match bit_depth {
                    One => {
                        return Err(String::from("One-bit Grayscale-Alpha PNGs currently not supported"));
                    },
                    Two => {
                        return Err(String::from("Two-bit Grayscale-Alpha PNGs currently not supported"));
                    },
                    Four => {
                        return Err(String::from("Four-bit Grayscale-Alpha PNGs currently not supported"));
                    },
                    Eight => {
                        return Err(String::from("Eight-bit Grayscale-Alpha PNGs currently not supported"));
                    },
                    Sixteen => {
                        return Err(String::from("Sixteen-bit Grayscale-Alpha PNGs currently not supported"));
                    }
                }
            },
            Rgba => {
                match bit_depth {
                    One => {
                        return Err(String::from("One-bit RGBA PNGs currently not supported"));
                    },
                    Two => {
                        return Err(String::from("Two-bit RGBA PNGs currently not supported"));
                    },
                    Four => {
                        return Err(String::from("Four-bit RGBA PNGs currently not supported"));
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
                                )?);
                                png.bytes.push(red);
                                png.bytes.push(green);
                                png.bytes.push(blue);
                                png.bytes.push(alpha);
                            }
                        }
                    },
                    Sixteen => {
                        return Err(String::from("Sixteen-bit RGBA PNGs currently not supported"));
                    }
                }
            }
        }
        Ok(png)
    }
}
