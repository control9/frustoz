use pbr::MultiBar;
use pbr::ProgressBar;
use std::sync::mpsc::Receiver;
use std::thread;
use super::Progress;

pub fn single_progress_bar(rx: Receiver<Progress>, iterations: u32) {
    thread::spawn(move || {
        let mut pb = ProgressBar::new(iterations as u64);
        let mut remaining = iterations as i64;
        while remaining > 0 {
            let Progress(increment, _) = rx.recv().unwrap();
            pb.add(increment as u64);
            remaining -= increment as i64;
        }
        pb.finish_println("Rendering completed")
    });
}

pub fn multi_progress_bar(rx: Receiver<Progress>, iterations: u32, iterations_per_thread: &Vec<u32>) {
    let mut mb = MultiBar::new();
    mb.println("Rendering per thread:");

    let mut bars: Vec<ProgressBar<_>> = iterations_per_thread.iter().enumerate()
        .map(|(i, size)| {
            let mut p = mb.create_bar(*size as u64);
            p.message(&format!("Thread {}: ", i + 1));
            p.format("[=> ]");
            p
        }).collect();

    thread::spawn(move || {
        mb.listen();
    });

    let mut remaining_per_thread = iterations_per_thread.clone();
    thread::spawn(move || {
        let mut remaining = iterations as u32;
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
}