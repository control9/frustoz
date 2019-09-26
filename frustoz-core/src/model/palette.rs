#[derive(Debug, PartialEq, Copy, Clone)]
pub struct RGB(pub f32, pub f32, pub f32);

#[derive(Clone, Debug)]
pub struct Palette {
    size: i32,
    colors: Vec<RGB>,
}

impl Palette {
    pub fn new(content: &[u8]) -> Self {
        let size = (content.len() / 3) as i32;
        let colors: Vec<RGB> = content.chunks(3)
            .take(size as usize)
            .map(|s| RGB(s[0] as f32 / 256.0, s[1] as f32 / 256.0, s[2] as f32 / 256.0))
            .collect();
        Palette { size, colors }
    }

    pub fn get_color(&self, color: f64) -> &RGB {
        let index = (color * self.size as f64) as usize;
        &self.colors[index]
    }
}