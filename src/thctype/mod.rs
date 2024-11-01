mod wtt;

pub type thchar_t = u8;

pub trait ThaiCharacter {
    fn is_tis(&self) -> bool;
    fn is_thai(&self) -> bool;
    fn is_eng(&self) -> bool;
    fn is_thcons(&self) -> bool;
    fn is_thvowel(&self) -> bool;
    fn is_thtone(&self) -> bool;
    fn is_thdiac(&self) -> bool;
    fn is_thdigit(&self) -> bool;
    fn is_thpunct(&self) -> bool;
    fn is_taillesscons(&self) -> bool;
    fn is_overshootcons(&self) -> bool;
    fn is_undershootcons(&self) -> bool;
    fn is_undersplitcons(&self) -> bool;
    fn is_ldvowel(&self) -> bool;
    fn is_flvowel(&self) -> bool;
    fn is_upvowel(&self) -> bool;
    fn is_blvowel(&self) -> bool;
    fn chlevel(&self) -> i8;
    fn is_combchar(&self) -> bool;
}

impl ThaiCharacter for thchar_t {
    fn is_tis(&self) -> bool {
        th_istis(*self)
    }

    fn is_thai(&self) -> bool {
        th_isthai(*self)
    }

    fn is_eng(&self) -> bool {
        th_iseng(*self)
    }

    fn is_thcons(&self) -> bool {
        th_isthcons(*self)
    }

    fn is_thvowel(&self) -> bool {
        th_isthvowel(*self)
    }

    fn is_thtone(&self) -> bool {
        th_isthtone(*self)
    }

    fn is_thdiac(&self) -> bool {
        th_isthdiac(*self)
    }

    fn is_thdigit(&self) -> bool {
        th_isthdigit(*self)
    }

    fn is_thpunct(&self) -> bool {
        th_isthpunct(*self)
    }

    fn is_taillesscons(&self) -> bool {
        th_istaillesscons(*self)
    }

    fn is_overshootcons(&self) -> bool {
        th_isovershootcons(*self)
    }

    fn is_undershootcons(&self) -> bool {
        th_isundershootcons(*self)
    }

    fn is_undersplitcons(&self) -> bool {
        th_isundersplitcons(*self)
    }

    fn is_ldvowel(&self) -> bool {
        th_isldvowel(*self)
    }

    fn is_flvowel(&self) -> bool {
        th_isflvowel(*self)
    }

    fn is_upvowel(&self) -> bool {
        th_isupvowel(*self)
    }

    fn is_blvowel(&self) -> bool {
        th_isblvowel(*self)
    }

    fn chlevel(&self) -> i8 {
        th_chlevel(*self)
    }

    fn is_combchar(&self) -> bool {
        th_iscombchar(*self)
    }
}

const fn isbit(bit: usize) -> u16 {
    1 << bit
}

const fn bitfield(base: u16, val: u16) -> u16 {
    val << base
}

const fn bitmask(base: u16, bits: u16) -> u16 {
    !((!0u16) << (bits)) << (base)
}

/// TIS-620 char
const _th_IStis: u16 = isbit(0);

/// Thai consonant
const _th_IScons: u16 = isbit(1);
/// Thai consonant shape masks
const _th_CClassMsk: u16 = bitmask(1, 3);
/// tailless cons
const _th_CCtailless: u16 = bitfield(2, 0) | _th_IScons;
/// overshoot cons
const _th_CCovershoot: u16 = bitfield(2, 1) | _th_IScons;
/// undershoot cons
const _th_CCundershoot: u16 = bitfield(2, 2) | _th_IScons;
/// undersplit cons
const _th_CCundersplit: u16 = bitfield(2, 3) | _th_IScons;
/// Thai vowel
const _th_ISvowel: u16 = isbit(4);
/// Thai vowel class masks
const _th_VClassMsk: u16 = bitmask(4, 3);
/// Thai following vowel
const _th_VCflvowel: u16 = bitfield(5, 0) | _th_ISvowel;
/// Thai leading vowel
const _th_VCldvowel: u16 = bitfield(5, 1) | _th_ISvowel;
/// Thai upper vowel
const _th_VCupvowel: u16 = bitfield(5, 2) | _th_ISvowel;
/// Thai below vowel
const _th_VCblvowel: u16 = bitfield(5, 3) | _th_ISvowel;
/// Thai tone mark
const _th_IStone: u16 = isbit(7);
/// Thai diacritic
const _th_ISdiac: u16 = isbit(8);
/// digit
const _th_ISdigit: u16 = isbit(9);
/// punctuation
const _th_ISpunct: u16 = isbit(10);

