use std::sync::mpsc::Receiver;
use std::thread;
use pbr::ProgressBar;

pub fn console_progress_bar(rx: Receiver<u32>, iterations: u32) {
    thread::spawn(move | |  {
        let mut pb = ProgressBar::new(iterations as u64);
        let mut remaining = iterations as i64;
        while remaining > 0 {
            let increment = rx.recv().unwrap() as u64;
            pb.add(increment);
            remaining -= increment as i64;
        }
        pb.finish_println("Rendering completed")
    });
}