use std::fmt::{Debug, Error, Formatter};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct RGB(pub f32, pub f32, pub f32);

#[derive(Clone)]
pub struct Palette {
    size: i32,
    colors: Vec<RGB>,
}

impl Debug for Palette {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Palette size [{}]", self.size)
    }
}

impl Palette {
    pub fn new(content: &[u8]) -> Self {
        let size = (content.len() / 3) as i32;
        let colors: Vec<RGB> = content
            .chunks(3)
            .take(size as usize)
            .map(|s| {
                RGB(
                    s[0] as f32 / 256.0,
                    s[1] as f32 / 256.0,
                    s[2] as f32 / 256.0,
                )
            })
            .collect();
        Palette { size, colors }
    }

    pub fn get_color(&self, color: f64) -> &RGB {
        let index = (color * self.size as f64) as usize;
        &self.colors[index]
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct RGB_INT(pub u8, pub u8, pub u8);

#[derive(Clone)]
pub struct PaletteU8 {
    size: u32,
    colors: Vec<RGB_INT>,
}

impl Debug for PaletteU8 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Palette size [{}]", self.size)
    }
}

impl PaletteU8 {
    pub fn new(content: &[u8]) -> Self {
        let size = (content.len() / 3) as u32;
        let colors: Vec<RGB_INT> = content
            .chunks(3)
            .take(size as usize)
            .map(|s| { RGB_INT(s[0], s[1] , s[2]) })
            .collect();
        PaletteU8 { size, colors }
    }

    pub fn get_color(&self, color: f64) -> &RGB_INT {
        let index = (color * self.size as f64) as usize;
        &self.colors[index]
    }
}