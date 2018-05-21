use color::Palette;

pub fn palette() -> Palette {
    let mut data = [0u8 ;256 * 3];
    for i in 0..256 {
        data[3*i] = 0;
        data[3*i+1] = i as u8;
        data[3*i+2] = (i/4) as u8;
    }
    Palette::new("green".to_string(), &data)
}
