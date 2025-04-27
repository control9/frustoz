use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::atomic::Ordering::Relaxed;

pub mod histogram;

#[derive(Clone)]
pub struct Canvas {
    data: Vec<HDRPixel>,
    width: u32,
    height: u32,
}


pub struct HDRPixel(pub AtomicU32, pub AtomicU32, pub AtomicU32, pub AtomicU32);

impl HDRPixel {
    fn zero() -> Self {
        HDRPixel(
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
            AtomicU32::new(0),
        )
    }
    
    fn add(&self, (a,b,c,d): (u32, u32, u32, u32)) {
        _ = &self.0.fetch_add(a, Relaxed);
        _ = &self.1.fetch_add(b, Relaxed);
        _ = &self.2.fetch_add(c, Relaxed);
        _ = &self.3.fetch_add(d, Relaxed);
    }
}

impl Default for HDRPixel {
    fn default() -> Self {
        Self::zero()
    }
}

impl Clone for HDRPixel {
    fn clone(&self) -> Self {
        HDRPixel(
            AtomicU32::new(*&self.0.load(Relaxed)),
            AtomicU32::new(*&self.1.load(Relaxed)),
            AtomicU32::new(*&self.2.load(Relaxed)),
            AtomicU32::new(*&self.3.load(Relaxed)),
        )
    }
}


#[derive(Copy, Clone)]
pub struct Progress(pub u64, pub usize);

pub trait ProgressReporter {
    fn new(iterations_per_thread: &Vec<u64>) -> Self;
    fn report(&mut self, progress: Progress);
}

#[derive(Copy, Clone)]
pub struct NoOpReporter {}