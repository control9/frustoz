use super::histogram::Camera;
use super::Histogram;
use super::Progress;
use super::ProgressReporter;
use crate::model::builders;
use crate::model::flame::Flame;
use crate::util::math::RealPoint;
use rand::prelude::*;

const SKIP_ITERATIONS: u64 = 20;
const REPORT_FREQUENCY_PERCENT: u64 = 1;

const SPLIT_FACTOR: usize = 32;

pub struct SplitRenderTask<T: ProgressReporter + Sized> {
    camera: Camera,
    canvas: Histogram,
    flame: Flame,
    iterations: u64,
    id: usize,
    progress_reporter: T,
}

#[derive(Debug)]
struct State {
    point: RealPoint,
    color: f64,
    rng: ThreadRng,
}

impl<T: ProgressReporter + Sized> SplitRenderTask<T> {
    pub fn new(flame: Flame, iterations: u64, id: usize, progress_reporter: T) -> Self {
        let camera = builders::camera(&flame.camera);
        let canvas = builders::histogram(&flame.render, flame.filter.width);

        SplitRenderTask {
            camera,
            canvas,
            flame,
            iterations,
            id,
            progress_reporter,
        }
    }

    pub fn render(mut self) -> Histogram {
        let report_frequency = self.iterations / 100 * REPORT_FREQUENCY_PERCENT;
        let mut progress = Progress(0, self.id);
        let rng = thread_rng();
        let stt: Vec<State> = (0..SPLIT_FACTOR)
            .map(|_| rng.clone())
            .map(|mut rng| State {
                point: RealPoint(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)),
                color: rng.gen_range(0.0..1.0),
                rng: rng.clone(),
            })
            .collect::<Vec<State>>();
        let mut state: [State; SPLIT_FACTOR] = stt.try_into().expect("AAA");

        for iteration in (0..self.iterations).step_by(SPLIT_FACTOR) {
            for i in 0..SPLIT_FACTOR {
                let State {
                    point,
                    color,
                    ref mut rng,
                } = &mut state[i];
                let trs = rng.gen_range(0.0..1.0);
                let tr = self.flame.transforms.get_transformation(trs);
                let (new_p, new_c) = tr.apply(point, *color, rng);
                state[i].point = new_p;
                state[i].color = new_c;
            }
            progress.0 += SPLIT_FACTOR as u64;

            if progress.0 > report_frequency {
                self.progress_reporter.report(progress);
                progress.0 -= report_frequency;
            }

            if iteration > (SKIP_ITERATIONS * SPLIT_FACTOR as u64) {
                state.iter().for_each(|p| {
                    let camera_coordinates = self.camera.project(&p.point);
                    let color = self.flame.palette.get_color(p.color);
                    self.canvas.project_and_update(&camera_coordinates, color);
                });
            }
        }
        self.progress_reporter.report(progress);
        self.canvas
    }
}
