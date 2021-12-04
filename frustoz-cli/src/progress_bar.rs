use pbr::{MultiBar, Pipe};
use pbr::ProgressBar;
use std::sync::mpsc;
use std::sync::mpsc::{Sender};
use std::thread;
use crate::render::Progress;
use crate::render::ProgressReporter;
use std::io::Stdout;

pub struct SingleProgressBar {
    remaining: u64,
    pb: ProgressBar<Stdout>,
}

impl ProgressReporter for SingleProgressBar {
    fn new(iterations_per_thread: &Vec<u64>) -> Self {
        let iterations = iterations_per_thread.iter().map(|&x| x as u64).sum();
        SingleProgressBar{
            remaining: iterations,
            pb: ProgressBar::new(iterations)
        }
    }

    fn report(&mut self, progress: Progress) {
        let mut increment = progress.0 as u64;
        increment = increment.min(self.remaining);
        self.pb.add(increment);
        self.remaining -= increment;
        if self.remaining == 0 {
            self.pb.finish_println("Rendering completed");
        }
    }
}

#[derive(Clone)]
pub struct MultiProgressBar {
    tx: Sender<Progress>,
}

impl ProgressReporter for MultiProgressBar {
    fn new(iterations_per_thread: &Vec<u64>) -> Self {
        let iterations: u64 = iterations_per_thread.iter().map(|&x| x as u64).sum();
        let mb = MultiBar::new();
        mb.println("Rendering per thread:");

        let mut bars: Vec<ProgressBar<Pipe>> = iterations_per_thread.iter().enumerate()
            .map(|(i, size)| {
                let mut p = mb.create_bar(*size as u64);
                p.message(&format!("Thread {}: ", i + 1));
                p.format("[=> ]");
                p
            }).collect();

        thread::spawn(move || {
            mb.listen();
        });

        let (tx, rx) = mpsc::channel();

        let mut remaining_per_thread = iterations_per_thread.clone();
        thread::spawn(move || {
            let mut remaining = iterations;
            while remaining > 0 {
                let Progress(increment, i) = rx.recv().unwrap();
                bars[i].add(increment as u64);
                remaining_per_thread[i] -= increment.min(remaining_per_thread[i]);
                if remaining_per_thread[i] == 0 {
                    bars[i].finish_print(&format!("Thread {} FINISHED", i + 1));
                }
                remaining -= increment.min(remaining);
            }
        });
        Self{tx}
    }

    fn report(&mut self, progress: Progress) {
        self.tx.send(progress).unwrap();
    }
}