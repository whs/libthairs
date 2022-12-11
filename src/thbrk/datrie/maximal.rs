////////////////////////////////////////////////////////////////////////////////
// Copyright (C) 2022 Manatsawin Hanmongkolchai
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
////////////////////////////////////////////////////////////////////////////////

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