const _none: u16 = 0;
const _cntrl: u16 = _th_IStis;
const _space: u16 = _th_IStis;
const _edigit: u16 = _th_IStis | _th_ISdigit;
const _elower: u16 = _th_IStis;
const _eupper: u16 = _th_IStis;
const _epunct: u16 = _th_IStis | _th_ISpunct;
const _tdigit: u16 = _th_IStis | _th_ISdigit;
const _tcons: u16 = _th_IStis | _th_IScons;
const _tflvowel: u16 = _th_IStis | _th_VCflvowel;
const _tldvowel: u16 = _th_IStis | _th_VCldvowel;
const _tupvowel: u16 = _th_IStis | _th_VCupvowel;
const _tblvowel: u16 = _th_IStis | _th_VCblvowel;
const _ttone: u16 = _th_IStis | _th_IStone;
const _tdiac: u16 = _th_IStis | _th_ISdiac;
const _tpunct: u16 = _th_IStis | _th_ISpunct;

const _th_ctype_tbl: [u16; 256] = [
    /* 00 */ _cntrl,
    /* 01 */ _cntrl,
    /* 02 */ _cntrl,
    /* 03 */ _cntrl,
    /* 04 */ _cntrl,
    /* 05 */ _cntrl,
    /* 06 */ _cntrl,
    /* 07 */ _cntrl,
    /* 08 */ _cntrl,
    /* 09 */ _space,
    /* 0a */ _space,
    /* 0b */ _cntrl,
    /* 0c */ _space,
    /* 0d */ _space,
    /* 0e */ _cntrl,
    /* 0f */ _cntrl,
    /* 10 */ _cntrl,
    /* 11 */ _cntrl,
    /* 12 */ _cntrl,
    /* 13 */ _cntrl,
    /* 14 */ _cntrl,
    /* 15 */ _cntrl,
    /* 16 */ _cntrl,
    /* 17 */ _cntrl,
    /* 18 */ _cntrl,
    /* 19 */ _cntrl,
    /* 1a */ _cntrl,
    /* 1b */ _cntrl,
    /* 1c */ _cntrl,
    /* 1d */ _cntrl,
    /* 1e */ _cntrl,
    /* 1f */ _cntrl,
    /* 20   */ _space,
    /* 21 ! */ _epunct,
    /* 22 " */ _epunct,
    /* 23 # */ _epunct,
    /* 24 $ */ _epunct,
    /* 25 % */ _epunct,
    /* 26 & */ _epunct,
    /* 27 ' */ _epunct,
    /* 28 ( */ _epunct,
    /* 29 ) */ _epunct,
    /* 2a * */ _epunct,
    /* 2b + */ _epunct,
    /* 2c , */ _epunct,
    /* 2d - */ _epunct,
    /* 2e . */ _epunct,
    /* 2f / */ _epunct,
    /* 30 0 */ _edigit,
    /* 31 1 */ _edigit,
    /* 32 2 */ _edigit,
    /* 33 3 */ _edigit,
    /* 34 4 */ _edigit,
    /* 35 5 */ _edigit,
    /* 36 6 */ _edigit,
    /* 37 7 */ _edigit,
    /* 38 8 */ _edigit,
    /* 39 9 */ _edigit,
    /* 3a : */ _epunct,
    /* 3b ; */ _epunct,
    /* 3c < */ _epunct,
    /* 3d = */ _epunct,
    /* 3e > */ _epunct,
    /* 3f ? */ _epunct,
    /* 40 @ */ _epunct,
    /* 41 A */ _eupper,
    /* 42 B */ _eupper,
    /* 43 C */ _eupper,
    /* 44 D */ _eupper,
    /* 45 E */ _eupper,
    /* 46 F */ _eupper,
    /* 47 G */ _eupper,
    /* 48 H */ _eupper,
    /* 49 I */ _eupper,
    /* 4a J */ _eupper,
    /* 4b K */ _eupper,
    /* 4c L */ _eupper,
    /* 4d M */ _eupper,
    /* 4e N */ _eupper,
    /* 4f O */ _eupper,
    /* 50 P */ _eupper,
    /* 51 Q */ _eupper,
    /* 52 R */ _eupper,
    /* 53 S */ _eupper,
    /* 54 T */ _eupper,
    /* 55 U */ _eupper,
    /* 56 V */ _eupper,
    /* 57 W */ _eupper,
    /* 58 X */ _eupper,
    /* 59 Y */ _eupper,
    /* 5a Z */ _eupper,
    /* 5b [ */ _epunct,
    /* 5c \ */ _epunct,
    /* 5d ] */ _epunct,
    /* 5e ^ */ _epunct,
    /* 5f _ */ _epunct,
    /* 60 ` */ _epunct,
    /* 61 a */ _elower,
    /* 62 b */ _elower,
    /* 63 c */ _elower,
    /* 64 d */ _elower,
    /* 65 e */ _elower,
    /* 66 f */ _elower,
    /* 67 g */ _elower,
    /* 68 h */ _elower,
    /* 69 i */ _elower,
    /* 6a j */ _elower,
    /* 6b k */ _elower,
    /* 6c l */ _elower,
    /* 6d m */ _elower,
    /* 6e n */ _elower,
    /* 6f o */ _elower,
    /* 70 p */ _elower,
    /* 71 q */ _elower,
    /* 72 r */ _elower,
    /* 73 s */ _elower,
    /* 74 t */ _elower,
    /* 75 u */ _elower,
    /* 76 v */ _elower,
    /* 77 w */ _elower,
    /* 78 x */ _elower,
    /* 79 y */ _elower,
    /* 7a z */ _elower,
    /* 7b { */ _epunct,
    /* 7c | */ _epunct,
    /* 7d } */ _epunct,
    /* 7e ~ */ _epunct,
    /* 7f */ _cntrl,
    /* 80 */ _none,
    /* 81 */ _none,
    /* 82 */ _none,
    /* 83 */ _none,
    /* 84 */ _none,
    /* 85 */ _none,
    /* 86 */ _none,
    /* 87 */ _none,
    /* 88 */ _none,
    /* 89 */ _none,
    /* 8a */ _none,
    /* 8b */ _none,
    /* 8c */ _none,
    /* 8d */ _none,
    /* 8e */ _none,
    /* 8f */ _none,
    /* 90 */ _none,
    /* 91 */ _none,
    /* 92 */ _none,
    /* 93 */ _none,
    /* 94 */ _none,
    /* 95 */ _none,
    /* 96 */ _none,
    /* 97 */ _none,
    /* 98 */ _none,
    /* 99 */ _none,
    /* 9a */ _none,
    /* 9b */ _none,
    /* 9c */ _none,
    /* 9d */ _none,
    /* 9e */ _none,
    /* 9f */ _none,
    /* a0 */ _none,
    /* a1 ก */ _tcons,
    /* a2 ข */ _tcons,
    /* a3 ฃ */ _tcons,
    /* a4 ค */ _tcons,
    /* a5 ฅ */ _tcons,
    /* a6 ฆ */ _tcons,
    /* a7 ง */ _tcons,
    /* a8 จ */ _tcons,
    /* a9 ฉ */ _tcons,
    /* aa ช */ _tcons,
    /* ab ซ */ _tcons,
    /* ac ฌ */ _tcons,
    /* ad ญ */ _tcons | _th_CCundersplit,
    /* ae ฎ */ _tcons | _th_CCundershoot,
    /* af ฏ */ _tcons | _th_CCundershoot,
    /* b0 ฐ */ _tcons | _th_CCundersplit,
    /* b1 ฑ */ _tcons,
    /* b2 ฒ */ _tcons,
    /* b3 ณ */ _tcons,
    /* b4 ด */ _tcons,
    /* b5 ต */ _tcons,
    /* b6 ถ */ _tcons,
    /* b7 ท */ _tcons,
    /* b8 ธ */ _tcons,
    /* b9 น */ _tcons,
    /* ba บ */ _tcons,
    /* bb ป */ _tcons | _th_CCovershoot,
    /* bc ผ */ _tcons,
    /* bd ฝ */ _tcons | _th_CCovershoot,
    /* be พ */ _tcons,
    /* bf ฟ */ _tcons | _th_CCovershoot,
    /* c0 ภ */ _tcons,
    /* c1 ม */ _tcons,
    /* c2 ย */ _tcons,
    /* c3 ร */ _tcons,
    /* c4 ฤ */ _tflvowel,
    /* c5 ล */ _tcons,
    /* c6 ฦ */ _tflvowel,
    /* c7 ว */ _tcons,
    /* c8 ศ */ _tcons,
    /* c9 ษ */ _tcons,
    /* ca ส */ _tcons,
    /* cb ห */ _tcons,
    /* cc ฬ */ _tcons | _th_CCovershoot,
    /* cd อ */ _tcons,
    /* ce ฮ */ _tcons,
    /* cf ฯ */ _tpunct,
    /* d0 ะ */ _tflvowel,
    /* d1 ั */ _tupvowel,
    /* d2 า */ _tflvowel,
    /* d3 ำ */ _tflvowel,
    /* d4 ิ */ _tupvowel,
    /* d5 ี */ _tupvowel,
    /* d6 ึ */ _tupvowel,
    /* d7 ื */ _tupvowel,
    /* d8 ุ */ _tblvowel,
    /* d9 ู */ _tblvowel,
    /* da ฺ */ _tdiac,
    /* db */ _none,
    /* dc */ _none,
    /* dd */ _none,
    /* de */ _none,
    /* df ฿ */ _tpunct,
    /* e0 เ */ _tldvowel,
    /* e1 แ */ _tldvowel,
    /* e2 โ */ _tldvowel,
    /* e3 ใ */ _tldvowel,
    /* e4 ไ */ _tldvowel,
    /* e5 ๅ */ _tflvowel,
    /* e6 ๆ */ _tpunct,
    /* e7 ็ */ _tdiac,
    /* e8 ่ */ _ttone,
    /* e9 ้ */ _ttone,
    /* ea ๊ */ _ttone,
    /* eb ๋ */ _ttone,
    /* ec ์ */ _tdiac,
    /* ed ํ */ _tdiac,
    /* ee ๎ */ _tdiac,
    /* ef ๏ */ _tpunct,
    /* f0 ๐ */ _tdigit,
    /* f1 ๑ */ _tdigit,
    /* f2 ๒ */ _tdigit,
    /* f3 ๓ */ _tdigit,
    /* f4 ๔ */ _tdigit,
    /* f5 ๕ */ _tdigit,
    /* f6 ๖ */ _tdigit,
    /* f7 ๗ */ _tdigit,
    /* f8 ๘ */ _tdigit,
    /* f9 ๙ */ _tdigit,
    /* fa ๚ */ _tpunct,
    /* fb ๛ */ _tpunct,
    /* fc */ _none,
    /* fd */ _none,
    /* fe */ _none,
    /* ff */ _none,
];

