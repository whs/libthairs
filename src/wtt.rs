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

//! WTT I/O implementation.
//!
//! WTT stands for Wing Tuk Tee (in Thai, Runs everywhere).
//! It was defined by TACTIS (Thai API Consortium/Thai Industrial Standard) in the NECTEC Thai Software Standard Project (1989-1991),
//! and later endorsed by Thai Industrial Standard Institute (TISI) as TIS 1566-2541 in 1998.

use ::libc;
pub type thchar_t = libc::c_uchar;

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WTTClass {
    /// control chars
    CTRL = 0,
    /// non composibles
    NON = 1,
    /// consonants
    CONS = 2,
    /// leading vowels
    LV = 3,
    /// following vowels 1
    FV1 = 4,
    /// following vowels 2
    FV2 = 5,
    /// following vowels 3
    FV3 = 6,
    /// below vowels 1
    BV1 = 7,
    /// below vowels 2
    BV2 = 8,
    /// below diacritics
    BD = 9,
    /// tonemarks
    TONE = 10,
    /// above diacritics 1
    AD1 = 11,
    /// above diacritics 2
    AD2 = 12,
    /// above diacritics 3
    AD3 = 13,
    /// above vowels 1
    AV1 = 14,
    /// above vowels 2
    AV2 = 15,
    /// above vowels 3
    AV3 = 16,
}

use WTTClass::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WTTOp {
    /// COMPOSIBLE - following char is displayed in the same cell as leading char, also implies ACCEPT
    CP = 1,
    /// Non-display
    XC = 2,
    /// ACCEPT - display the following char in the next cell
    AC = 3,
    /// REJECT - discard that following char, ignore it
    RJ = 4,
    /// STRICT REJECT - REJECT only if in strict mode
    SR = 5,
}

use WTTOp::*;

// XXX: These two arrays are exported as extern short in libthai, but unexported in libthairs

#[rustfmt::skip]
const TACchtype_: [WTTClass; 256] = [
    CTRL, CTRL, CTRL, CTRL, CTRL, CTRL, CTRL, CTRL, /*  0 -  7 */
    CTRL, CTRL, CTRL, CTRL, CTRL, CTRL, CTRL, CTRL, /*  8 - 15 */
    CTRL, CTRL, CTRL, CTRL, CTRL, CTRL, CTRL, CTRL, /* 16 - 23 */
    CTRL, CTRL, CTRL, CTRL, CTRL, CTRL, CTRL, CTRL, /* 24 - 31 */
    NON, NON, NON, NON, NON, NON, NON, NON, /* 32 - 39 */
    NON, NON, NON, NON, NON, NON, NON, NON, /* 40 - 47 */
    NON, NON, NON, NON, NON, NON, NON, NON, /* 48 - 55 */
    NON, NON, NON, NON, NON, NON, NON, NON, /* 56 - 63 */
    NON, NON, NON, NON, NON, NON, NON, NON, /* 64 - 71 */
    NON, NON, NON, NON, NON, NON, NON, NON, /* 72 - 79 */
    NON, NON, NON, NON, NON, NON, NON, NON, /* 80 - 87 */
    NON, NON, NON, NON, NON, NON, NON, NON, /* 88 - 95 */
    NON, NON, NON, NON, NON, NON, NON, NON, /* 96 - 103 */
    NON, NON, NON, NON, NON, NON, NON, NON, /* 104 - 111 */
    NON, NON, NON, NON, NON, NON, NON, NON, /* 112 - 119 */
    NON, NON, NON, NON, NON, NON, NON, CTRL, /* 120 - 127 */
    CTRL, CTRL, CTRL, CTRL, CTRL, CTRL, CTRL, CTRL, /* 128 - 135 */
    CTRL, CTRL, CTRL, CTRL, CTRL, CTRL, CTRL, CTRL, /* 136 - 143 */
    CTRL, CTRL, CTRL, CTRL, CTRL, CTRL, CTRL, CTRL, /* 144 - 151 */
    CTRL, CTRL, CTRL, CTRL, CTRL, CTRL, CTRL, CTRL, /* 152 - 159 */
    NON, CONS, CONS, CONS, CONS, CONS, CONS, CONS, /* 160 - 167 */
    CONS, CONS, CONS, CONS, CONS, CONS, CONS, CONS, /* 168 - 175 */
    CONS, CONS, CONS, CONS, CONS, CONS, CONS, CONS, /* 176 - 183 */
    CONS, CONS, CONS, CONS, CONS, CONS, CONS, CONS, /* 184 - 191 */
    CONS, CONS, CONS, CONS, FV3, CONS, FV3, CONS, /* 192 - 199 */
    CONS, CONS, CONS, CONS, CONS, CONS, CONS, NON, /* 200 - 207 */
    FV1, AV2, FV1, FV1, AV1, AV3, AV2, AV3, /* 208 - 215 */
    BV1, BV2, BD, NON, NON, NON, NON, NON, /* 216 - 223 */
    LV, LV, LV, LV, LV, FV2, NON, AD2, /* 224 - 231 */
    TONE, TONE, TONE, TONE, AD1, AD1, AD3, NON, /* 232 - 239 */
    NON, NON, NON, NON, NON, NON, NON, NON, /* 240 - 247 */
    NON, NON, NON, NON, NON, NON, NON, CTRL, /* 248 - 255 */
];

