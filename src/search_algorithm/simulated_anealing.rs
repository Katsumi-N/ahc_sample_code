use std::time::{Duration, Instant};
use rand::prelude::*;
use proconio::{fastout, input};

// 時間を管理する構造体
struct TimeKeeper {
    start_time: Instant,
    time_threshold: i64,
}

impl TimeKeeper {
    // 時間制限をミリ秒単位で指定してインスタンスをつくる。
    fn new(time_threshold: i64) -> Self {
        Self {
            start_time: Instant::now(),
            time_threshold,
        }
    }

    // インスタンス生成した時から指定した時間制限を超過したか判定する。
    fn is_time_over(&self) -> bool {
        self.start_time.elapsed().as_millis() >= self.time_threshold as u128
    }

    fn elapsed_time(&self) -> u128 {
        self.start_time.elapsed().as_millis()
    }
}

struct State {
    // TODO: add parameter
}

impl State {
    fn new() -> Self {
        // TODO: Implement constructor
        Self {

        }
    }

    fn get_score(&self) -> ScoreType {
        // TODO: Return the calculated score
        0
    }

    fn init(&mut self) {
        // TODO: Implement initialization logic
    }

    fn transition(&mut self) {
        // TODO: Implement the state transition logic
    }
}

fn simulated_annealing_with_time_threshold(
    state: State,
    time_threshold: i64,
    start_temp: f64,
    end_temp: f64,
) -> State {
    let mut time_keeper = TimeKeeper::new(time_threshold);
    let mut now_state = state;
    now_state.init();
    let mut best_score = now_state.get_score();
    let mut now_score = best_score;
    let mut best_state = now_state.clone();
    let mut rng = rand::thread_rng();

    loop {
        let mut next_state = now_state.clone();
        next_state.transition();
        let next_score = next_state.get_score();
        let elapsed_ratio = time_keeper.elapsed_time();
        let temp = start_temp + (end_temp - start_temp) * elapsed_ratio;
        let probability = ((next_score - now_score) as f64 / temp).exp();
        let is_force_next = probability > rng.gen_range(0.0, 1.0);
        if next_score > now_score || is_force_next {
            now_score = next_score;
            now_state = next_state;
        }
        if next_score > best_score {
            best_score = next_score;
            best_state = next_state;
        }
        time_keeper.set_now_time();
        if time_keeper.is_time_over() {
            break;
        }
    }
    best_state
}

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [[usize; n]; n]
    }
    let initial_state = State {};
    let time_threshold = 2000 as i64;
    let start_temp = 10.0;
    let end_temp = 1.0;

    let result = simulated_annealing_with_time_threshold(
        initial_state,
        time_threshold,
        start_temp,
        end_temp,
    );
    // TODO: Handle the result appropriately.
}
