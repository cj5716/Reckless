use crate::{thread::ThreadData, types::Score};

pub fn correct_eval(td: &ThreadData, raw_eval: i32, correction_value: i32) -> i32 {
    let mut eval = ((raw_eval as i64 * (21061 + td.board.material()) as i64
        + (td.optimism[td.board.side_to_move()] * (1519 + td.board.material())) as i64)
        / 26556) as i32;

    eval = eval * (200 - td.board.halfmove_clock() as i32) / 200;

    eval += correction_value;

    eval.clamp(-Score::TB_WIN_IN_MAX + 1, Score::TB_WIN_IN_MAX - 1)
}
