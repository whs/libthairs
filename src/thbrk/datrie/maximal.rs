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
use crate::thbrk::datrie::{BreakInput, SetStorage};
/// Thai word break with maximal matching scheme
use crate::DatrieBrk;
use fst::{Automaton, IntoStreamer, Streamer};
use std::iter;
use std::str::from_utf8_unchecked;

/// maximal_do return character position to cut
pub(super) fn maximal_do(brk: &DatrieBrk, input: &BreakInput) -> Vec<usize> {
    // expand brkpos::hints to index of [u8]
    let brkpos_hints_expanded = brkpos::hints(&input.tis)
        .into_iter()
        .enumerate()
        .flat_map(|(index, can_cut)| {
            let ch = input.char[index];
            iter::once(can_cut).chain(iter::repeat(false).take(ch.len_utf8() - 1))
        })
        .collect::<Vec<bool>>();
    let mut hints: &[bool] = &brkpos_hints_expanded;

    let mut txt: &str = &input.str();
    debug_assert_eq!(brkpos_hints_expanded.len(), txt.len());
    let mut out = Vec::new();
    let mut pos = 0;
    let mut longest = Vec::with_capacity(txt.len());
    while txt.len() > 0 {
        let matcher = LongestSubstring::new(txt);
        let mut stream = match &brk.trie {
            SetStorage::Vec(t) => t.search(matcher).into_stream(),
            SetStorage::Mmap(t) => t.search(matcher).into_stream(),
        };

        unsafe {
            longest.set_len(0);
        }

        while let Some(item) = stream.next() {
            match hints.get(item.len()) {
                Some(false) => continue,
                _ => {}
            }
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

        let longest_txt = unsafe { from_utf8_unchecked(&longest) };
        let longest_char_len = longest_txt.chars().count();

        pos += longest_char_len;
        out.push(pos);
        txt = &txt[longest.len()..];
        hints = &hints[longest.len()..];
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

    #[inline]
    fn start(&self) -> Self::State {
        Some(0)
    }

    fn is_match(&self, state: &Self::State) -> bool {
        match *state {
            Some(v) if v <= self.str.len() => true,
            _ => false,
        }
    }

    #[inline]
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
