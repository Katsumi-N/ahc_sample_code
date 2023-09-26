use std::collections::BinaryHeap;
use std::time::{Duration, Instant};

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

type ScoreType = i64;
const INF: ScoreType = 1_000_000_000;

#[derive(Clone, Eq, PartialEq)]
struct State {
    evaluated_score: ScoreType,
    first_action: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.evaluated_score.cmp(&self.evaluated_score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn new() -> Self {
        State {
            evaluated_score: 0,
            first_action: -1,
        }
    }

    fn is_done(&self) -> bool {
        // TODO: Implement the termination condition.
        false
    }

    fn evaluate_score(&mut self) {
        // TODO: Implement the score evaluation logic.
        self.evaluated_score = 0;
    }

    fn advance(&mut self, action: i32) {
        // TODO: Implement the logic to advance the game by the given action.
    }

    fn legal_actions(&self) -> Vec<i32> {
        // TODO: Implement the logic to get all legal actions.
        vec![]
    }
}

fn beam_search_action_with_time_threshold(state: State, beam_width: usize, time_threshold: Duration) -> i32 {
    let time_keeper = TimeKeeper::new(time_threshold);
    let mut now_beam = BinaryHeap::new();
    let mut best_state = state.clone();

    now_beam.push(state);

    loop {
        let mut next_beam = BinaryHeap::new();
        for _ in 0..beam_width {
            if time_keeper.is_time_over() {
                return best_state.first_action;
            }
            if let Some(mut now_state) = now_beam.pop() {
                for &action in now_state.legal_actions().iter() {
                    let mut next_state = now_state.clone();
                    next_state.advance(action);
                    next_state.evaluate_score();
                    if now_state.first_action == -1 {
                        next_state.first_action = action;
                    }
                    next_beam.push(next_state);
                }
            }
        }
        now_beam = next_beam;
        if let Some(state) = now_beam.peek() {
            best_state = state.clone();
        }
        if best_state.is_done() {
            break;
        }
    }
    best_state.first_action
}

fn main() {
    input! {
        n: usize,
        m: usize,
        grid: [[usize; n]; m]
    }
    let initial_state = State::new();
    let beam_width = 10;
    let time_threshold = Duration::from_millis(1000);

    let action = beam_search_action_with_time_threshold(initial_state, beam_width, time_threshold);
    println!("Selected action: {}", action);
}
