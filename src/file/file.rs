use crate::elements::common::{ Coord, Pixel };
use crate::elements::layer::{ Scene, Camera, Layer };

use std::fs::File;
use png::{ Decoder, ColorType, BitDepth };
use ColorType::*;
use BitDepth::*;
use std::collections::HashMap;

pub struct Png {
    height: u32,
    width: u32,
    color_type: png::ColorType,
    bit_depth: png::BitDepth,
    bytes: Vec<u8>
}

impl Png {
    pub fn open(path: String) -> Result<Self, String> {
        if let Ok(file) = File::open(&path) {
            let decoder = Decoder::new(file);
            if let Ok(mut reader) = decoder.read_info() {
                let mut buf = vec![0; reader.output_buffer_size()];
                let info: png::OutputInfo;
                if let Ok(info) = reader.next_frame(&mut buf) {
                    let bytes = buf[..info.buffer_size()].to_vec();
                    //println!("height: {}, width: {}", info.height, info.width);
                    Ok(Png {
                        height: info.height,
                        width: info.width,
                        color_type: info.color_type,
                        bit_depth: info.bit_depth,
                        bytes: bytes
                    })
                } else {
                    return Err(format!("error decoding next frame for '{}'", path));
                }
            } else {
                return Err(format!("error decoding file '{}'", path));
            }
        } else {
            return Err(format!("could not open file '{}'", path));
        }
    }
    pub fn to_scene(self) -> Result<Scene, String> {
        let dim: Coord = Coord{ x: self.height as isize, y: self.width as isize };
        let mut scene: Scene = Scene::new(dim, vec![None; dim.area() as usize])?;
        match self.color_type {
            png::ColorType::Grayscale => {
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
            png::ColorType::Rgb => {
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
            png::ColorType::Indexed => {
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
            png::ColorType::GrayscaleAlpha => {
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
            png::ColorType::Rgba => {
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
}
