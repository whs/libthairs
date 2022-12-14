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

use crate::data;

pub fn hints(input: &[u8], out: &mut Vec<bool>) {
    out.clear();
    out.resize(input.len(), false);

    let mut i = 0;
    while i < input.len() {
        if data::is_th_cons(input[i]) {
            if i + 1 < input.len() && input[i] == data::TIS_ทัณฑฆาต {
                i += 2 /* the cons + ทัณฑฆาต */
            } else if i + 2 < input.len() && input[i] == data::TIS_ทัณฑฆาต {
                i += 3; /* the cons + intermediate char + ทัณฑฆาต */
            } else if i + 2 < input.len()
                && input[i] != data::TIS_ก
                && input[i + 1] == data::TIS_ไม้ไต่คู้
                && (input[i + 2] == data::TIS_อ || input[i + 2] == data::TIS_ว)
            {
                out[i] = true;
                i += 4; /* the cons + ไม้ไต่คู้ + อ/ว + cons */
            } else if (i > 0
                && (input[i - 1] == data::TIS_ไม้หันอากาศ
                    || input[i - 1] == data::TIS_สระอือ))
                || (i > 1
                    && data::is_th_tone(input[i - 1])
                    && (input[i - 2] == data::TIS_ไม้หันอากาศ
                        || input[i - 2] == data::TIS_สระอือ))
            {
                i += 1;
            } else {
                out[i] = true;
                i += 1;
            }
        } else if input[i] == data::TIS_เ || input[i] == data::TIS_แ {
            out[i] = true; /* สระเอ / สระแอ */
            i += 2; /* สระเอ / สระแอ + the supposedly cons */
            if i >= input.len() {
                break;
            }
            if input[i] == data::TIS_ไม้ไต่คู้ {
                i += 2; /* ไม้ไต่คู้ + the supposedly cons */
            } else if data::is_upper_vowel(input[i]) {
                i += 1; /* the upper vowel, as part of composite vowel */
                if i < input.len() && data::is_th_tone(input[i]) {
                    i += 1;
                }
                i += 1; /* the supposedly cons */
            } else if i + 2 < input.len()
                && ((input[i + 1] == data::TIS_า && input[i + 2] == data::TIS_ะ)
                    || (input[i] != data::TIS_ก
                        && input[i + 1] == data::TIS_ไม้ไต่คู้
                        && input[i + 2] != data::TIS_อ
                        && input[i + 2] != data::TIS_ว))
            {
                i += 3; /* 2nd cons + สระอา + สระอะ, or
                         * 2nd cons + ไม้ไต่คู้ + final cons
                         */
            }
        } else if data::is_leading_vowel(input[i]) {
            out[i] = true; // the ldvowel
            i += 2; /* the ldvowel + the supposedly cons */
        } else if input[i] == data::TIS_ฤ || input[i] == data::TIS_ฦ {
            out[i] = true;
            i += 1;
        } else {
            i += 1;
        }
    }
}

/// expand output of char-index bitmap to bytes-indexed bitmap, with internal bits set as false
pub fn expand_hint_bytes(s: &[char], s_len_bytes: usize, input: &[bool], out: &mut Vec<bool>) {
    out.clear();
    out.resize(s_len_bytes, false);

    let mut bytes_idx = 0;
    for (ch_idx, ch) in s.iter().enumerate() {
        out[bytes_idx] = input[ch_idx];
        bytes_idx += ch.len_utf8();
    }
}
