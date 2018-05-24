use render::canvas::Histogram;
use render::HDRPixel;
use rayon::prelude::*;


const GAMMA_FACTOR: f64 = 1.0 / 2.2;
const EPSILON : f64 = 0.0000000001;

pub struct HistogramProcessor {
    quality: u32,
    width: u32,
    height: u32,
    oversampling: u32,
}

impl HistogramProcessor {
    pub fn new(quality: u32, width: u32, height: u32, oversampling: u32) -> Self {
        HistogramProcessor{quality, width, height, oversampling}
    }

    pub fn process_to_raw(&self, histograms: Vec<Histogram>) -> Vec<u8> {
        let histogram : Histogram = HistogramProcessor::combine(histograms);
        self.do_process(histogram)
    }

    fn combine(histograms: Vec<Histogram>) -> Histogram {
        let length = histograms.iter()
            .map(|h| h.len())
            .min().unwrap_or(0);

         (0..length).into_par_iter()
            .map(|i| {
                let (mut r, mut b, mut g, mut a) = (0.0, 0.0, 0.0, 0.0);
                for hist in &histograms {
                    add_pixel(&mut r,&mut g,&mut b,&mut a, &hist[i]);
                }
                HDRPixel(r, g, b, a)
        }).collect()
    }

    fn do_process(&self, histogram: Histogram) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];

        for j in 0..self.height {
            for i in 0..self.width {
                let HDRPixel(r, g, b, a) = self.extract_pixel(i, j, &histogram);
                let scale: f64 = 2.5 * (1.0 + a).log10() / self.quality as f64;
                let mut new_a = a * scale;
                let gamma_scale;
                if new_a < EPSILON {
                    gamma_scale = 1.0;
                } else {
                    let gamma_a = apply_gamma(new_a);
                    gamma_scale = gamma_a / new_a;
                }

                result.push((r * scale * gamma_scale) as u8);
                result.push((g * scale * gamma_scale) as u8);
                result.push((b * scale * gamma_scale) as u8);
            }
        }
        result
    }

    fn extract_pixel(&self, x:u32, y:u32, histogram: &Histogram) -> HDRPixel {
        let (mut r, mut b, mut g, mut a) = (0.0, 0.0, 0.0, 0.0);
        let os = self.oversampling;
        for j in 0..os {
            for i in 0..os {
                let index = (x * os + i) + (y * os + j) * &self.width * os;
                let pixel = &histogram[index as usize];
                add_pixel(&mut r,&mut g,&mut b,&mut a, pixel);
            }
        }
        HDRPixel(r, g, b, a)
    }
}

fn add_pixel(r : &mut f64,g : &mut f64,b : &mut f64,a : &mut f64, &HDRPixel(rn, gn, bn, an) : &HDRPixel) {
    *r = *r + rn;
    *g = *g + gn;
    *b = *b + bn;
    *a = *a + an;
}

fn apply_gamma(color: f64) -> f64 {
    (color / 255.0).min(1.0).max(0.0).powf(GAMMA_FACTOR) * 255.0
}

