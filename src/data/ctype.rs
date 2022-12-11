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

/// Thai character classifications

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum CharClass {
    Control,
    Space,
    Lower,
    Upper,
    /// Thai consonant
    Consonant(ConsonantType),
    /// Thai Vowel
    Vowel(VowelType),
    /// Thai tone mark
    Tonemark,
    /// Thai diacritic
    Diacritic,
    Digit,
    Punctuation,
    Invalid,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum ConsonantType {
    Tailless,
    Overshoot,
    Undershoot,
    Undersplit,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum VowelType {
    Following,
    Leading,
    Upper,
    Below,
}

const CTYPE_TABLE: [CharClass; 256] = [
    /* 00 */ CharClass::Control,
    /* 01 */ CharClass::Control,
    /* 02 */ CharClass::Control,
    /* 03 */ CharClass::Control,
    /* 04 */ CharClass::Control,
    /* 05 */ CharClass::Control,
    /* 06 */ CharClass::Control,
    /* 07 */ CharClass::Control,
    /* 08 */ CharClass::Control,
    /* 09 */ CharClass::Space,
    /* 0a */ CharClass::Space,
    /* 0b */ CharClass::Control,
    /* 0c */ CharClass::Space,
    /* 0d */ CharClass::Space,
    /* 0e */ CharClass::Control,
    /* 0f */ CharClass::Control,
    /* 10 */ CharClass::Control,
    /* 11 */ CharClass::Control,
    /* 12 */ CharClass::Control,
    /* 13 */ CharClass::Control,
    /* 14 */ CharClass::Control,
    /* 15 */ CharClass::Control,
    /* 16 */ CharClass::Control,
    /* 17 */ CharClass::Control,
    /* 18 */ CharClass::Control,
    /* 19 */ CharClass::Control,
    /* 1a */ CharClass::Control,
    /* 1b */ CharClass::Control,
    /* 1c */ CharClass::Control,
    /* 1d */ CharClass::Control,
    /* 1e */ CharClass::Control,
    /* 1f */ CharClass::Control,
    /* 20   */ CharClass::Space,
    /* 21 ! */ CharClass::Punctuation,
    /* 22 " */ CharClass::Punctuation,
    /* 23 # */ CharClass::Punctuation,
    /* 24 $ */ CharClass::Punctuation,
    /* 25 % */ CharClass::Punctuation,
    /* 26 & */ CharClass::Punctuation,
    /* 27 ' */ CharClass::Punctuation,
    /* 28 ( */ CharClass::Punctuation,
    /* 29 ) */ CharClass::Punctuation,
    /* 2a * */ CharClass::Punctuation,
    /* 2b + */ CharClass::Punctuation,
    /* 2c , */ CharClass::Punctuation,
    /* 2d - */ CharClass::Punctuation,
    /* 2e . */ CharClass::Punctuation,
    /* 2f / */ CharClass::Punctuation,
    /* 30 0 */ CharClass::Digit,
    /* 31 1 */ CharClass::Digit,
    /* 32 2 */ CharClass::Digit,
    /* 33 3 */ CharClass::Digit,
    /* 34 4 */ CharClass::Digit,
    /* 35 5 */ CharClass::Digit,
    /* 36 6 */ CharClass::Digit,
    /* 37 7 */ CharClass::Digit,
    /* 38 8 */ CharClass::Digit,
    /* 39 9 */ CharClass::Digit,
    /* 3a : */ CharClass::Punctuation,
    /* 3b ; */ CharClass::Punctuation,
    /* 3c < */ CharClass::Punctuation,
    /* 3d = */ CharClass::Punctuation,
    /* 3e > */ CharClass::Punctuation,
    /* 3f ? */ CharClass::Punctuation,
    /* 40 @ */ CharClass::Punctuation,
    /* 41 A */ CharClass::Upper,
    /* 42 B */ CharClass::Upper,
    /* 43 C */ CharClass::Upper,
    /* 44 D */ CharClass::Upper,
    /* 45 E */ CharClass::Upper,
    /* 46 F */ CharClass::Upper,
    /* 47 G */ CharClass::Upper,
    /* 48 H */ CharClass::Upper,
    /* 49 I */ CharClass::Upper,
    /* 4a J */ CharClass::Upper,
    /* 4b K */ CharClass::Upper,
    /* 4c L */ CharClass::Upper,
    /* 4d M */ CharClass::Upper,
    /* 4e N */ CharClass::Upper,
    /* 4f O */ CharClass::Upper,
    /* 50 P */ CharClass::Upper,
    /* 51 Q */ CharClass::Upper,
    /* 52 R */ CharClass::Upper,
    /* 53 S */ CharClass::Upper,
    /* 54 T */ CharClass::Upper,
    /* 55 U */ CharClass::Upper,
    /* 56 V */ CharClass::Upper,
    /* 57 W */ CharClass::Upper,
    /* 58 X */ CharClass::Upper,
    /* 59 Y */ CharClass::Upper,
    /* 5a Z */ CharClass::Upper,
    /* 5b [ */ CharClass::Punctuation,
    /* 5c \ */ CharClass::Punctuation,
    /* 5d ] */ CharClass::Punctuation,
    /* 5e ^ */ CharClass::Punctuation,
    /* 5f _ */ CharClass::Punctuation,
    /* 60 ` */ CharClass::Punctuation,
    /* 61 a */ CharClass::Lower,
    /* 62 b */ CharClass::Lower,
    /* 63 c */ CharClass::Lower,
    /* 64 d */ CharClass::Lower,
    /* 65 e */ CharClass::Lower,
    /* 66 f */ CharClass::Lower,
    /* 67 g */ CharClass::Lower,
    /* 68 h */ CharClass::Lower,
    /* 69 i */ CharClass::Lower,
    /* 6a j */ CharClass::Lower,
    /* 6b k */ CharClass::Lower,
    /* 6c l */ CharClass::Lower,
    /* 6d m */ CharClass::Lower,
    /* 6e n */ CharClass::Lower,
    /* 6f o */ CharClass::Lower,
    /* 70 p */ CharClass::Lower,
    /* 71 q */ CharClass::Lower,
    /* 72 r */ CharClass::Lower,
    /* 73 s */ CharClass::Lower,
    /* 74 t */ CharClass::Lower,
    /* 75 u */ CharClass::Lower,
    /* 76 v */ CharClass::Lower,
    /* 77 w */ CharClass::Lower,
    /* 78 x */ CharClass::Lower,
    /* 79 y */ CharClass::Lower,
    /* 7a z */ CharClass::Lower,
    /* 7b { */ CharClass::Punctuation,
    /* 7c | */ CharClass::Punctuation,
    /* 7d } */ CharClass::Punctuation,
    /* 7e ~ */ CharClass::Punctuation,
    /* 7f */ CharClass::Control,
    /* 80 */ CharClass::Invalid,
    /* 81 */ CharClass::Invalid,
    /* 82 */ CharClass::Invalid,
    /* 83 */ CharClass::Invalid,
    /* 84 */ CharClass::Invalid,
    /* 85 */ CharClass::Invalid,
    /* 86 */ CharClass::Invalid,
    /* 87 */ CharClass::Invalid,
    /* 88 */ CharClass::Invalid,
    /* 89 */ CharClass::Invalid,
    /* 8a */ CharClass::Invalid,
    /* 8b */ CharClass::Invalid,
    /* 8c */ CharClass::Invalid,
    /* 8d */ CharClass::Invalid,
    /* 8e */ CharClass::Invalid,
    /* 8f */ CharClass::Invalid,
    /* 90 */ CharClass::Invalid,
    /* 91 */ CharClass::Invalid,
    /* 92 */ CharClass::Invalid,
    /* 93 */ CharClass::Invalid,
    /* 94 */ CharClass::Invalid,
    /* 95 */ CharClass::Invalid,
    /* 96 */ CharClass::Invalid,
    /* 97 */ CharClass::Invalid,
    /* 98 */ CharClass::Invalid,
    /* 99 */ CharClass::Invalid,
    /* 9a */ CharClass::Invalid,
    /* 9b */ CharClass::Invalid,
    /* 9c */ CharClass::Invalid,
    /* 9d */ CharClass::Invalid,
    /* 9e */ CharClass::Invalid,
    /* 9f */ CharClass::Invalid,
    /* a0 */ CharClass::Invalid,
    /* a1 ก */ CharClass::Consonant(ConsonantType::Tailless),
    /* a2 ข */ CharClass::Consonant(ConsonantType::Tailless),
    /* a3 ฃ */ CharClass::Consonant(ConsonantType::Tailless),
    /* a4 ค */ CharClass::Consonant(ConsonantType::Tailless),
    /* a5 ฅ */ CharClass::Consonant(ConsonantType::Tailless),
    /* a6 ฆ */ CharClass::Consonant(ConsonantType::Tailless),
    /* a7 ง */ CharClass::Consonant(ConsonantType::Tailless),
    /* a8 จ */ CharClass::Consonant(ConsonantType::Tailless),
    /* a9 ฉ */ CharClass::Consonant(ConsonantType::Tailless),
    /* aa ช */ CharClass::Consonant(ConsonantType::Tailless),
    /* ab ซ */ CharClass::Consonant(ConsonantType::Tailless),
    /* ac ฌ */ CharClass::Consonant(ConsonantType::Tailless),
    /* ad ญ */ CharClass::Consonant(ConsonantType::Undersplit),
    /* ae ฎ */ CharClass::Consonant(ConsonantType::Undershoot),
    /* af ฏ */ CharClass::Consonant(ConsonantType::Undershoot),
    /* b0 ฐ */ CharClass::Consonant(ConsonantType::Undersplit),
    /* b1 ฑ */ CharClass::Consonant(ConsonantType::Tailless),
    /* b2 ฒ */ CharClass::Consonant(ConsonantType::Tailless),
    /* b3 ณ */ CharClass::Consonant(ConsonantType::Tailless),
    /* b4 ด */ CharClass::Consonant(ConsonantType::Tailless),
    /* b5 ต */ CharClass::Consonant(ConsonantType::Tailless),
    /* b6 ถ */ CharClass::Consonant(ConsonantType::Tailless),
    /* b7 ท */ CharClass::Consonant(ConsonantType::Tailless),
    /* b8 ธ */ CharClass::Consonant(ConsonantType::Tailless),
    /* b9 น */ CharClass::Consonant(ConsonantType::Tailless),
    /* ba บ */ CharClass::Consonant(ConsonantType::Tailless),
    /* bb ป */ CharClass::Consonant(ConsonantType::Overshoot),
    /* bc ผ */ CharClass::Consonant(ConsonantType::Tailless),
    /* bd ฝ */ CharClass::Consonant(ConsonantType::Overshoot),
    /* be พ */ CharClass::Consonant(ConsonantType::Tailless),
    /* bf ฟ */ CharClass::Consonant(ConsonantType::Overshoot),
    /* c0 ภ */ CharClass::Consonant(ConsonantType::Tailless),
    /* c1 ม */ CharClass::Consonant(ConsonantType::Tailless),
    /* c2 ย */ CharClass::Consonant(ConsonantType::Tailless),
    /* c3 ร */ CharClass::Consonant(ConsonantType::Tailless),
    /* c4 ฤ */ CharClass::Vowel(VowelType::Following),
    /* c5 ล */ CharClass::Consonant(ConsonantType::Tailless),
    /* c6 ฦ */ CharClass::Vowel(VowelType::Following),
    /* c7 ว */ CharClass::Consonant(ConsonantType::Tailless),
    /* c8 ศ */ CharClass::Consonant(ConsonantType::Tailless),
    /* c9 ษ */ CharClass::Consonant(ConsonantType::Tailless),
    /* ca ส */ CharClass::Consonant(ConsonantType::Tailless),
    /* cb ห */ CharClass::Consonant(ConsonantType::Tailless),
    /* cc ฬ */ CharClass::Consonant(ConsonantType::Overshoot),
    /* cd อ */ CharClass::Consonant(ConsonantType::Tailless),
    /* ce ฮ */ CharClass::Consonant(ConsonantType::Tailless),
    /* cf ฯ */ CharClass::Punctuation,
    /* d0 ะ */ CharClass::Vowel(VowelType::Following),
    /* d1 ั */ CharClass::Vowel(VowelType::Upper),
    /* d2 า */ CharClass::Vowel(VowelType::Following),
    /* d3 ำ */ CharClass::Vowel(VowelType::Following),
    /* d4 ิ */ CharClass::Vowel(VowelType::Upper),
    /* d5 ี */ CharClass::Vowel(VowelType::Upper),
    /* d6 ึ */ CharClass::Vowel(VowelType::Upper),
    /* d7 ื */ CharClass::Vowel(VowelType::Upper),
    /* d8 ุ */ CharClass::Vowel(VowelType::Below),
    /* d9 ู */ CharClass::Vowel(VowelType::Below),
    /* da ฺ */ CharClass::Diacritic,
    /* db */ CharClass::Invalid,
    /* dc */ CharClass::Invalid,
    /* dd */ CharClass::Invalid,
    /* de */ CharClass::Invalid,
    /* df ฿ */ CharClass::Punctuation,
    /* e0 เ */ CharClass::Vowel(VowelType::Leading),
    /* e1 แ */ CharClass::Vowel(VowelType::Leading),
    /* e2 โ */ CharClass::Vowel(VowelType::Leading),
    /* e3 ใ */ CharClass::Vowel(VowelType::Leading),
    /* e4 ไ */ CharClass::Vowel(VowelType::Leading),
    /* e5 ๅ */ CharClass::Vowel(VowelType::Following),
    /* e6 ๆ */ CharClass::Punctuation,
    /* e7 ็ */ CharClass::Diacritic,
    /* e8 ่ */ CharClass::Tonemark,
    /* e9 ้ */ CharClass::Tonemark,
    /* ea ๊ */ CharClass::Tonemark,
    /* eb ๋ */ CharClass::Tonemark,
    /* ec ์ */ CharClass::Diacritic,
    /* ed ํ */ CharClass::Diacritic,
    /* ee ๎ */ CharClass::Diacritic,
    /* ef ๏ */ CharClass::Punctuation,
    /* f0 ๐ */ CharClass::Digit,
    /* f1 ๑ */ CharClass::Digit,
    /* f2 ๒ */ CharClass::Digit,
    /* f3 ๓ */ CharClass::Digit,
    /* f4 ๔ */ CharClass::Digit,
    /* f5 ๕ */ CharClass::Digit,
    /* f6 ๖ */ CharClass::Digit,
    /* f7 ๗ */ CharClass::Digit,
    /* f8 ๘ */ CharClass::Digit,
    /* f9 ๙ */ CharClass::Digit,
    /* fa ๚ */ CharClass::Punctuation,
    /* fb ๛ */ CharClass::Punctuation,
    /* fc */ CharClass::Invalid,
    /* fd */ CharClass::Invalid,
    /* fe */ CharClass::Invalid,
    /* ff */ CharClass::Invalid,
];

const CHARACTER_LEVEL_TABLE: [i8; 256] = [
    /* 00 */ 0, /* 01 */ 0, /* 02 */ 0, /* 03 */ 0, /* 04 */ 0,
    /* 05 */ 0, /* 06 */ 0, /* 07 */ 0, /* 08 */ 0, /* 09 */ 0,
    /* 0a */ 0, /* 0b */ 0, /* 0c */ 0, /* 0d */ 0, /* 0e */ 0,
    /* 0f */ 0, /* 10 */ 0, /* 11 */ 0, /* 12 */ 0, /* 13 */ 0,
    /* 14 */ 0, /* 15 */ 0, /* 16 */ 0, /* 17 */ 0, /* 18 */ 0,
    /* 19 */ 0, /* 1a */ 0, /* 1b */ 0, /* 1c */ 0, /* 1d */ 0,
    /* 1e */ 0, /* 1f */ 0, /* 20   */ 0, /* 21 ! */ 0, /* 22 " */ 0,
    /* 23 # */ 0, /* 24 $ */ 0, /* 25 % */ 0, /* 26 & */ 0, /* 27 ' */ 0,
    /* 28 ( */ 0, /* 29 ) */ 0, /* 2a * */ 0, /* 2b + */ 0, /* 2c , */ 0,
    /* 2d - */ 0, /* 2e . */ 0, /* 2f / */ 0, /* 30 0 */ 0, /* 31 1 */ 0,
    /* 32 2 */ 0, /* 33 3 */ 0, /* 34 4 */ 0, /* 35 5 */ 0, /* 36 6 */ 0,
    /* 37 7 */ 0, /* 38 8 */ 0, /* 39 9 */ 0, /* 3a : */ 0, /* 3b ; */ 0,
    /* 3c < */ 0, /* 3d = */ 0, /* 3e > */ 0, /* 3f ? */ 0, /* 40 @ */ 0,
    /* 41 A */ 0, /* 42 B */ 0, /* 43 C */ 0, /* 44 D */ 0, /* 45 E */ 0,
    /* 46 F */ 0, /* 47 G */ 0, /* 48 H */ 0, /* 49 I */ 0, /* 4a J */ 0,
    /* 4b K */ 0, /* 4c L */ 0, /* 4d M */ 0, /* 4e N */ 0, /* 4f O */ 0,
    /* 50 P */ 0, /* 51 Q */ 0, /* 52 R */ 0, /* 53 S */ 0, /* 54 T */ 0,
    /* 55 U */ 0, /* 56 V */ 0, /* 57 W */ 0, /* 58 X */ 0, /* 59 Y */ 0,
    /* 5a Z */ 0, /* 5b [ */ 0, /* 5c \ */ 0, /* 5d ] */ 0, /* 5e ^ */ 0,
    /* 5f _ */ 0, /* 60 ` */ 0, /* 61 a */ 0, /* 62 b */ 0, /* 63 c */ 0,
    /* 64 d */ 0, /* 65 e */ 0, /* 66 f */ 0, /* 67 g */ 0, /* 68 h */ 0,
    /* 69 i */ 0, /* 6a j */ 0, /* 6b k */ 0, /* 6c l */ 0, /* 6d m */ 0,
    /* 6e n */ 0, /* 6f o */ 0, /* 70 p */ 0, /* 71 q */ 0, /* 72 r */ 0,
    /* 73 s */ 0, /* 74 t */ 0, /* 75 u */ 0, /* 76 v */ 0, /* 77 w */ 0,
    /* 78 x */ 0, /* 79 y */ 0, /* 7a z */ 0, /* 7b { */ 0, /* 7c | */ 0,
    /* 7d } */ 0, /* 7e ~ */ 0, /* 7f */ 0, /* 80 */ 0, /* 81 */ 0,
    /* 82 */ 0, /* 83 */ 0, /* 84 */ 0, /* 85 */ 0, /* 86 */ 0,
    /* 87 */ 0, /* 88 */ 0, /* 89 */ 0, /* 8a */ 0, /* 8b */ 0,
    /* 8c */ 0, /* 8d */ 0, /* 8e */ 0, /* 8f */ 0, /* 90 */ 0,
    /* 91 */ 0, /* 92 */ 0, /* 93 */ 0, /* 94 */ 0, /* 95 */ 0,
    /* 96 */ 0, /* 97 */ 0, /* 98 */ 0, /* 99 */ 0, /* 9a */ 0,
    /* 9b */ 0, /* 9c */ 0, /* 9d */ 0, /* 9e */ 0, /* 9f */ 0,
    /* a0 */ 0, /* a1 ก */ 0, /* a2 ข */ 0, /* a3 ฃ */ 0,
    /* a4 ค */ 0, /* a5 ฅ */ 0, /* a6 ฆ */ 0, /* a7 ง */ 0,
    /* a8 จ */ 0, /* a9 ฉ */ 0, /* aa ช */ 0, /* ab ซ */ 0,
    /* ac ฌ */ 0, /* ad ญ */ 0, /* ae ฎ */ 0, /* af ฏ */ 0,
    /* b0 ฐ */ 0, /* b1 ฑ */ 0, /* b2 ฒ */ 0, /* b3 ณ */ 0,
    /* b4 ด */ 0, /* b5 ต */ 0, /* b6 ถ */ 0, /* b7 ท */ 0,
    /* b8 ธ */ 0, /* b9 น */ 0, /* ba บ */ 0, /* bb ป */ 0,
    /* bc ผ */ 0, /* bd ฝ */ 0, /* be พ */ 0, /* bf ฟ */ 0,
    /* c0 ภ */ 0, /* c1 ม */ 0, /* c2 ย */ 0, /* c3 ร */ 0,
    /* c4 ฤ */ 0, /* c5 ล */ 0, /* c6 ฦ */ 0, /* c7 ว */ 0,
    /* c8 ศ */ 0, /* c9 ษ */ 0, /* ca ส */ 0, /* cb ห */ 0,
    /* cc ฬ */ 0, /* cd อ */ 0, /* ce ฮ */ 0, /* cf ฯ */ 0,
    /* d0 ะ */ 0, /* d1 ั */ 1, /* d2 า */ 0, /* d3 ำ */ 0,
    /* d4 ิ */ 1, /* d5 ี */ 1, /* d6 ึ */ 1, /* d7 ื */ 1,
    /* d8 ุ */ -1, /* d9 ู */ -1, /* da ฺ */ -1, /* db */ 0,
    /* dc */ 0, /* dd */ 0, /* de */ 0, /* df ฿ */ 0, /* e0 เ */ 0,
    /* e1 แ */ 0, /* e2 โ */ 0, /* e3 ใ */ 0, /* e4 ไ */ 0,
    /* e5 ๅ */ 0, /* e6 ๆ */ 0, /* e7 ็ */ 3, /* e8 ่ */ 2,
    /* e9 ้ */ 2, /* ea ๊ */ 2, /* eb ๋ */ 2, /* ec ์ */ 2,
    /* ed ํ */ 3, /* ee ๎ */ 1, /* ef ๏ */ 0, /* f0 ๐ */ 0,
    /* f1 ๑ */ 0, /* f2 ๒ */ 0, /* f3 ๓ */ 0, /* f4 ๔ */ 0,
    /* f5 ๕ */ 0, /* f6 ๖ */ 0, /* f7 ๗ */ 0, /* f8 ๘ */ 0,
    /* f9 ๙ */ 0, /* fa ๚ */ 0, /* fb ๛ */ 0, /* fc */ 0,
    /* fd */ 0, /* fe */ 0, /* ff */ 0,
];

#[inline]
fn ctype(c: u8) -> CharClass {
    CTYPE_TABLE[c as usize]
}

/**
 * Is the character a valid TIS-620 code?
 *
 * TIS-620 here means US-ASCII plus TIS-620 extension. Character codes
 * in CR area (0x80-0x9f), non-breaking space (0xa0), code gap range
 * (0xdb-0xde and 0xfc-0xff) are excluded.
 */
#[inline]
pub fn is_tis(c: u8) -> bool {
    ctype(c) != CharClass::Invalid
}

/// Is the character a Thai character?
#[inline]
pub fn is_thai(c: u8) -> bool {
    is_tis(c) && (c & 0x80 != 0)
}

/// Is the character an English character?
#[inline]
pub fn is_eng(c: u8) -> bool {
    c & 0x80 == 0
}

/// Is the character a Thai consonant?
#[inline]
pub fn is_th_cons(c: u8) -> bool {
    match ctype(c) {
        CharClass::Consonant(_) => true,
        _ => false,
    }
}

/// Is the character a Thai vowel?
#[inline]
pub fn is_th_vowel(c: u8) -> bool {
    match ctype(c) {
        CharClass::Vowel(_) => true,
        _ => false,
    }
}

/// Is the character a Thai tone mark?
#[inline]
pub fn is_th_tone(c: u8) -> bool {
    ctype(c) == CharClass::Tonemark
}

/// Is the character a Thai diacritic?
#[inline]
pub fn is_th_diac(c: u8) -> bool {
    ctype(c) == CharClass::Diacritic
}

/// Is the character a Thai digit?
#[inline]
pub fn is_th_digit(c: u8) -> bool {
    ctype(c) == CharClass::Digit
}

/// Is the character a Thai punctuation?
#[inline]
pub fn is_th_punct(c: u8) -> bool {
    ctype(c) == CharClass::Punctuation
}

/// Is the character a Thai consonant that fits the x-height?
#[inline]
pub fn is_tailless_cons(c: u8) -> bool {
    ctype(c) == CharClass::Consonant(ConsonantType::Tailless)
}

/// Is the character a Thai consonant with stem above ascender?
#[inline]
pub fn is_overshoot_cons(c: u8) -> bool {
    ctype(c) == CharClass::Consonant(ConsonantType::Overshoot)
}

/// Is the character a Thai consonant with stem below baseline?
#[inline]
pub fn is_undershoot_cons(c: u8) -> bool {
    ctype(c) == CharClass::Consonant(ConsonantType::Undershoot)
}

/// Is the character a Thai consonant with split part below baseline?
#[inline]
pub fn is_undersplit_cons(c: u8) -> bool {
    ctype(c) == CharClass::Consonant(ConsonantType::Undersplit)
}

/// Is the character a Thai leading vowel?
#[inline]
pub fn is_leading_vowel(c: u8) -> bool {
    ctype(c) == CharClass::Vowel(VowelType::Leading)
}

/// Is the character a Thai following vowel?
#[inline]
pub fn is_following_vowel(c: u8) -> bool {
    ctype(c) == CharClass::Vowel(VowelType::Following)
}

/// Is the character a Thai upper vowel?
#[inline]
pub fn is_upper_vowel(c: u8) -> bool {
    ctype(c) == CharClass::Vowel(VowelType::Upper)
}

/// Is the character a Thai below vowel?
#[inline]
pub fn is_below_vowel(c: u8) -> bool {
    ctype(c) == CharClass::Vowel(VowelType::Below)
}

/**
 * Position for rendering
 *   - 3 = above/top
 *   - 2 = top
 *   - 1 = above
 *   - 0 = base
 *   - -1 = below
 */
#[inline]
pub fn chlevel(c: u8) -> i8 {
    CHARACTER_LEVEL_TABLE[c as usize]
}

/// Is the character a combining character?
#[inline]
pub fn is_comb_char(c: u8) -> bool {
    CHARACTER_LEVEL_TABLE[c as usize] != 0
}
