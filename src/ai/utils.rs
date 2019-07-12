use crate::board::{Board, Evaluation};

pub fn ab_search<E: Evaluation>(
    board: Board,
    eval: &E,
    depth: usize,
    alpha: i32,
    beta: i32,
) -> i32 {
    if depth == 0 || board.is_finished() {
        return eval.eval(&board);
    }
    let mut alpha = alpha;
    for next in board.list_next() {
        let a = -ab_search(next, eval, depth - 1, -beta, -alpha);
        if a > alpha {
            alpha = a;
        }
        if alpha >= beta {
            break;
        }
    }
    alpha
}
