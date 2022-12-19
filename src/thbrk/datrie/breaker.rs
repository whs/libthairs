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

use crate::thbrk::data::*;
use crate::thbrk::datrie::maximal::MaximalBuffers;
use crate::thbrk::datrie::{maximal, BreakInput};
use crate::DatrieBrk;

const MAX_ACRONYM_FRAG_LEN: usize = 3;

pub(super) fn find_breaks(brk: &DatrieBrk, input: &BreakInput, max_out: usize) -> Vec<usize> {
    let tis_input = &input.tis;
    if tis_input.len() == 0 {
        return Vec::new();
    }

    let mut out: Vec<usize> = Vec::with_capacity(max_out);
    let mut chunk = 0;
    let mut acronym_end = 0;
    let mut prev_class = brk_class(tis_input[0]);
    let mut effective_class = prev_class;
    let mut maximal_buf = MaximalBuffers::default();

    let mut p = 1;

    while p < tis_input.len() && out.len() < max_out {
        let mut new_class = brk_class(tis_input[p]);

        if prev_class == BreakClass::Thai || prev_class == BreakClass::Alpha {
            // handle acronyms
            if tis_input[p] == '.' as u8 && p - acronym_end <= MAX_ACRONYM_FRAG_LEN {
                new_class = prev_class;
                acronym_end = p + 1;
            } else if acronym_end > chunk {
                // an acronym was marked
                if new_class != prev_class || p - acronym_end > MAX_ACRONYM_FRAG_LEN {
                    // end of Thai/Alpha chunk or entered non-acronym word,
                    // jump back to the acronym end
                    prev_class = brk_class('.' as u8);
                    effective_class = prev_class;

                    chunk = acronym_end;
                    p = acronym_end;
                    new_class = brk_class(tis_input[p]);
                }
            }

            // break chunk if leaving Thai chunk
            if prev_class == BreakClass::Thai && new_class != BreakClass::Thai && p > chunk {
                let n_brk = maximal::maximal_do(brk, &input.substring(chunk, p), &mut maximal_buf);
                out.extend(n_brk.into_iter().map(|i| i + chunk));

                // remove last break if at string end
                // note that even if it's allowed, the table-lookup
                // operation below will take care of it anyway
                if out.last().copied() == Some(p) {
                    out.pop();
                }

                if out.len() >= max_out {
                    break;
                }
            }
        }

        // reset chunk on switching
        if new_class != prev_class {
            chunk = p;
            acronym_end = p;
        }

        let op = brk_op(effective_class, new_class);

        match op {
            BreakOperation::Allowed => {
                if tis_input[p] != '\n' as u8 || tis_input[p - 1] != '\r' as u8 {
                    out.push(p);
                }
            }
            BreakOperation::Indirect => {
                if prev_class == BreakClass::Space {
                    out.push(p);
                }
            }
            _ => {}
        }

        prev_class = new_class;
        if op == BreakOperation::Allowed || new_class != BreakClass::Space {
            effective_class = new_class;
        }

        p += 1;
    }

    // break last Thai non-acronym chunk
    if prev_class == BreakClass::Thai && acronym_end <= chunk && out.len() < max_out {
        let n_brk = maximal::maximal_do(brk, &input.substring(chunk, p), &mut maximal_buf);
        out.extend(n_brk.into_iter().map(|i| i + chunk));

        // remove last break if at string end
        if out.last().copied() == Some(p) {
            out.pop();
        }
    }

    out
}
