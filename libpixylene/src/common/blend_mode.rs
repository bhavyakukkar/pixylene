use crate::common::Pixel;

pub enum BlendMode {
    Overwrite,
    Normal,
}

impl BlendMode {
    pub fn merge_down(&self, top: Pixel, bottom: Pixel) -> Pixel {
        match self {
            Self::Overwrite => {
                top
            },
            Self::Normal => {
                //todo!();
                if top.a == 255 {
                    top
                }
                else if top.a == 0 {
                    bottom
                }
                else {
                    let mut sum: Pixel = Pixel{ r: 0, g: 0, b: 0, a: 0 };
                    sum.a = top.a + ((bottom.a / 255)*(255 - top.a));
                    sum.r = (((top.a*top.r) + (bottom.a*bottom.r)) as u16/510).try_into().unwrap();
                    sum.g = (((top.a*top.g) + (bottom.a*bottom.g)) as u16/510).try_into().unwrap();
                    sum.b = (((top.a*top.b) + (bottom.a*bottom.b)) as u16/510).try_into().unwrap();
                    sum
                }
                /*
                let r = (((top.a as f32)*(top.r as f32))/((top.a as u16 + bottom.a as u16) as f32) + (((bottom.a as f32)*(bottom.r as f32))/((top.a as u16 + bottom.a as u16) as f32))) as u8;
                let g = (((top.a as f32)*(top.g as f32))/((top.a as u16 + bottom.a as u16) as f32) + (((bottom.a as f32)*(bottom.g as f32))/((top.a as u16 + bottom.a as u16) as f32))) as u8;
                let b = (((top.a as f32)*(top.b as f32))/((top.a as u16 + bottom.a as u16) as f32) + (((bottom.a as f32)*(bottom.b as f32))/((top.a as u16 + bottom.a as u16) as f32))) as u8;
                let a = std::cmp::max(0u16, std::cmp::min(255u16, bottom.a as u16 + (((top.a as f32)/((256 as u16 - bottom.a as u16) as f32)) as u16))) as u8;
                Pixel{ r, g, b, a }
                */
            },
        }
    }
}
