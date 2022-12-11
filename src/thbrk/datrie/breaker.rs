use crate::thbrk::data::*;
use crate::thbrk::datrie::maximal;
use crate::DatrieBrk;

const MAX_ACRONYM_FRAG_LEN: usize = 3;

pub fn find_breaks(brk: &DatrieBrk, input: &[u8], max_out: usize) -> Vec<usize> {
    if input.len() == 0 {
        return Vec::new();
    }

    let mut out: Vec<usize> = Vec::with_capacity(max_out);
    let mut chunk = 0;
    let mut acronym_end = 0;
    let mut prev_class = brk_class(input[0]);
    let mut effective_class = prev_class;

    let mut p = 1;

    while p < input.len() && out.len() < max_out {
        let mut new_class = brk_class(input[p]);

        if prev_class == BreakClass::Thai || prev_class == BreakClass::Alpha {
            // handle acronyms
            if input[p] == '.' as u8 && p - acronym_end <= MAX_ACRONYM_FRAG_LEN {
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
                    new_class = brk_class(input[p]);
                }
            }

            // break chunk if leaving Thai chunk
            if prev_class == BreakClass::Thai && new_class != BreakClass::Thai && p > chunk {
                let n_brk = maximal::maximal_do(brk, &input[chunk..p]);
                out.extend(n_brk.iter().map(|i| *i + chunk));

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
                if input[p] != '\n' as u8 || input[p - 1] != '\r' as u8 {
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
        let n_brk = maximal::maximal_do(brk, &input[chunk..p]);
        out.extend(n_brk.iter().map(|i| *i + chunk));

        // remove last break if at string end
        if out.last().copied() == Some(p) {
            out.pop();
        }
    }

    out
}
