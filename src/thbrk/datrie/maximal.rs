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
use crate::{utils, DatrieBrk};
use fst::{Automaton, IntoStreamer, Streamer};
use std::str::from_utf8_unchecked;

#[derive(Default)]
pub(super) struct MaximalBuffers {
    brkpos: Vec<bool>,
    brkpos_expanded: Vec<bool>,
    input: Vec<u8>,
    out: Vec<usize>,
    longest_str: Vec<u8>,
}

impl MaximalBuffers {
    fn reset_with_min_cap(&mut self, len: usize) {
        self.out.clear();

        let brkpos_cap = len.checked_sub(self.brkpos.capacity());
        match brkpos_cap {
            Some(l) => self.brkpos.reserve_exact(l),
            None => {}
        }

        let expanded_cap = (len * 4).checked_sub(self.brkpos_expanded.capacity());
        match expanded_cap {
            Some(l) => self.brkpos_expanded.reserve_exact(l),
            None => {}
        }

        let longest_str_cap = len.checked_sub(self.longest_str.capacity());
        match longest_str_cap {
            Some(l) => self.longest_str.reserve_exact(l),
            None => {}
        }

        let input_cap = len.checked_sub(self.input.capacity());
        match input_cap {
            Some(l) => self.input.reserve_exact(l),
            None => {}
        }
    }
}

/// maximal_do return character position to cut
pub(super) fn maximal_do<'a>(
    brk: &DatrieBrk,
    input: &BreakInput,
    buf: &'a mut MaximalBuffers,
) -> &'a [usize] {
    // TODO: There's a problem with non-dictionary matchable that c libthai doesn't have..
    buf.reset_with_min_cap(input.char.len());

    utils::chars_as_bytes(&input.char, &mut buf.input);
    let mut txt: &[u8] = &buf.input;

    brkpos::hints(&input.tis, &mut buf.brkpos);
    brkpos::expand_hint_bytes(
        &input.char,
        txt.len(),
        &buf.brkpos,
        &mut buf.brkpos_expanded,
    );
    let mut hints: &[bool] = &buf.brkpos_expanded;
    debug_assert_eq!(hints.len(), txt.len());

    let mut out = &mut buf.out;
    let mut pos = 0;
    let mut longest = &mut buf.longest_str;
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
            if item.len() < hints.len() && !hints[item.len()] {
                continue;
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

    &buf.out
}

#[derive(Debug, Clone)]
struct LongestSubstring<'a> {
    str: &'a [u8],
}

impl<'a> LongestSubstring<'a> {
    #[inline]
    fn new(str: &'a [u8]) -> Self {
        Self { str }
    }
}

impl<'a> Automaton for LongestSubstring<'a> {
    type State = Option<usize>;

    #[inline]
    fn start(&self) -> Self::State {
        Some(0)
    }

    #[inline]
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
