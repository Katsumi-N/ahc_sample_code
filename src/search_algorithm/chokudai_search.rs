use std::collections::BinaryHeap;

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
}

type ScoreType = i64; // ゲームの評価スコアの型を決めておく。
const INF: ScoreType = 1000000000; // あり得ないぐらい大きなスコアの例を用意しておく

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    evaluated_score: ScoreType, // 探索上で評価したスコア
    first_action: i32,          // 探索木のルートノードで最初に選択した行動
}

impl State {
    // ゲームの終了判定
    fn is_done(&self) -> bool {
        /*処理*/
        // return /*(bool)終了判定*/;
        false // 仮の実装
    }

    // 探索用の盤面評価をする
    fn evaluate_score(&mut self) {
        self.evaluated_score = /*(ScoreType)評価結果*/;
        // 仮の実装
    }

    // 指定したactionでゲームを1ターン進める
    fn advance(&mut self, action: i32) {
        /*処理*/
    }

    // 現在の状況でプレイヤーが可能な行動を全て取得する
    fn legal_actions(&self) -> Vec<i32> {
        let actions: Vec<i32> = Vec::new();
        /*処理*/
        actions
    }
}

// 探索時のソート用に評価を比較する
use std::cmp::Ord;
use std::cmp::Ordering;

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.evaluated_score.cmp(&self.evaluated_score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// ビーム1本あたりのビームの幅と深さ、制限時間(ms)を指定してchokudaiサーチで行動を決定する
fn chokudai_search_action_with_time_threshold(
    state: &State,
    beam_width: usize,
    beam_depth: usize,
    time_threshold: i64,
) -> i32 {
    let time_keeper = TimeKeeper::new(time_threshold);
    let mut beam: Vec<BinaryHeap<State>> = vec![BinaryHeap::new(); beam_depth + 1];
    beam[0].push(*state);
    loop {
        for t in 0..beam_depth {
            let now_beam = &mut beam[t];
            let next_beam = &mut beam[t + 1];
            for _ in 0..beam_width {
                if let Some(mut now_state) = now_beam.pop() {
                    if now_state.is_done() {
                        break;
                    }
                    let legal_actions = now_state.legal_actions();
                    for action in legal_actions {
                        let mut next_state = now_state;
                        next_state.advance(action);
                        next_state.evaluate_score();
                        if t == 0 {
                            next_state.first_action = action;
                        }
                        next_beam.push(next_state);
                    }
                } else {
                    break;
                }
            }
        }
        if time_keeper.is_time_over() {
            break;
        }
    }
    for t in (0..=beam_depth).rev() {
        if let Some(final_state) = beam[t].pop() {
            return final_state.first_action;
        }
    }

    -1
}