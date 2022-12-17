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
use crate::thbrk::datrie::BreakInput;
/// Thai word break with maximal matching scheme
use crate::DatrieBrk;
use fst::automaton::Str;
use fst::{Automaton, IntoStreamer, Streamer};
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::str::from_utf8;

#[derive(Default, Clone)]
struct Shot {
    dict_state: i32, // TODO
    str_pos: usize,
    brk_pos: Vec<usize>,
    penalty: i32,
}

impl Eq for Shot {}

impl PartialEq<Self> for Shot {
    fn eq(&self, other: &Self) -> bool {
        self.str_pos == other.str_pos
            && self.penalty == other.penalty
            && self.brk_pos == other.brk_pos
    }
}

impl PartialOrd<Self> for Shot {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.brk_pos.len() == 0 {
            if other.brk_pos.len() == 0 {
                return Some(Ordering::Equal);
            }
            return Some(Ordering::Less);
        }
        if other.brk_pos.len() == 0 {
            return Some(Ordering::Greater);
        }

        let my_last = self.brk_pos.last().unwrap();
        let their_last = other.brk_pos.last().unwrap();
        return if my_last == their_last {
            Some(Ordering::Equal)
        } else if my_last < their_last {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        };
    }
}

impl Ord for Shot {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Default, Clone)]
struct BestBrk {
    brk_pos: Vec<usize>,
    str_pos: usize,
    penalty: i32,
}

#[derive(Default, Copy, Clone)]
struct RecovHist {
    pos: Option<usize>,
    recov: Option<i32>,
}

pub(super) fn maximal_do(brk: &DatrieBrk, input: &BreakInput) -> Vec<usize> {
    let brkpos_hints = brkpos::hints(&input.tis);

    let matcher = Str::new(&input.utf);
    let mut stream = brk.trie.search_with_state(matcher).into_stream();

    while let Some((item, state)) = stream.next() {
        println!("input {} matched {}", input.utf, from_utf8(item).unwrap());
    }
    println!("input {} done", input.utf);

    Vec::new()
}
