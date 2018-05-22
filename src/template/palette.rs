#[derive(Debug, PartialEq)]
pub struct RGB(pub u8, pub u8, pub u8);

pub struct Palette {
    size: i32,
    colors: Vec<RGB>,
}

impl Palette {
    pub fn new(content: &[u8]) -> Self {
        let size = (content.len() / 3) as i32;
        let colors: Vec<RGB> = content.chunks(3)
            .map(|s| RGB(s[0], s[1], s[2]))
            .collect();
        Palette { size, colors }
    }

    pub fn get_color(&self, color: f64) -> &RGB {
        let index = (color * self.size as f64) as usize;
        &self.colors[index]
    }
}