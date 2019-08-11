use fnv::FnvHashMap;
use lazy_static::lazy_static;

use mancala_rust::{compact_key, learn::*, Board, Evaluator, NN6Evaluator};

const STEALING: bool = true;

lazy_static! {
    static ref SCORE_MAP: FnvHashMap<u64, i8> = just_load(STEALING);
}

#[derive(Copy, Clone, PartialOrd, PartialEq)]
struct F(f64);

impl Eq for F {}

impl std::cmp::Ord for F {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn search(board: &Board, eval: &mut NN6Evaluator, depth: u8, alpha: f64, beta: f64) -> (f64, bool) {
    let key = compact_key(&board);
    if let Some(s) = SCORE_MAP.get(&key) {
        return (f64::from(board.score() + *s), true);
    }
    if board.is_finished() {
        return (f64::from(board.last_score()), true);
    }
    if depth == 0 {
        return (eval.eval(&board), false);
    }
    let mut best_score = alpha;
    let mut full = true;
    for next in board.list_next() {
        let (s, f) = search(&next, eval, depth - 1, -beta, -best_score);
        best_score = if best_score > -s { best_score } else { -s };
        full = full && f;
        if best_score >= beta {
            break;
        }
    }

    (best_score, full)
}

fn make(
    data: &mut FnvHashMap<u64, (f64, bool)>,
    board: &Board,
    eval: &mut NN6Evaluator,
    depth: u8,
) -> (f64, bool) {
    let key = compact_key(&board);
    if depth == 0 {
        println!("{}", board);
        let (score, full) = search(board, eval, 20, -10000.0, 10000.0);
        data.insert(key, (-score, full));
        println!("{} {}", -score, full);
        return (-score, full);
    }
    let mut best_score = std::f64::MIN;
    let mut full = true;
    let mut next_list = board.list_next().drain().collect::<Vec<_>>();
    next_list.sort_by_key(|b| F(-eval.eval(b)));
    for next in next_list {
        let (s, f) = make(data, &next, eval, depth - 1);
        best_score = if best_score > -s { best_score } else { -s };
        full = full && f;
    }
    data.insert(key, (best_score, full));
    (best_score, full)
}

fn main() {
    let board = Board::new(STEALING);
    let mut eval = NN6Evaluator::new(STEALING);
    let mut data = FnvHashMap::default();
    make(&mut data, &board, &mut eval, 6);
}
