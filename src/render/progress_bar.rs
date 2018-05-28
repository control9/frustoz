use std::sync::mpsc::Receiver;
use std::thread;

pub fn console_progress_bar(rx: Receiver<u32>, iterations: u32, report_percentage: u32) {
    thread::spawn(move | |  {
        let mut previous_percentage = 0;
        let mut current_percentage;
        let mut iterations_completed = 0;

        while iterations_completed < iterations {
            iterations_completed += rx.recv().unwrap();
            current_percentage = (100.0 * iterations_completed as f32 / iterations as f32) as u32;
            while current_percentage - previous_percentage >= report_percentage {
                previous_percentage += report_percentage;
                println!("Rendering progress: {}%", previous_percentage);
            }
        }
    });
}