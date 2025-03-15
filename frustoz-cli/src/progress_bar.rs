use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

use crate::render::Progress;
use crate::render::ProgressReporter;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;

pub struct SingleProgressBar {
    remaining: u64,
    pb: ProgressBar,
}

impl ProgressReporter for SingleProgressBar {
    fn new(iterations_per_thread: &Vec<u64>) -> Self {
        let iterations = iterations_per_thread.iter().map(|&x| x as u64).sum();
        SingleProgressBar {
            remaining: iterations,
            pb: ProgressBar::new(iterations),
        }
    }

    fn report(&mut self, progress: Progress) {
        let mut increment = progress.0 as u64;
        increment = increment.min(self.remaining);
        self.pb.inc(increment);
        self.remaining -= increment;
        if self.remaining == 0 {
            self.pb.finish_with_message("Rendering completed");
        }
    }
}

#[derive(Clone)]
pub struct MultiProgressBar {
    tx: Sender<Progress>,
}

impl ProgressReporter for MultiProgressBar {
    fn new(iterations_per_thread: &Vec<u64>) -> Self {
        let style = ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .progress_chars("#>-");
        let iterations: u64 = iterations_per_thread.iter().map(|&x| x as u64).sum();
        let mb = crate::MB.get_or_init(|| MultiProgress::new()).clone();
        mb.clear().expect("Failed to clear progress bar");
        mb.println("Rendering per thread:").expect("Failed to start reporting progress");

        let bars: Vec<ProgressBar> = iterations_per_thread
            .iter()
            .enumerate()
            .map(|(i, &size)| {
                let p = mb.add(ProgressBar::new(size));
                p.set_style(style.clone());
                p.set_message(format!("Thread {}: ", i + 1));
                p
            })
            .collect();

        let (tx, rx) = mpsc::channel();

        let mut remaining_per_thread = iterations_per_thread.clone();
        thread::spawn(move || {
            let mut remaining = iterations;
            while remaining > 0 {
                let Progress(increment, i) = rx.recv().unwrap();
                bars[i].inc(increment as u64);
                remaining_per_thread[i] -= increment.min(remaining_per_thread[i]);
                if remaining_per_thread[i] == 0 {
                    bars[i].finish_with_message(format!("Thread {} FINISHED", i + 1));
                }
                remaining -= increment.min(remaining);
            }
        });
        Self { tx }
    }
    fn report(&mut self, progress: Progress) {
        self.tx.send(progress).unwrap();
    }
}
