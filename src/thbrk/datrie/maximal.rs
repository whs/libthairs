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

use crate::thbrk::datrie::BreakInput;
/// Thai word break with maximal matching scheme
use crate::DatrieBrk;
use fst::{Automaton, IntoStreamer, Streamer};
use std::str::from_utf8;

/// maximal_do return character position to cut
pub(super) fn maximal_do(brk: &DatrieBrk, input: &BreakInput) -> Vec<usize> {
    // brkpos_hint is not used because this algorithm operate on &[u8] and it would be costly
    // to convert it to character positions

    let mut txt: &str = &input.utf;
    let mut out = Vec::new();
    let mut pos_char = 0;
    let mut longest = Vec::with_capacity(txt.len());
    while txt.len() > 0 {
        let matcher = LongestSubstring::new(txt);
        let mut stream = brk.trie.search(matcher).into_stream();

        unsafe {
            longest.set_len(0);
        }

        while let Some(item) = stream.next() {
            if item.len() > longest.len() {
                unsafe {
                    longest.set_len(item.len());
                }
                longest.copy_from_slice(item);
            }
        }

        if longest.len() == 0 {
            break;
        }

        pos_char += from_utf8(&longest).unwrap().chars().count();
        out.push(pos_char);
        txt = &txt[longest.len()..];
    }

    out
}

#[derive(Debug, Clone)]
struct LongestSubstring<'a> {
    str: &'a [u8],
}

impl<'a> LongestSubstring<'a> {
    fn new(str: &'a str) -> Self {
        Self {
            str: str.as_bytes(),
        }
    }
}

impl<'a> Automaton for LongestSubstring<'a> {
    type State = Option<usize>;

    fn start(&self) -> Self::State {
        Some(0)
    }

    fn is_match(&self, state: &Self::State) -> bool {
        match *state {
            Some(v) if v <= self.str.len() => true,
            _ => false,
        }
    }

    fn can_match(&self, pos: &Option<usize>) -> bool {
        pos.is_some()
    }

    fn accept(&self, pos: &Self::State, byte: u8) -> Self::State {
        // if we aren't already past the end...
        if let Some(pos) = *pos {
            // and there is still a matching byte at the current position...
            if self.str.get(pos).cloned() == Some(byte) {
                // then move forward
                return Some(pos + 1);
            }
        }
        // otherwise we're either past the end or didn't match the byte
        None
    }
}
