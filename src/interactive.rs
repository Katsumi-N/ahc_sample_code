use proconio::input;
use proconio::source::line::LineSource;
use std::io::{stdin, BufRead, BufReader};
use rand::Rng;
use std::time::Instant;
use rand_distr::{Normal, Distribution};

// 座標を保持する
#[derive(Clone,Copy)]
struct Coord {
    y: usize,
    x: usize,
}

enum Mode {
    Simple,
    HighSTD,
}

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

struct Judge;

impl Judge {
    fn set_temperature(&self, temperature: &Vec<Vec<usize>>) {
        for row in temperature {
            println!("{}", row.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(" "));
        }
    }

    fn answer(&self, estimate: &Vec<usize>) {
        println!("-1 -1 -1");
        for &e in estimate {
            println!("{}", e);
        }
    }

    fn measure<R: BufRead>(&self, source: &mut LineSource<R>, i: usize, y: isize, x: isize) -> isize {
        println!("{} {} {}", i, y, x);
        input!{
            from source,
            v: isize,
        };

        if v == -1 {
            eprintln!("something went wrong. i={} y={} x={}", i, y, x);
            std::process::exit(1);
        }
        v
    }
}

struct Solver {
    L: usize,
    N: usize,
    S: usize,
    landing_pos: Vec<Coord>,
    judge: Judge,
}

impl Solver {
    fn new(L: usize, N: usize, S: usize, landing_pos: Vec<Coord>) -> Solver {
        Solver {
            L,
            N,
            S,
            landing_pos,
            judge: Judge,
        }
    }

    fn solve<R: BufRead>(&mut self, source: &mut LineSource<R>) {
        let temperature = self.create_temperature();
        self.judge.set_temperature(&temperature);
        let estimate = self.predict(&temperature, &mut *source);
        self.judge.answer(&estimate);
    }

    fn predict<R: BufRead>(&self, temperature: &Vec<Vec<usize>>, source: &mut LineSource<R>) -> Vec<usize> {
        let mut estimate = vec![0; self.N as usize];

        estimate
    }

}

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    
    input! {
        from &mut source,
        L: usize,
        N: usize,
        S: usize,
        landing_pos: [(usize, usize); N],
    }

    let landing_pos = landing_pos.into_iter().map(|(y, x)| Coord { y, x }).collect();

    let mut solver = Solver::new(L, N, S, landing_pos);
    solver.solve(&mut source);
}