use render::canvas::HistogramLayer;
use render::HDRPixel;
use render::Histogram;
use std::f64::consts::FRAC_2_PI;
use std::f64::consts::PI;

const GAUSS_SUPPORT: f64 = 1.5;

fn _sinc(x: f64) -> f64 {
    let xt = x * PI;
    match xt {
        0.0 => 1.0,
        xt => xt.sin() / xt
    }
}

fn gaussian(x: f64) -> f64 {
    (-2.0 * x * x).exp() * (FRAC_2_PI)
}

pub fn create_filter(_filter_type: u32, oversample: u32, radius: f64) -> (usize, Vec<f64>) {
    let fw: f64 = 2.0 * oversample as f64 * radius as f64 * GAUSS_SUPPORT;
    let mut filter_width = fw as usize + 1;
    if (filter_width as u32 ^ oversample) == 1 {
        filter_width += 1;
    }

    let size = filter_width * filter_width;
    let adjust = match fw {
        0.0 => 1.0,
        fw => GAUSS_SUPPORT * filter_width as f64 / fw
    };

    let mut filter = vec![];
    for j in 0..filter_width {
        for i in 0..filter_width {
            let ii = adjust * ((2.0 * i as f64 + 1.0) / filter_width as f64 - 1.0);
            let jj = adjust * ((2.0 * j as f64 + 1.0) / filter_width as f64 - 1.0);

            filter.push(gaussian(ii) + gaussian(jj));
        }
    }
    (filter_width, filter)
}


pub fn apply_filter( (&filter_width, filter): (&usize, &Vec<f64>),
                    histogram: &Histogram,
                    image_width: u32,
                    image_height: u32,
                    histogram_width: u32,
                    histogram_height: u32,
                    oversample: u32) -> Histogram {
    assert_eq!(histogram_width * histogram_height, histogram.len() as u32, "Incorrect histogram size");

    let mut result = vec![];
    result.reserve(image_width as usize * image_height as usize);

    for j in 0..image_height as usize {
        let y = j * oversample as usize;
        for i in 0..image_width as usize {
            let x = i * oversample as usize;

            let (mut r, mut g, mut b, mut a) = (0.0, 0.0, 0.0, 0.0);

            for filter_y in 0..filter_width {
                for filter_x in 0..filter_width {
                    let k = filter[filter_x + filter_y * filter_width];
                    let index = (x + filter_x) + (y + filter_y) * histogram_width as usize;
                    let &HDRPixel(rn, gn, bn, an) = &histogram[indexK];

                    r += rn * k;
                    g += gn * k;
                    b += bn * k;
                    a += an * k;
                }
            }

            result.push(HDRPixel(r, g, b, a))
        }
    }
    result
}