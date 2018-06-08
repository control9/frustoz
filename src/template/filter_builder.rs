
use render::filter::FilterKernel;
use std::time::Instant;
use template::FilterConfig;
use util::math::EPSILON;

pub fn filter(&FilterConfig { filter_type, radius }: &FilterConfig, oversample: u32) -> FilterKernel {
    let now = Instant::now();
    let fw: f64 = 2.0 * oversample as f64 * radius as f64 * filter_type.get_spatial_support();

    let mut filter_width = fw as u32 + 1;
    if (filter_width as u32 ^ oversample) == 1 {
        filter_width += 1;
    }

    let adjust = match fw {
        zero if zero < EPSILON => 1.0,
        fw => filter_type.get_spatial_support() * filter_width as f64 / fw
    };

    let mut filter = vec![];
    for j in 0..filter_width {
        for i in 0..filter_width {
            let ii = adjust * ((2.0 * i as f64 + 1.0) / filter_width as f64 - 1.0);
            let jj = adjust * ((2.0 * j as f64 + 1.0) / filter_width as f64 - 1.0);

            filter.push(filter_type.apply(ii) * filter_type.apply(jj));
        }
    }
    normalize(&mut filter);

    let elapsed = now.elapsed();
    println!("Creating filter took: {:?}, filter width: {}", (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0), filter_width);
    FilterKernel { width: filter_width, coefficients: filter }
}

fn normalize(matrix: &mut Vec<f64>){
    let sum: f64 = matrix.iter().sum();
    assert!(sum.abs() >= EPSILON);
    for x in matrix.iter_mut() {
        *x = *x / sum;
    }
}