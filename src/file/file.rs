use crate::elements::common::{ Coord, Pixel };
use crate::elements::layer::{ Scene, Camera, Layer };
use crate::project::Project;

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
    pub fn open(path: String) -> Self {
        let decoder = Decoder::new(File::open(path).unwrap());
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf).unwrap();
        let bytes = buf[..info.buffer_size()].to_vec();
        println!("height: {}, width: {}", info.height, info.width);
        Png {
            height: info.height,
            width: info.width,
            color_type: info.color_type,
            bit_depth: info.bit_depth,
            bytes: bytes
        }
    }
    pub fn to_project(self) -> Result<Project, String> {
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
        let camera = Camera::new(&scene, Coord{ x: 65, y: 130 }, Coord{ x: 32, y: 32 }, 1, Coord{ x: 1, y: 2 })?;
        Ok(Project {
            layers: vec![Layer {
                scene: scene,
                opacity: 255
            }],
            camera: camera,
            selected_layer: 0,
            strokes: HashMap::new()
        })
    }
}