/// Table for Thai Cell Manipulation
#[rustfmt::skip]
const TACio_op_: [[WTTOp; 17]; 17] = [
    // Table 2: WTT I/O sequence check rules
    // row: leading char,  column: following char
  // CTRL NON CONS LV FV1 FV2 FV3 BV1 BV2 BD TONE AD1 AD2 AD3 AV1 AV2 AV3 */
     [XC, AC, AC, AC, AC, AC, AC, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ]/*CTRL*/
    ,[XC, AC, AC, AC, SR, SR, AC, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ]/*NON*/
    ,[XC, AC, AC, AC, AC, SR, AC, CP, CP, CP, CP, CP, CP, CP, CP, CP, CP]/*CONS*/
    ,[XC, SR, AC, SR, SR, SR, SR, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ]/*LV*/
    ,[XC, AC, AC, AC, AC, SR, AC, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ]/*FV1*/
    ,[XC, AC, AC, AC, AC, SR, AC, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ]/*FV2*/
    ,[XC, AC, AC, AC, SR, AC, SR, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ]/*FV3*/
    ,[XC, AC, AC, AC, AC, SR, AC, RJ, RJ, RJ, CP, CP, RJ, RJ, RJ, RJ, RJ]/*BV1*/
    ,[XC, AC, AC, AC, SR, SR, AC, RJ, RJ, RJ, CP, RJ, RJ, RJ, RJ, RJ, RJ]/*BV2*/
    ,[XC, AC, AC, AC, SR, SR, AC, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ]/*BD*/
    ,[XC, AC, AC, AC, AC, AC, AC, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ]/*TONE*/
    ,[XC, AC, AC, AC, SR, SR, AC, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ]/*AD1*/
    ,[XC, AC, AC, AC, SR, SR, AC, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ]/*AD2*/
    ,[XC, AC, AC, AC, SR, SR, AC, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ, RJ]/*AD3*/
    ,[XC, AC, AC, AC, SR, SR, AC, RJ, RJ, RJ, CP, CP, RJ, RJ, RJ, RJ, RJ]/*AV1*/
    ,[XC, AC, AC, AC, SR, SR, AC, RJ, RJ, RJ, CP, RJ, RJ, RJ, RJ, RJ, RJ]/*AV2*/
    ,[XC, AC, AC, AC, SR, SR, AC, RJ, RJ, RJ, CP, RJ, CP, RJ, RJ, RJ, RJ]/*AV3*/
];

/// WTT character class
#[no_mangle]
pub const extern "C" fn TACchtype(c: thchar_t) -> WTTClass {
    TACchtype_[c as usize] as WTTClass
}

/// WTT I/O operation
#[no_mangle]
pub const extern "C" fn TACio_op(c1: thchar_t, c2: thchar_t) -> WTTOp {
    TACio_op_[TACchtype(c1) as usize][TACchtype(c2) as usize]
}
