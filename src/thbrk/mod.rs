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

mod brkpos;
mod c_tis_api;
mod c_w_api;
mod data;
mod datrie;
#[cfg(test)]
mod test;

pub use self::datrie::DatrieBrk;
use encoding_rs::WINDOWS_874;
use std::str::from_utf8_unchecked;

/// TisBreaker implement Thai word breaking algorithm with TIS-620 input
pub trait TisBreaker {
    fn find_breaks_tis<'a>(&'a self, input: &'a [u8], max_out: usize) -> Vec<usize>;

    fn split_tis<'a>(&'a self, input: &'a [u8]) -> Vec<&[u8]> {
        let breaks = self.find_breaks_tis(input, input.len());
        let mut out = Vec::new();

        let mut last_break = 0;
        for brk in breaks {
            out.push(&input[last_break..brk]);
            last_break = brk;
        }
        let remainder = &input[last_break..];
        if remainder.len() > 0 {
            out.push(remainder);
        }

        out
    }
}

/// StrBreaker implement Thai word breaking algorithm with UTF-8 input
pub trait StrBreaker {
    /// The return value is character index and not byte index
    fn find_breaks<'a>(&'a self, input: &'a str, max_out: usize) -> Vec<usize>;

    fn split<'a>(&'a self, input: &'a str) -> Vec<&'a str> {
        let mut breaks = self.find_breaks(input, input.len()).into_iter();

        let mut cur_break = match breaks.next() {
            Some(v) => v,
            None => return vec![input],
        };
        let mut out: Vec<&str> = Vec::new();
        let mut last_ch_pos = 0;
        for (ch_pos, (byte_pos, _)) in input.char_indices().enumerate() {
            if ch_pos == cur_break {
                out.push(&input[last_ch_pos..byte_pos]);
                last_ch_pos = byte_pos;
                cur_break = match breaks.next() {
                    Some(v) => v,
                    None => break,
                }
            }
        }
        out.push(&input[last_ch_pos..]);

        out
    }
}

impl TisBreaker for dyn StrBreaker {
    fn find_breaks_tis<'a>(&'a self, input: &'a [u8], max_out: usize) -> Vec<usize> {
        let (input_utf, _) = WINDOWS_874.decode_without_bom_handling(input);
        self.find_breaks(&input_utf, max_out)
    }
}
