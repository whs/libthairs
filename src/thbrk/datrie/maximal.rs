use crate::thbrk::brkpos;
/// Thai word break with maximal matching scheme
use crate::DatrieBrk;

struct Shot {
    dict_state: i32, // TODO
    str_pos: i32,
    brk_pos: i32,
    n_brk_pos: i32,
    cur_brk_pos: i32,
    penalty: i32,
}

struct Pool {
    // next: *Pool,
    shot: Shot,
}

pub fn maximal_do(brk: &DatrieBrk, input: &[u8]) -> Vec<usize> {
    let brkpos_hints = brkpos::hints(input);
    Vec::new()
}