const _th_chlevel_tbl: [i8; 256] = [
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

const fn isctype(c: thchar_t, type_: u16) -> bool {
    _th_ctype_tbl[c as usize] & type_ != 0
}

const fn isbits(c: thchar_t, mask: u16, val: u16) -> bool {
    _th_ctype_tbl[c as usize] & mask == val
}

pub const fn th_istis(c: thchar_t) -> bool {
    isctype(c, _th_IStis)
}

pub const fn th_isthai(c: thchar_t) -> bool {
    th_istis(c) && (c & 0x80) != 0
}

pub const fn th_iseng(c: thchar_t) -> bool {
    c & 0x80 == 0
}

// Thai letter classification

pub const fn th_isthcons(c: thchar_t) -> bool {
    isctype(c, _th_IScons)
}

pub const fn th_isthvowel(c: thchar_t) -> bool {
    isctype(c, _th_ISvowel)
}

pub const fn th_isthtone(c: thchar_t) -> bool {
    isctype(c, _th_IStone)
}

pub const fn th_isthdiac(c: thchar_t) -> bool {
    isctype(c, _th_ISdiac)
}

pub const fn th_isthdigit(c: thchar_t) -> bool {
    isctype(c, _th_ISdigit)
}

pub const fn th_isthpunct(c: thchar_t) -> bool {
    isctype(c, _th_ISpunct)
}

//Thai consonant shapes classification

pub const fn th_istaillesscons(c: thchar_t) -> bool {
    isbits(c, _th_CClassMsk, _th_CCtailless)
}

pub const fn th_isovershootcons(c: thchar_t) -> bool {
    isbits(c, _th_CClassMsk, _th_CCovershoot)
}

pub const fn th_isundershootcons(c: thchar_t) -> bool {
    isbits(c, _th_CClassMsk, _th_CCundershoot)
}

pub const fn th_isundersplitcons(c: thchar_t) -> bool {
    isbits(c, _th_CClassMsk, _th_CCundersplit)
}

// Thai vowel classification

pub const fn th_isldvowel(c: thchar_t) -> bool {
    isbits(c, _th_VClassMsk, _th_VCldvowel)
}

pub const fn th_isflvowel(c: thchar_t) -> bool {
    isbits(c, _th_VClassMsk, _th_VCflvowel)
}

pub const fn th_isupvowel(c: thchar_t) -> bool {
    isbits(c, _th_VClassMsk, _th_VCupvowel)
}
pub const fn th_isblvowel(c: thchar_t) -> bool {
    isbits(c, _th_VClassMsk, _th_VCblvowel)
}

pub const fn th_chlevel(c: thchar_t) -> i8 {
    _th_chlevel_tbl[c as usize]
}

pub const fn th_iscombchar(c: thchar_t) -> bool {
    th_chlevel(c) != 0
}
