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

pub const TIS_KO_KAI: u8 = 0xa1;
pub const TIS_KHO_KHAI: u8 = 0xa2;
pub const TIS_KHO_KHUAT: u8 = 0xa3;
pub const TIS_KHO_KHWAI: u8 = 0xa4;
pub const TIS_KHO_KHON: u8 = 0xa5;
pub const TIS_KHO_RAKHANG: u8 = 0xa6;
pub const TIS_NGO_NGU: u8 = 0xa7;
pub const TIS_CHO_CHAN: u8 = 0xa8;
pub const TIS_CHO_CHING: u8 = 0xa9;
pub const TIS_CHO_CHANG: u8 = 0xaa;
pub const TIS_SO_SO: u8 = 0xab;
pub const TIS_CHO_CHOE: u8 = 0xac;
pub const TIS_YO_YING: u8 = 0xad;
pub const TIS_DO_CHADA: u8 = 0xae;
pub const TIS_TO_PATAK: u8 = 0xaf;
pub const TIS_THO_THAN: u8 = 0xb0;
pub const TIS_THO_NANGMONTHO: u8 = 0xb1;
pub const TIS_THO_PHUTHAO: u8 = 0xb2;
pub const TIS_NO_NEN: u8 = 0xb3;
pub const TIS_DO_DEK: u8 = 0xb4;
pub const TIS_TO_TAO: u8 = 0xb5;
pub const TIS_THO_THUNG: u8 = 0xb6;
pub const TIS_THO_THAHAN: u8 = 0xb7;
pub const TIS_THO_THONG: u8 = 0xb8;
pub const TIS_NO_NU: u8 = 0xb9;
pub const TIS_BO_BAIMAI: u8 = 0xba;
pub const TIS_PO_PLA: u8 = 0xbb;
pub const TIS_PHO_PHUNG: u8 = 0xbc;
pub const TIS_FO_FA: u8 = 0xbd;
pub const TIS_PHO_PHAN: u8 = 0xbe;
pub const TIS_FO_FAN: u8 = 0xbf;
pub const TIS_PHO_SAMPHAO: u8 = 0xc0;
pub const TIS_MO_MA: u8 = 0xc1;
pub const TIS_YO_YAK: u8 = 0xc2;
pub const TIS_RO_RUA: u8 = 0xc3;
pub const TIS_RU: u8 = 0xc4;
pub const TIS_LO_LING: u8 = 0xc5;
pub const TIS_LU: u8 = 0xc6;
pub const TIS_WO_WAEN: u8 = 0xc7;
pub const TIS_SO_SALA: u8 = 0xc8;
pub const TIS_SO_RUSI: u8 = 0xc9;
pub const TIS_SO_SUA: u8 = 0xca;
pub const TIS_HO_HIP: u8 = 0xcb;
pub const TIS_LO_CHULA: u8 = 0xcc;
pub const TIS_O_ANG: u8 = 0xcd;
pub const TIS_HO_NOKHUK: u8 = 0xce;
pub const TIS_PAIYANNOI: u8 = 0xcf;
pub const TIS_SARA_A: u8 = 0xd0;
pub const TIS_MAI_HAN_AKAT: u8 = 0xd1;
pub const TIS_SARA_AA: u8 = 0xd2;
pub const TIS_SARA_AM: u8 = 0xd3;
pub const TIS_SARA_I: u8 = 0xd4;
pub const TIS_SARA_II: u8 = 0xd5;
pub const TIS_SARA_UE: u8 = 0xd6;
pub const TIS_SARA_UEE: u8 = 0xd7;
pub const TIS_SARA_U: u8 = 0xd8;
pub const TIS_SARA_UU: u8 = 0xd9;
pub const TIS_PHINTHU: u8 = 0xda;
pub const TIS_SYMBOL_BAHT: u8 = 0xdf;
pub const TIS_SARA_E: u8 = 0xe0;
pub const TIS_SARA_AE: u8 = 0xe1;
pub const TIS_SARA_O: u8 = 0xe2;
pub const TIS_SARA_AI_MAIMUAN: u8 = 0xe3;
pub const TIS_SARA_AI_MAIMALAI: u8 = 0xe4;
pub const TIS_LAKKHANGYAO: u8 = 0xe5;
pub const TIS_MAIYAMOK: u8 = 0xe6;
pub const TIS_MAITAIKHU: u8 = 0xe7;
pub const TIS_MAI_EK: u8 = 0xe8;
pub const TIS_MAI_THO: u8 = 0xe9;
pub const TIS_MAI_TRI: u8 = 0xea;
pub const TIS_MAI_CHATTAWA: u8 = 0xeb;
pub const TIS_THANTHAKHAT: u8 = 0xec;
pub const TIS_NIKHAHIT: u8 = 0xed;
pub const TIS_YAMAKKAN: u8 = 0xee;
pub const TIS_FONGMAN: u8 = 0xef;
pub const TIS_THAI_DIGIT_ZERO: u8 = 0xf0;
pub const TIS_THAI_DIGIT_ONE: u8 = 0xf1;
pub const TIS_THAI_DIGIT_TWO: u8 = 0xf2;
pub const TIS_THAI_DIGIT_THREE: u8 = 0xf3;
pub const TIS_THAI_DIGIT_FOUR: u8 = 0xf4;
pub const TIS_THAI_DIGIT_FIVE: u8 = 0xf5;
pub const TIS_THAI_DIGIT_SIX: u8 = 0xf6;
pub const TIS_THAI_DIGIT_SEVEN: u8 = 0xf7;
pub const TIS_THAI_DIGIT_EIGHT: u8 = 0xf8;
pub const TIS_THAI_DIGIT_NINE: u8 = 0xf9;
pub const TIS_ANGKHANKHU: u8 = 0xfa;
pub const TIS_KHOMUT: u8 = 0xfb;

// For backward compatibility with libthai 0.1.28 and below
pub const TIS_YMBOL_BAHT: u8 = 0xdf;
