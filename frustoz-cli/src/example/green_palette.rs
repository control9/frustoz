use model::palette::Palette;

pub fn palette() -> Palette {
    let mut data = [0u8 ;256 * 3];
    for i in 0..256 {
        data[3*i] =(i as f64 * 0.75) as u8;;
        data[3*i+1] = i as u8;
        data[3*i+2] = (i/16) as u8;
    }
    Palette::new(&data)
}
