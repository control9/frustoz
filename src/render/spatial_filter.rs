use rayon::prelude::*;
use render::HDRPixel;
use render::Histogram;
use render::pixel_filter::PixelFilter;
use std::time::Instant;

pub fn apply_filter(filter: &PixelFilter,
                    histogram: &Histogram,
                    image_width: u32,
                    image_height: u32,
                    histogram_width: u32,
                    histogram_height: u32,
                    oversample: u32) -> Histogram {
    assert_eq!(histogram_width * histogram_height, histogram.len() as u32, "Incorrect histogram size");
    let &PixelFilter{width: filter_width, coefficients: ref filter_coefficients} = filter;
    let now = Instant::now();
    let result = (0..(image_height * image_width))
        .into_par_iter()
        .map(|i| (i % image_width * oversample, i / image_width * oversample))
        .map(|(x, y)| process_point(x, y, filter_width as u32, filter_coefficients, histogram_width, histogram))
        .collect();
    let elapsed = now.elapsed();
    println!("Filtering took: {:?}", (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0));
    result
}

fn process_point(x: u32, y: u32, filter_width: u32, filter: &Vec<f64>, histogram_width: u32, histogram: &Histogram) -> HDRPixel {
    let (mut r, mut g, mut b, mut a) = (0.0, 0.0, 0.0, 0.0);

    for filter_y in 0..filter_width {
        for filter_x in 0..filter_width {
            let k = filter[(filter_x + filter_y * filter_width) as usize];
            let index = (x + filter_x) + (y + filter_y) * histogram_width;
            let &HDRPixel(rn, gn, bn, an) = &histogram[index as usize];

            r += rn * k;
            g += gn * k;
            b += bn * k;
            a += an * k;
        }
    }

    HDRPixel(r, g, b, a)
}