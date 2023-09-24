use std::time::{Duration, Instant};
use rand::prelude::*;

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

const H: usize = 5;
const W: usize = 5;
const END_TURN: usize = 5;
const CHARACTER_N: usize = 3;

type ScoreType = i64;
const INF: ScoreType = 1_000_000_000;

struct State;

impl State {
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
    time_threshold: Duration,
    start_temp: f64,
    end_temp: f64,
) -> State {
    let mut time_keeper = TimeKeeperDouble::new(time_threshold);
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
        let elapsed_ratio =
            time_keeper.get_now_time().as_secs_f64() / time_threshold.as_secs_f64();
        let temp = start_temp + (end_temp - start_temp) * elapsed_ratio;
        let probability = ((next_score - now_score) as f64 / temp).exp();
        let is_force_next = probability > rng.gen_range(0.0..1.0);
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
    let initial_state = State {};
    let time_threshold = Duration::from_millis(1000);
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