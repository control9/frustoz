use rayon::prelude::*;
use render::HDRPixel;
use render::Histogram;
use std::time::Instant;
use super::FilterKernel;

pub fn apply_filter(filter: &FilterKernel,
                    histogram: &Histogram,
                    image_width: u32,
                    image_height: u32,
                    oversample: u32) -> Histogram {
    let now = Instant::now();
    let data = (0..(image_height * image_width))
        .into_par_iter()
        .map(|i| (i % image_width, i / image_width))
        .map(|(x, y)| (x * oversample, y * oversample))
        .map(|(x, y)| process_point(x, y, filter, histogram))
        .collect();
    let elapsed = now.elapsed();
    info!("Filtering took: {:?}", (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0));
    Histogram{data, width: image_width, height: image_height}
}

fn process_point(x: u32, y: u32, filter: &FilterKernel, histogram: &Histogram) -> HDRPixel {
    let &FilterKernel { width: filter_width, coefficients: ref kernel } = filter;
    let (mut r, mut g, mut b, mut a) = (0.0, 0.0, 0.0, 0.0);

    for filter_y in 0..filter_width {
        for filter_x in 0..filter_width {
            let k = kernel[(filter_x + filter_y * filter_width) as usize];
            let index = (x + filter_x) + (y + filter_y) * histogram.width;
            let &HDRPixel(rn, gn, bn, an) = &histogram.data[index as usize];

            r += rn * k;
            g += gn * k;
            b += bn * k;
            a += an * k;
        }
    }

    HDRPixel(
        r.min(1.0).max(0.0),
        g.min(1.0).max(0.0),
        b.min(1.0).max(0.0),
        a.min(1.0).max(0.0))
}